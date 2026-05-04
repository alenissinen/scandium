use chrono::{Duration, Utc};
use domain::{
    auth::{email::EmailService, error::AuthError, repository::PasswordResetRepository},
    user::repository::UserRepository,
};
use rand::RngExt;
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Clone)]
pub struct ForgotPasswordUseCase {
    user_repo: Arc<dyn UserRepository>,
    pw_reset_repo: Arc<dyn PasswordResetRepository>,
    email_repo: Arc<dyn EmailService>,
    app_url: String,
}

impl ForgotPasswordUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pw_reset_repo: Arc<dyn PasswordResetRepository>,
        email_repo: Arc<dyn EmailService>,
        app_url: String,
    ) -> Self {
        Self {
            user_repo,
            pw_reset_repo,
            email_repo,
            app_url,
        }
    }

    pub async fn execute(&self, req: ForgotPasswordRequest) -> Result<(), AuthError> {
        let user = match self.user_repo.find_by_email(&req.email).await {
            Ok(user) => user,
            Err(_) => return Ok(()),
        };

        // Generate 32 byte token and encode it
        let raw_token: [u8; 32] = rand::rng().random();
        let encoded_token = hex::encode(raw_token);

        // Hash raw token
        let token_hash = hex::encode(Sha256::digest(raw_token));

        // Token expires in 1h
        let expires_at = Utc::now() + Duration::hours(1);

        self.pw_reset_repo
            .create_reset_token(user.id, token_hash, expires_at)
            .await?;

        let reset_link = format!(
            "{}/auth/reset-password?token={}",
            self.app_url, encoded_token
        );

        self.email_repo
            .send_password_reset(&user.email, &reset_link)
            .await?;

        Ok(())
    }
}
