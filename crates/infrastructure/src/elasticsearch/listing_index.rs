use domain::listing::document::ListingDocument;
use elasticsearch::{
    Elasticsearch, IndexParts,
    indices::{IndicesCreateParts, IndicesExistsParts},
};

use serde_json::json;

pub struct ListingIndex {
    client: Elasticsearch,
    index_name: String,
}

impl ListingIndex {
    pub fn new(client: Elasticsearch) -> Self {
        Self {
            client,
            index_name: "listings".to_string(),
        }
    }

    pub async fn ensure_index_exists(&self) -> Result<(), String> {
        let exists = self
            .client
            .indices()
            .exists(IndicesExistsParts::Index(&[&self.index_name]))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if exists.status_code().is_success() {
            return Ok(());
        }

        self.client
            .indices()
            .create(IndicesCreateParts::Index(&self.index_name))
            .body(json!({
                "mappings": {
                    "properties": {
                        "listing_id":   { "type": "keyword" },
                        "user_id":      { "type": "keyword" },
                        "title":        { "type": "text", "fields": { "keyword": { "type": "keyword" } } },
                        "description":  { "type": "text" },
                        "price":        { "type": "integer" },
                        "listing_type": { "type": "keyword" },
                        "condition":    { "type": "keyword" },
                        "location":     { "type": "keyword" },
                        "created_at":   { "type": "date" }
                    }
                }
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        tracing::info!("Created ES index: {}", self.index_name);

        Ok(())
    }

    pub async fn index_listing(&self, listing: ListingDocument) -> Result<(), String> {
        let id = listing.id.clone();

        self.client
            .index(IndexParts::IndexId(&self.index_name, &id))
            .body(&listing)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        tracing::info!(listing_id = %id, "Indexed listing in ES");

        Ok(())
    }
}
