use domain::listing::document::ListingDocument;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum IncomingEvent {
    Created(ListingDocument),
}
