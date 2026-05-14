use serde::{Deserialize, Serialize};

// Elasticsearch document for indexing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingDocument {
    #[serde(alias = "listing_id")]
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub listing_type: String,
    pub condition: String,
    pub location: String,
    pub created_at: String,
}
