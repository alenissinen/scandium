use std::sync::Arc;

use domain::user::{entity::User, error::UserError, repository::UserRepository};
use uuid::Uuid;

// Use dynamic dispatch for repository type to make mock testing easier
pub struct GetUserUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<User, UserError> {
        let user = self.user_repository.find_by_id(id).await?;

        if !user.is_active {
            return Err(UserError::AccountDeactivated);
        }

        Ok(user)
    }
}
