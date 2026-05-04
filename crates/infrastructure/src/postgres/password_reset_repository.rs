use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use domain::auth::{
    entity::PasswordResetToken, error::AuthError, repository::PasswordResetRepository,
};

pub struct PgPasswordResetRepository {
    pool: PgPool,
}

impl PgPasswordResetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

struct PasswordResetTokenRow {
    id: Uuid,
    user_id: Uuid,
    token_hash: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    used_at: Option<DateTime<Utc>>,
}

impl From<PasswordResetTokenRow> for PasswordResetToken {
    fn from(row: PasswordResetTokenRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            token_hash: row.token_hash,
            created_at: row.created_at,
            expires_at: row.expires_at,
            used_at: row.used_at,
        }
    }
}

#[async_trait]
impl PasswordResetRepository for PgPasswordResetRepository {
    async fn create_reset_token(
        &self,
        user_id: Uuid,
        token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> Result<PasswordResetToken, AuthError> {
        let row = sqlx::query_as!(
            PasswordResetTokenRow,
            r#"
            INSERT INTO password_reset_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, expires_at, used_at, created_at
            "#,
            user_id,
            token_hash,
            expires_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AuthError::Infrastructure(e.to_string()))?;

        Ok(row.into())
    }

    async fn find_by_token_hash(&self, token_hash: &str) -> Result<PasswordResetToken, AuthError> {
        let row = sqlx::query_as!(
            PasswordResetTokenRow,
            r#"
            SELECT id, user_id, token_hash, expires_at, used_at, created_at
            FROM password_reset_tokens
            WHERE token_hash = $1
            "#,
            token_hash,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AuthError::Infrastructure(e.to_string()))?
        .ok_or(AuthError::TokenNotFound)?;

        Ok(row.into())
    }

    async fn mark_token_used(&self, id: Uuid) -> Result<Uuid, AuthError> {
        sqlx::query!(
            r#"
            UPDATE password_reset_tokens
            SET used_at = now()
            WHERE id = $1
            "#,
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::Infrastructure(e.to_string()))?;

        Ok(id)
    }
}
