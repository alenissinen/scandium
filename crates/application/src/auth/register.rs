use std::sync::Arc;

use domain::user::{
    entity::User,
    error::UserError,
    repository::{CreateUserInput, UserRepository},
};
use shared::password::hash_password;

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

        let password_hash = hash_password(&req.password).map_err(UserError::Infrastructure)?;

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
        email_exists: bool,
        username_exists: bool,
    }

    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn create(&self, input: CreateUserInput) -> Result<User, UserError> {
            Ok(User {
                id: Uuid::new_v4(),
                email: input.email,
                username: input.username,
                password_hash: input.password_hash,
                display_name: input.display_name,
                avatar_url: None,
                location: None,
                phone: None,
                is_verified: false,
                is_active: true,
                role: UserRole::User,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }

        async fn update_password(&self, _: Uuid, _: String) -> Result<(), UserError> {
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<User, UserError> {
            Err(UserError::NotFound(id.into()))
        }

        async fn find_by_email(&self, email: &str) -> Result<User, UserError> {
            if self.email_exists {
                Err(UserError::EmailTaken(email.to_string()))
            } else {
                Err(UserError::NotFound(Uuid::nil().into()))
            }
        }

        async fn find_by_username(&self, username: &str) -> Result<User, UserError> {
            if self.username_exists {
                Err(UserError::UsernameTaken(username.to_string()))
            } else {
                Err(UserError::NotFound(Uuid::nil().into()))
            }
        }

        async fn find_by_username_or_email(&self, _: &str) -> Result<User, UserError> {
            Err(UserError::NotFound(Uuid::nil().into()))
        }
    }

    fn create_use_case(email_exists: bool, username_exists: bool) -> RegisterUseCase {
        RegisterUseCase::new(Arc::new(MockUserRepo {
            email_exists,
            username_exists,
        }))
    }

    fn valid_request() -> RegisterRequest {
        RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        }
    }

    #[tokio::test]
    async fn register_invalid_email_returns_error() {
        let uc = create_use_case(false, false);
        let mut req = valid_request();
        req.email = "notanemail".to_string();

        assert!(matches!(
            uc.execute(req).await,
            Err(UserError::InvalidEmail)
        ));
    }

    #[tokio::test]
    async fn register_invalid_username_chars_returns_error() {
        let uc = create_use_case(false, false);
        let mut req = valid_request();
        req.username = "invalid username!".to_string();

        assert!(matches!(
            uc.execute(req).await,
            Err(UserError::InvalidUsername)
        ));
    }

    #[tokio::test]
    async fn register_short_username_returns_error() {
        let uc = create_use_case(false, false);
        let mut req = valid_request();
        req.username = "ab".to_string();

        assert!(matches!(
            uc.execute(req).await,
            Err(UserError::UsernameTooShort)
        ));
    }

    #[tokio::test]
    async fn register_short_password_returns_error() {
        let uc = create_use_case(false, false);
        let mut req = valid_request();
        req.password = "short".to_string();

        assert!(matches!(
            uc.execute(req).await,
            Err(UserError::PasswordTooShort)
        ));
    }

    #[tokio::test]
    async fn register_success_calls_create() {
        let uc = create_use_case(false, false);
        let result = uc.execute(valid_request()).await;

        assert!(result.is_ok());
    }
}
