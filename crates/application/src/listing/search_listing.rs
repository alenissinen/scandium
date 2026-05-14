use std::sync::Arc;

use domain::listing::search::{ListingSearchParams, ListingSearchPort, ListingSearchResult};

pub struct SearchListingsRequest {
    pub query: Option<String>,
    pub listing_type: Option<String>,
    pub condition: Option<Vec<String>>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Clone)]
pub struct SearchListingsUseCase {
    listing_search: Arc<dyn ListingSearchPort>,
}

impl SearchListingsUseCase {
    pub fn new(listing_search: Arc<dyn ListingSearchPort>) -> Self {
        Self { listing_search }
    }

    pub async fn execute(&self, req: SearchListingsRequest) -> Result<ListingSearchResult, String> {
        let per_page = req.per_page.min(50);

        self.listing_search
            .search(ListingSearchParams {
                query: req.query,
                listing_type: req.listing_type,
                condition: req.condition,
                min_price: req.min_price,
                max_price: req.max_price,
                page: req.page.max(1),
                per_page,
            })
            .await
    }
}
