use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::auth::{entity::PasswordResetToken, error::AuthError};

#[async_trait]
pub trait PasswordResetRepository: Send + Sync {
    async fn create_reset_token(
        &self,
        user_id: Uuid,
        token_hash: String,
        expires_at: DateTime<Utc>,
    ) -> Result<PasswordResetToken, AuthError>;
    async fn find_by_token_hash(&self, token_hash: &str) -> Result<PasswordResetToken, AuthError>;
    async fn mark_token_used(&self, id: Uuid) -> Result<Uuid, AuthError>;
}
