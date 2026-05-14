mod events;
mod processors;

use std::sync::Arc;

use infrastructure::elasticsearch::{create_client, listing_index::ListingIndex};
use rdkafka::{
    ClientConfig, Message,
    consumer::{CommitMode, Consumer, StreamConsumer},
};

use crate::events::IncomingEvent;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Env variables
    let kafka_brokers =
        std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let es_url =
        std::env::var("ELASTICSEARCH_URL").unwrap_or_else(|_| "http://localhost:9200".to_string());

    // Create ES client and indexes
    let es_client = create_client(&es_url);
    let listing_index = Arc::new(ListingIndex::new(es_client));

    // Make sure ES indexes exists
    listing_index
        .ensure_index_exists()
        .await
        .expect("Failed to ensure listing index exists");

    // Create async Kafka consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_brokers)
        .set("group.id", "scandium-consumer")
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create Kafka consumer");

    // Subscribe topics
    consumer
        .subscribe(&["listing.created"])
        .expect("Failed to subscribe to topics");

    tracing::info!("Kafka consumer started, listening for events.");

    loop {
        match consumer.recv().await {
            Err(e) => {
                tracing::error!("Kafka error: {}", e);
            }
            Ok(message) => {
                // Read message payload (bytes -> &str)
                let payload = match message.payload_view::<str>() {
                    Some(Ok(p)) => p,
                    Some(Err(e)) => {
                        tracing::error!(error = %e, "Failed to read message payload");
                        continue;
                    }
                    None => {
                        tracing::warn!("Empty message payload, skipping");
                        continue;
                    }
                };

                match process_message(payload, &listing_index).await {
                    Ok(_) => {
                        consumer
                            .commit_message(&message, CommitMode::Async)
                            .unwrap_or_else(|e| tracing::error!("Failed to commit offset: {}", e));
                    }
                    Err(e) => {
                        tracing::error!(error = %e, "Failed to process message");
                    }
                }
            }
        }
    }
}

async fn process_message(payload: &str, listing_index: &ListingIndex) -> Result<(), String> {
    let event: IncomingEvent =
        serde_json::from_str(payload).map_err(|e| format!("Failed to deserialize event: {}", e))?;

    match event {
        IncomingEvent::Created(doc) => {
            processors::listing::handle_created(doc, listing_index).await?;
        }
    }

    Ok(())
}
