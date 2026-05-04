use std::sync::Arc;

use chrono::Utc;
use domain::{
    auth::{error::AuthError, repository::PasswordResetRepository},
    user::repository::UserRepository,
};
use sha2::{Digest, Sha256};
use shared::password::hash_password;

pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[derive(Clone)]
pub struct ResetPasswordUseCase {
    user_repo: Arc<dyn UserRepository>,
    pw_reset_repo: Arc<dyn PasswordResetRepository>,
}

impl ResetPasswordUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pw_reset_repo: Arc<dyn PasswordResetRepository>,
    ) -> Self {
        Self {
            user_repo,
            pw_reset_repo,
        }
    }

    pub async fn execute(&self, req: ResetPasswordRequest) -> Result<(), AuthError> {
        if req.password.len() < 8 {
            return Err(AuthError::Infrastructure(
                "Password too has to be atleast 8 characters long".to_string(),
            ));
        }

        let token_hash = hex::encode(Sha256::digest(req.token.as_bytes()));
        let token = self.pw_reset_repo.find_by_token_hash(&token_hash).await?;

        if token.used_at.is_some() {
            return Err(AuthError::TokenAlreadyUsed);
        }

        if token.expires_at < Utc::now() {
            return Err(AuthError::TokenExpired);
        }

        let password_hash = hash_password(&req.password).map_err(AuthError::Infrastructure)?;

        self.user_repo
            .update_password(token.user_id, password_hash)
            .await
            .map_err(|e| AuthError::Infrastructure(e.to_string()))?;

        self.pw_reset_repo.mark_token_used(token.id).await?;

        Ok(())
    }
}
