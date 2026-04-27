use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use domain::user::{
    entity::User,
    error::UserError,
    service::{CreateUserInput, UserRepository},
};

pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Clone)]
pub struct RegisterUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl RegisterUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, req: RegisterRequest) -> Result<User, UserError> {
        if !domain::user::validation::is_valid_email(&req.email) {
            return Err(UserError::InvalidEmail);
        }

        if !req
            .username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(UserError::InvalidUsername);
        }

        if req.username.len() < 3 {
            return Err(UserError::UsernameTooShort);
        }

        if req.password.len() < 8 {
            return Err(UserError::PasswordTooShort);
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| UserError::Infrastructure(e.to_string()))?
            .to_string();

        let user = self
            .user_repo
            .create(CreateUserInput {
                email: req.email,
                username: req.username,
                password_hash,
                display_name: req.display_name,
            })
            .await?;

        tracing::info!(user_id = %user.id, username = %user.username, "New user created");

        Ok(user)
    }
}
