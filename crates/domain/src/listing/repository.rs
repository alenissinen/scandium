use async_trait::async_trait;
use uuid::Uuid;

use crate::listing::{
    entity::{Listing, ListingCondition, ListingType},
    error::ListingError,
};

pub struct CreateListingInput {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub year: Option<i32>,
    pub listing_type: ListingType,
    pub condition: ListingCondition,
    pub location: String,
}

#[async_trait]
pub trait ListingRepository: Send + Sync {
    async fn create(&self, input: CreateListingInput) -> Result<Listing, ListingError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Listing, ListingError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Listing>, ListingError>;
}
