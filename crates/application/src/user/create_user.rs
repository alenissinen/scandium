use std::sync::Arc;

use domain::user::{
    entity::User,
    error::UserError,
    service::{CreateUserInput, UserRepository},
    validation::is_valid_email,
};

pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
}

// Use dynamic dispatch for repository type to make mock testing easier
pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl CreateUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, req: CreateUserRequest) -> Result<User, UserError> {
        if !is_valid_email(&req.email) {
            return Err(UserError::InvalidEmail);
        }

        // Validate username, accept only a-Z, 0-9, _
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

        // Check for duplicate accounts
        if self
            .user_repository
            .exists_by_username(&req.username)
            .await?
        {
            return Err(UserError::UsernameTaken(req.username));
        }

        if self.user_repository.exists_by_email(&req.email).await? {
            return Err(UserError::EmailTaken(req.email));
        }

        // Create user
        let user = self
            .user_repository
            .create(CreateUserInput {
                email: req.email,
                username: req.username,
                password_hash: None, // TODO: change this after auth is ready
                display_name: req.display_name,
            })
            .await?;

        Ok(user)
    }
}
