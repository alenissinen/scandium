use std::time::Duration;

use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum ListingEvent {
    Created {
        listing_id: String,
        user_id: String,
        title: String,
        price: i32,
        listing_type: String,
        condition: String,
        location: String,
        description: Option<String>,
    },
}

pub struct KafkaProducer {
    // FutureProducer is rdkafka's async producer
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create Kafka producer");

        Self { producer }
    }

    pub async fn send_listing_event(&self, event: ListingEvent) -> Result<(), String> {
        // Convert event to json
        let payload = serde_json::to_string(&event).map_err(|e| e.to_string())?;

        let topic = match &event {
            ListingEvent::Created { .. } => "listing.created",
        };

        let key = match &event {
            ListingEvent::Created { listing_id, .. } => listing_id.clone(),
        };

        self.producer
            .send(
                FutureRecord::to(topic).payload(&payload).key(&key),
                Duration::from_secs(5),
            )
            .await
            .map_err(|(e, _)| e.to_string())?;

        tracing::info!(topic = topic, key = key, "Kafka event sent");

        Ok(())
    }
}
