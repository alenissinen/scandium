use std::sync::Arc;

use domain::listing::{
    entity::{Listing, ListingCondition, ListingType},
    error::ListingError,
    repository::{CreateListingInput, ListingRepository},
};
use uuid::Uuid;

pub struct CreateListingRequest {
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub listing_type: ListingType,
    pub condition: ListingCondition,
    pub location: String,
}

#[derive(Clone)]
pub struct CreateListingUseCase {
    listing_repo: Arc<dyn ListingRepository>,
}

impl CreateListingUseCase {
    pub fn new(listing_repo: Arc<dyn ListingRepository>) -> Self {
        Self { listing_repo }
    }

    pub async fn execute(&self, req: CreateListingRequest) -> Result<Listing, ListingError> {
        if req.title.trim().len() < 3 {
            return Err(ListingError::TitleTooShort);
        }

        if req.price <= 0 {
            return Err(ListingError::InvalidPrice);
        }

        let listing = self
            .listing_repo
            .create(CreateListingInput {
                user_id: req.user_id,
                title: req.title,
                description: req.description,
                price: req.price,
                listing_type: req.listing_type,
                condition: req.condition,
                location: req.location,
            })
            .await?;

        Ok(listing)
    }
}
