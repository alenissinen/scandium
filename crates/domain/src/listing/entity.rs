use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::listing::document::ListingDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListingType {
    Skis,
    Snowboard,
    Boots,
    Bindings,
    Clothing,
    Protection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ListingCondition {
    New,
    Excellent,
    Good,
    Used,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingImage {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub url: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub year: Option<i32>,
    pub listing_type: ListingType,
    pub condition: ListingCondition,
    pub location: String,
    pub is_active: bool,
    pub images: Vec<ListingImage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Listing> for ListingDocument {
    fn from(value: &Listing) -> Self {
        Self {
            id: value.id.to_string(),
            user_id: value.user_id.to_string(),
            title: value.title.clone(),
            description: value.description.clone(),
            price: value.price,
            year: value.year,
            listing_type: format!("{:?}", value.listing_type).to_lowercase(),
            condition: format!("{:?}", value.condition).to_lowercase(),
            location: value.location.clone(),
            created_at: value.created_at.to_rfc3339(),
        }
    }
}
