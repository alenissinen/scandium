use std::sync::Arc;

use chrono::Utc;
use domain::auth::{error::AuthError, repository::PasswordResetRepository};
use sha2::{Digest, Sha256};

pub struct VerifyResetTokenRequest {
    pub token: String,
}

#[derive(Clone)]
pub struct VerifyResetTokenUseCase {
    pw_reset_repo: Arc<dyn PasswordResetRepository>,
}

impl VerifyResetTokenUseCase {
    pub fn new(pw_reset_repo: Arc<dyn PasswordResetRepository>) -> Self {
        Self { pw_reset_repo }
    }

    pub async fn execute(&self, req: VerifyResetTokenRequest) -> Result<(), AuthError> {
        let token_hash = hex::encode(Sha256::digest(req.token.as_bytes()));

        let token = self.pw_reset_repo.find_by_token_hash(&token_hash).await?;

        if token.used_at.is_some() {
            return Err(AuthError::TokenAlreadyUsed);
        }

        if token.expires_at < Utc::now() {
            return Err(AuthError::TokenExpired);
        }

        Ok(())
    }
}
