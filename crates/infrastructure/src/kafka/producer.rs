use std::time::Duration;

use async_trait::async_trait;
use domain::listing::{entity::Listing, events::ListingEventPublisher};
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

#[async_trait]
impl ListingEventPublisher for KafkaProducer {
    async fn publish_listing_created(&self, listing: &Listing) -> Result<(), String> {
        // Convert Listing entity to Kafka event and send it to consumer
        self.send_listing_event(ListingEvent::Created {
            listing_id: listing.id.to_string(),
            user_id: listing.user_id.to_string(),
            title: listing.title.clone(),
            price: listing.price,
            listing_type: format!("{:?}", listing.listing_type).to_lowercase(),
            condition: format!("{:?}", listing.condition).to_lowercase(),
            location: listing.location.clone(),
            description: listing.description.clone(),
        })
        .await
    }
}
