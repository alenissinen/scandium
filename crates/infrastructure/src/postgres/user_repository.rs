use async_trait::async_trait;
use domain::user::{
    entity::{User, UserRole},
    error::UserError,
    service::{CreateUserInput, UserRepository},
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// Map database row to user entity
struct UserRow {
    id: Uuid,
    email: String,
    username: String,
    display_name: String,
    password_hash: String,
    avatar_url: Option<String>,
    location: Option<String>,
    phone: Option<String>,
    is_verified: bool,
    is_active: bool,
    role: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            username: row.username,
            display_name: row.display_name,
            password_hash: row.password_hash,
            avatar_url: row.avatar_url,
            location: row.location,
            phone: row.phone,
            is_verified: row.is_verified,
            is_active: row.is_active,
            role: match row.role.as_str() {
                "admin" => UserRole::Admin,
                _ => UserRole::User,
            },
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, input: CreateUserInput) -> Result<User, UserError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            INSERT INTO users (email, username, password_hash, display_name)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id, email, username, display_name, password_hash,
                avatar_url, location, phone, is_verified, is_active,
                role::text as "role!", created_at, updated_at
            "#,
            input.email,
            input.username,
            input.password_hash,
            input.display_name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.constraint() == Some("users_email_key") => {
                UserError::EmailTaken(input.email)
            }
            sqlx::Error::Database(db_err) if db_err.constraint() == Some("users_username_key") => {
                UserError::UsernameTaken(input.username)
            }
            _ => UserError::Infrastructure(e.to_string()),
        })?;

        Ok(row.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<User, UserError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                id, email, username, display_name, password_hash, 
                avatar_url, location, phone, is_verified, is_active,
                role::text as "role!", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserError::Infrastructure(e.to_string()))?
        .ok_or(UserError::NotFound(id.to_string()))?;

        Ok(row.into())
    }

    async fn find_by_email(&self, email: &str) -> Result<User, UserError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                id, email, username, display_name, password_hash, 
                avatar_url, location, phone, is_verified, is_active,
                role::text as "role!", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserError::Infrastructure(e.to_string()))?
        .ok_or(UserError::NotFound(email.to_string()))?;

        Ok(row.into())
    }

    async fn find_by_username(&self, username: &str) -> Result<User, UserError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                id, email, username, display_name, password_hash, 
                avatar_url, location, phone, is_verified, is_active,
                role::text as "role!", created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserError::Infrastructure(e.to_string()))?
        .ok_or(UserError::NotFound(username.to_string()))?;

        Ok(row.into())
    }

    async fn find_by_username_or_email(&self, handle: &str) -> Result<User, UserError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT
                id, email, username, display_name, password_hash, 
                avatar_url, location, phone, is_verified, is_active,
                role::text as "role!", created_at, updated_at
            FROM users
            WHERE username = $1 OR email = $1
            "#,
            handle
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserError::Infrastructure(e.to_string()))?
        .ok_or(UserError::NotFound(handle.to_string()))?;

        Ok(row.into())
    }
}
