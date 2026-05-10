use application::listing::create_listing::CreateListingRequest;
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use domain::listing::entity::{ListingCondition, ListingType};
use serde::Deserialize;

use crate::{errors::ApiError, middleware::auth::AuthenticatedUser, state::AppState};

#[derive(Deserialize)]
pub struct CreateListingBody {
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub listing_type: ListingType,
    pub condition: ListingCondition,
    pub location: String,
}

pub async fn create_listing(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Json(body): Json<CreateListingBody>,
) -> Result<impl IntoResponse, ApiError> {
    let listing = state
        .listings
        .create
        .execute(CreateListingRequest {
            user_id: auth_user.user_id,
            title: body.title,
            description: body.description,
            price: body.price,
            listing_type: body.listing_type,
            condition: body.condition,
            location: body.location,
        })
        .await
        .map_err(ApiError::Listing)?;

    Ok((StatusCode::CREATED, Json(listing)))
}
