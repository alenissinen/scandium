use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use domain::user::{entity::User, error::UserError, repository::UserRepository};

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
        let user = self
            .user_repo
            .find_by_username_or_email(&req.login_handle)
            .await?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| UserError::Infrastructure(e.to_string()))?;

        if Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(UserError::InvalidCredentials);
        }

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
    use shared::password::hash_password;
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
            Err(UserError::NotFound(id.into()))
        }

        async fn find_by_email(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }

        async fn find_by_username(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }

        async fn find_by_username_or_email(&self, _: &str) -> Result<User, UserError> {
            match &self.user {
                Some(u) => Ok(u.clone()),
                None => Err(UserError::NotFound(Uuid::nil().into())),
            }
        }
    }

    fn make_user(password: &str, is_active: bool) -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password_hash: hash_password(password).unwrap(),
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

    fn create_use_case(user: Option<User>) -> LoginUseCase {
        LoginUseCase::new(Arc::new(MockUserRepo { user }))
    }

    #[tokio::test]
    async fn login_user_not_found_fails() {
        let uc = create_use_case(None);
        let result = uc
            .execute(LoginRequest {
                login_handle: "nobody".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(UserError::NotFound(_))));
    }

    #[tokio::test]
    async fn login_wrong_password_fails() {
        let uc = create_use_case(Some(make_user("correctpassword", true)));
        let result = uc
            .execute(LoginRequest {
                login_handle: "testuser".to_string(),
                password: "wrongpassword".to_string(),
            })
            .await;

        assert!(matches!(result, Err(UserError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn login_deactivated_user_fails() {
        let uc = create_use_case(Some(make_user("password123", false)));
        let result = uc
            .execute(LoginRequest {
                login_handle: "testuser".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(UserError::AccountDeactivated)));
    }

    #[tokio::test]
    async fn login_success_returns_user() {
        let uc = create_use_case(Some(make_user("password123", true)));
        let result = uc
            .execute(LoginRequest {
                login_handle: "testuser".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }
}
