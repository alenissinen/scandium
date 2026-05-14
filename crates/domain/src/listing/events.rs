use async_trait::async_trait;

use crate::listing::entity::Listing;

#[async_trait]
pub trait ListingEventPublisher: Send + Sync {
    async fn publish_listing_created(&self, listing: &Listing) -> Result<(), String>;
}
