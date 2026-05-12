use std::sync::Arc;

use domain::listing::{
    entity::{Listing, ListingCondition, ListingType},
    error::ListingError,
    events::ListingEventPublisher,
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
    event_publisher: Arc<dyn ListingEventPublisher>,
}

impl CreateListingUseCase {
    pub fn new(
        listing_repo: Arc<dyn ListingRepository>,
        event_publisher: Arc<dyn ListingEventPublisher>,
    ) -> Self {
        Self {
            listing_repo,
            event_publisher,
        }
    }

    pub async fn execute(&self, req: CreateListingRequest) -> Result<Listing, ListingError> {
        if req.title.trim().len() < 3 {
            return Err(ListingError::TitleTooShort);
        }

        if req.price <= 0 {
            return Err(ListingError::InvalidPrice);
        }

        // Save listing to database
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

        // Send Kafka event, in case it fails, just log the error since the listing
        // has already been saved to db
        if let Err(e) = self.event_publisher.publish_listing_created(&listing).await {
            tracing::error!(listing_id = %listing.id, error = %e, "Failed to publish listing created event");
        }

        Ok(listing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use domain::listing::{
        entity::{Listing, ListingCondition, ListingType},
        error::ListingError,
        repository::{CreateListingInput, ListingRepository},
    };
    use uuid::Uuid;

    struct MockListingRepo {
        should_fail: bool,
    }

    struct MockEventPublisher;

    #[async_trait]
    impl ListingRepository for MockListingRepo {
        async fn create(&self, input: CreateListingInput) -> Result<Listing, ListingError> {
            if self.should_fail {
                return Err(ListingError::Infrastructure("DB error".to_string()));
            }
            Ok(Listing {
                id: Uuid::new_v4(),
                user_id: input.user_id,
                title: input.title,
                description: input.description,
                price: input.price,
                listing_type: input.listing_type,
                condition: input.condition,
                location: input.location,
                is_active: true,
                images: vec![],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Listing, ListingError> {
            Err(ListingError::NotFound(id))
        }

        async fn find_by_user_id(&self, _: Uuid) -> Result<Vec<Listing>, ListingError> {
            Ok(vec![])
        }
    }

    #[async_trait]
    impl ListingEventPublisher for MockEventPublisher {
        async fn publish_listing_created(&self, _: &Listing) -> Result<(), String> {
            Ok(())
        }
    }

    fn create_use_case(should_fail: bool) -> CreateListingUseCase {
        CreateListingUseCase::new(
            Arc::new(MockListingRepo { should_fail }),
            Arc::new(MockEventPublisher),
        )
    }

    fn valid_request() -> CreateListingRequest {
        CreateListingRequest {
            user_id: Uuid::new_v4(),
            title: "Atomic Redster X9 170cm".to_string(),
            description: None,
            price: 485,
            listing_type: ListingType::Skis,
            condition: ListingCondition::New,
            location: "Helsinki".to_string(),
        }
    }

    #[tokio::test]
    async fn create_listing_success() {
        let uc = create_use_case(false);
        let result = uc.execute(valid_request()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_listing_title_too_short() {
        let uc = create_use_case(false);
        let mut req = valid_request();
        req.title = "ab".to_string();

        let result = uc.execute(req).await;

        assert!(matches!(result, Err(ListingError::TitleTooShort)));
    }

    #[tokio::test]
    async fn create_listing_invalid_price() {
        let uc = create_use_case(false);
        let mut req = valid_request();
        req.price = 0;

        let result = uc.execute(req).await;

        assert!(matches!(result, Err(ListingError::InvalidPrice)));
    }

    #[tokio::test]
    async fn create_listing_negative_price() {
        let uc = create_use_case(false);
        let mut req = valid_request();
        req.price = -100;

        let result = uc.execute(req).await;

        assert!(matches!(result, Err(ListingError::InvalidPrice)));
    }

    #[tokio::test]
    async fn create_listing_infrastructure_error_propagates() {
        let uc = create_use_case(true);
        let result = uc.execute(valid_request()).await;

        assert!(matches!(result, Err(ListingError::Infrastructure(_))));
    }

    #[tokio::test]
    async fn create_listing_sets_correct_user_id() {
        let uc = create_use_case(false);
        let req = valid_request();
        let user_id = req.user_id;
        let listing = uc.execute(req).await.unwrap();

        assert_eq!(listing.user_id, user_id);
    }
}
