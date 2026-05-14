use domain::listing::document::ListingDocument;
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::{Value, json};

pub struct ListingSearchParams {
    pub query: Option<String>,
    pub listing_type: Option<String>,
    pub condition: Option<Vec<String>>,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub page: u32,
    pub per_page: u32,
}

pub struct ListingSearchResult {
    pub listings: Vec<ListingDocument>,
    pub total: u64,
}

pub struct ListingSearch {
    client: Elasticsearch,
    index_name: String,
}

impl ListingSearch {
    pub fn new(client: Elasticsearch) -> Self {
        Self {
            client,
            index_name: "listings".to_string(),
        }
    }

    pub async fn search(&self, params: ListingSearchParams) -> Result<ListingSearchResult, String> {
        let mut must: Vec<Value> = vec![];

        if let Some(q) = &params.query {
            must.push(json!({
                "multi_match": {
                    "query": q,
                    "fields": ["title^2", "description"], // title^2 -> title is more important
                    "fuzziness": "AUTO" // accept typos
                }
            }));
        }

        let mut filters: Vec<Value> = vec![];

        if let Some(listing_type) = &params.listing_type {
            filters.push(json!({
                "term": { "listing_type": listing_type }
            }));
        }

        if let Some(conditions) = &params.condition
            && !conditions.is_empty()
        {
            filters.push(json!({
                "terms": { "condition": conditions }
            }));
        }

        // Price range using range query
        if params.min_price.is_some() || params.max_price.is_some() {
            let mut range = json!({});
            if let Some(min) = params.min_price {
                range["gte"] = json!(min);
            }
            if let Some(max) = params.max_price {
                range["lte"] = json!(max);
            }
            filters.push(json!({
                "range": { "price": range }
            }));
        }

        // Concat must and filters arrays with bool query
        let query = json!({
            "bool": {
                "must": must,
                "filter": filters
            }
        });

        // Pagination
        let from = (params.page.saturating_sub(1)) * params.per_page;

        let body = json!({
            "query": query,
            "from": from,
            "size": params.per_page,
            "sort": [
                { "_score": { "order": "desc" } },
                { "created_at": { "order": "desc" } }
            ]
        });

        let response = self
            .client
            .search(SearchParts::Index(&[&self.index_name]))
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_body = response.json::<Value>().await.map_err(|e| e.to_string())?;

        // Parse amount of total hits
        let total = response_body["hits"]["total"]["value"]
            .as_u64()
            .unwrap_or(0);

        // Parse listings
        let listings: Vec<ListingDocument> = response_body["hits"]["hits"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|hit| {
                serde_json::from_value::<ListingDocument>(hit["_source"].clone()).ok()
            })
            .collect();

        Ok(ListingSearchResult { listings, total })
    }
}
