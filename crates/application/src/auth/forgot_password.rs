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
    email_service: Arc<dyn EmailService>,
    app_url: String,
}

impl ForgotPasswordUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        pw_reset_repo: Arc<dyn PasswordResetRepository>,
        email_service: Arc<dyn EmailService>,
        app_url: String,
    ) -> Self {
        Self {
            user_repo,
            pw_reset_repo,
            email_service,
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
        let token_hash = hex::encode(Sha256::digest(encoded_token.as_bytes()));

        // Token expires in 1h
        let expires_at = Utc::now() + Duration::hours(1);

        self.pw_reset_repo
            .create_reset_token(user.id, token_hash, expires_at)
            .await?;

        let reset_link = format!(
            "{}/auth/reset-password?token={}",
            self.app_url, encoded_token
        );

        self.email_service
            .send_password_reset(&user.email, &reset_link)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use domain::{
        auth::{
            email::EmailService, entity::PasswordResetToken, error::AuthError,
            repository::PasswordResetRepository,
        },
        user::{
            entity::{User, UserRole},
            error::UserError,
            repository::{CreateUserInput, UserRepository},
        },
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
            Err(UserError::NotFound(id.into()))
        }

        async fn find_by_email(&self, _: &str) -> Result<User, UserError> {
            match &self.user {
                Some(u) => Ok(u.clone()),
                None => Err(UserError::NotFound(Uuid::nil().into())),
            }
        }

        async fn find_by_username(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }

        async fn find_by_username_or_email(&self, _: &str) -> Result<User, UserError> {
            unimplemented!()
        }
    }

    struct MockPwResetRepo;

    #[async_trait]
    impl PasswordResetRepository for MockPwResetRepo {
        async fn create_reset_token(
            &self,
            user_id: Uuid,
            token_hash: String,
            expires_at: chrono::DateTime<Utc>,
        ) -> Result<PasswordResetToken, AuthError> {
            Ok(PasswordResetToken {
                id: Uuid::new_v4(),
                user_id,
                token_hash,
                expires_at,
                used_at: None,
                created_at: Utc::now(),
            })
        }
        async fn find_by_token_hash(&self, _: &str) -> Result<PasswordResetToken, AuthError> {
            unimplemented!()
        }
        async fn mark_token_used(&self, id: Uuid) -> Result<Uuid, AuthError> {
            Ok(id)
        }
    }

    struct MockEmailService {
        should_fail: bool,
    }

    #[async_trait]
    impl EmailService for MockEmailService {
        async fn send_password_reset(&self, _: &str, _: &str) -> Result<(), AuthError> {
            if self.should_fail {
                Err(AuthError::Infrastructure("Email failed".to_string()))
            } else {
                Ok(())
            }
        }
    }

    fn create_user() -> User {
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
            is_active: true,
            role: UserRole::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_use_case(user: Option<User>, email_fails: bool) -> ForgotPasswordUseCase {
        ForgotPasswordUseCase::new(
            Arc::new(MockUserRepo { user }),
            Arc::new(MockPwResetRepo),
            Arc::new(MockEmailService {
                should_fail: email_fails,
            }),
            "http://localhost:3000".to_string(),
        )
    }

    #[tokio::test]
    async fn forgot_password_user_not_found_returns_ok() {
        let uc = create_use_case(None, false);
        let result = uc
            .execute(ForgotPasswordRequest {
                email: "nobody@example.com".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn forgot_password_success_sends_email() {
        let uc = create_use_case(Some(create_user()), false);
        let result = uc
            .execute(ForgotPasswordRequest {
                email: "test@example.com".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn forgot_password_email_failure_propagates_error() {
        let uc = create_use_case(Some(create_user()), true);
        let result = uc
            .execute(ForgotPasswordRequest {
                email: "test@example.com".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::Infrastructure(_))));
    }
}
