use application::listing::{
    create_listing::CreateListingRequest, search_listing::SearchListingsRequest,
};
use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::listing::entity::{ListingCondition, ListingType};
use serde::Deserialize;

use crate::{errors::ApiError, middleware::auth::AuthenticatedUser, state::AppState};

#[derive(Deserialize)]
pub struct CreateListingBody {
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub year: Option<i32>,
    pub listing_type: ListingType,
    pub condition: ListingCondition,
    pub location: String,
}

#[derive(Deserialize)]
pub struct SearchListingsQuery {
    pub q: Option<String>,
    pub listing_type: Option<String>,
    pub condition: Option<String>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub page: Option<u32>,
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
            year: body.year,
            listing_type: body.listing_type,
            condition: body.condition,
            location: body.location,
        })
        .await
        .map_err(ApiError::Listing)?;

    Ok((StatusCode::CREATED, Json(listing)))
}

pub async fn search_listings(
    State(state): State<AppState>,
    Query(params): Query<SearchListingsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let condition = params
        .condition
        .map(|c| c.split('|').map(|s| s.to_string()).collect::<Vec<String>>());

    let result = state
        .listings
        .search
        .execute(SearchListingsRequest {
            query: params.q,
            listing_type: params.listing_type,
            condition,
            min_price: params.min_price,
            max_price: params.max_price,
            page: params.page.unwrap_or(1),
            per_page: 20,
        })
        .await
        .map_err(ApiError::Internal)?;

    Ok(Json(result))
}
