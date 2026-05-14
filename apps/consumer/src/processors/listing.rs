use domain::listing::document::ListingDocument;
use infrastructure::elasticsearch::listing_index::ListingIndex;

pub async fn handle_created(
    doc: ListingDocument,
    listing_index: &ListingIndex,
) -> Result<(), String> {
    let listing_id = doc.id.clone();
    listing_index.index_listing(doc).await?;

    tracing::info!(listing_id = %listing_id, "Successfully indexed listing");

    Ok(())
}
