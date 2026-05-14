use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::listing::{
    entity::{Listing, ListingCondition, ListingImage, ListingType},
    error::ListingError,
    repository::{CreateListingInput, ListingRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgListingRepository {
    pool: PgPool,
}

impl PgListingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

struct ListingRow {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    price: i32,
    pub year: Option<i32>,
    listing_type: String,
    condition: String,
    location: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ListingRow {
    fn into_listing(self, images: Vec<ListingImage>) -> Listing {
        Listing {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            description: self.description,
            price: self.price,
            year: self.year,
            listing_type: parse_listing_type(&self.listing_type),
            condition: parse_condition(&self.condition),
            location: self.location,
            is_active: self.is_active,
            images,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

struct ListingImageRow {
    id: Uuid,
    listing_id: Uuid,
    url: String,
    position: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ListingImageRow> for ListingImage {
    fn from(row: ListingImageRow) -> Self {
        Self {
            id: row.id,
            listing_id: row.listing_id,
            url: row.url,
            position: row.position,
            created_at: row.created_at,
        }
    }
}

fn parse_listing_type(s: &str) -> ListingType {
    match s {
        "snowboard" => ListingType::Snowboard,
        "boots" => ListingType::Boots,
        "bindings" => ListingType::Bindings,
        "clothing" => ListingType::Clothing,
        "protection" => ListingType::Protection,
        _ => ListingType::Skis,
    }
}

fn parse_condition(s: &str) -> ListingCondition {
    match s {
        "excellent" => ListingCondition::Excellent,
        "good" => ListingCondition::Good,
        "used" => ListingCondition::Used,
        _ => ListingCondition::New,
    }
}

fn listing_type_to_str(t: &ListingType) -> &'static str {
    match t {
        ListingType::Skis => "skis",
        ListingType::Snowboard => "snowboard",
        ListingType::Boots => "boots",
        ListingType::Bindings => "bindings",
        ListingType::Clothing => "clothing",
        ListingType::Protection => "protection",
    }
}

fn condition_to_str(c: &ListingCondition) -> &'static str {
    match c {
        ListingCondition::New => "new",
        ListingCondition::Excellent => "excellent",
        ListingCondition::Good => "good",
        ListingCondition::Used => "used",
    }
}

#[async_trait]
impl ListingRepository for PgListingRepository {
    async fn create(&self, input: CreateListingInput) -> Result<Listing, ListingError> {
        let row = sqlx::query_as!(
            ListingRow,
            r#"
            INSERT INTO listings (user_id, title, description, price, listing_type, condition, location)
            VALUES ($1, $2, $3, $4, $5::listing_type, $6::listing_condition, $7)
            RETURNING
                id, user_id, title, description, price, year,
                listing_type as "listing_type!: String",
                condition as "condition!: String",
                location, is_active, created_at, updated_at
            "#,
            input.user_id,
            input.title,
            input.description,
            input.price,
            listing_type_to_str(&input.listing_type) as &str,
            condition_to_str(&input.condition) as &str,
            input.location,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ListingError::Infrastructure(e.to_string()))?;

        Ok(row.into_listing(vec![]))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Listing, ListingError> {
        let row = sqlx::query_as!(
            ListingRow,
            r#"
            SELECT
                id, user_id, title, description, price, year,
                listing_type as "listing_type!: String",
                condition as "condition!: String",
                location, is_active, created_at, updated_at
            FROM listings
            WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ListingError::Infrastructure(e.to_string()))?
        .ok_or(ListingError::NotFound(id))?;

        let images = sqlx::query_as!(
            ListingImageRow,
            r#"
            SELECT id, listing_id, url, position, created_at
            FROM listing_images
            WHERE listing_id = $1
            ORDER BY position ASC
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ListingError::Infrastructure(e.to_string()))?;

        Ok(row.into_listing(images.into_iter().map(Into::into).collect()))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Listing>, ListingError> {
        let rows = sqlx::query_as!(
            ListingRow,
            r#"
            SELECT
                id, user_id, title, description, price, year,
                listing_type as "listing_type!: String",
                condition as "condition!: String",
                location, is_active, created_at, updated_at
            FROM listings
            WHERE user_id = $1 AND is_active = true
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ListingError::Infrastructure(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| row.into_listing(vec![]))
            .collect())
    }
}
