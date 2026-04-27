use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use domain::user::{entity::User, error::UserError, service::UserRepository};

pub struct LoginRequest {
    pub login_handle: String,
    pub password: String,
}

#[derive(Clone)]
pub struct LoginUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl LoginUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, req: LoginRequest) -> Result<User, UserError> {
        let user = self.user_repo.find_by_username_or_email(&req.login_handle).await?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| UserError::Infrastructure(e.to_string()))?;

        if Argon2::default().verify_password(req.password.as_bytes(), &parsed_hash).is_err() {
            return Err(UserError::InvalidCredentials);
        }

        if !user.is_active {
            return Err(UserError::AccountDeactivated);
        }

        Ok(user)
    }
}