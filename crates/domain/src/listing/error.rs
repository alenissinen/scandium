use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ListingError {
    #[error("Listing not found: {0}")]
    NotFound(Uuid),

    #[error("Unauthorized to modify this listing")]
    Unauthorized,

    #[error("Title is too short (minimum 3 characters)")]
    TitleTooShort,

    #[error("Price must be greater than zero")]
    InvalidPrice,

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}
