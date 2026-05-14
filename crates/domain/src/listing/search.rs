use super::document::ListingDocument;
use async_trait::async_trait;
use serde::Serialize;

pub struct ListingSearchParams {
    pub query: Option<String>,
    pub listing_type: Option<String>,
    pub condition: Option<Vec<String>>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Serialize)]
pub struct ListingSearchResult {
    pub listings: Vec<ListingDocument>,
    pub total: u64,
}

#[async_trait]
pub trait ListingSearchPort: Send + Sync {
    async fn search(&self, params: ListingSearchParams) -> Result<ListingSearchResult, String>;
}
