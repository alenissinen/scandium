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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use domain::user::{
        entity::{User, UserRole},
        error::UserError,
        repository::{CreateUserInput, UserRepository},
    };
    use uuid::Uuid;

    struct MockUserRepo {
        user: Option<User>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn create(&self, _: CreateUserInput) -> Result<User, UserError> {
            unimplemented!()
        }

        async fn update_password(&self, _: Uuid, _: String) -> Result<(), UserError> {
            unimplemented!()
        }

        async fn find_by_id(&self, id: Uuid) -> Result<User, UserError> {
            match &self.user {
                Some(u) => Ok(u.clone()),
                None => Err(UserError::NotFound(id.into())),
            }
        }

        async fn find_by_email(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }

        async fn find_by_username(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }

        async fn find_by_username_or_email(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }
    }

    fn create_user(is_active: bool) -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password_hash: "hash".to_string(),
            display_name: "Test User".to_string(),
            avatar_url: None,
            location: None,
            phone: None,
            is_verified: false,
            is_active,
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_use_case(user: Option<User>) -> GetUserUseCase {
        GetUserUseCase::new(Arc::new(MockUserRepo { user }))
    }

    #[tokio::test]
    async fn get_active_user_returns_user() {
        let user = create_user(true);
        let id = user.id;
        let uc = create_use_case(Some(user));
        let result = uc.execute(id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_inactive_user_returns_deactivated_error() {
        let user = create_user(false);
        let id = user.id;
        let uc = create_use_case(Some(user));
        let result = uc.execute(id).await;

        assert!(matches!(result, Err(UserError::AccountDeactivated)));
    }

    #[tokio::test]
    async fn get_nonexistent_user_propagates_not_found() {
        let uc = create_use_case(None);
        let result = uc.execute(Uuid::new_v4()).await;

        assert!(matches!(result, Err(UserError::NotFound(_))));
    }
}
