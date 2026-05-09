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
                "Password has to be at least 8 characters long".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    use domain::{
        auth::{entity::PasswordResetToken, error::AuthError, repository::PasswordResetRepository},
        user::{
            entity::User,
            error::UserError,
            repository::{CreateUserInput, UserRepository},
        },
    };
    use uuid::Uuid;

    struct MockUserRepo;

    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn create(&self, _: CreateUserInput) -> Result<User, UserError> {
            unimplemented!()
        }
        async fn update_password(&self, _: Uuid, _: String) -> Result<(), UserError> {
            Ok(())
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
            unimplemented!()
        }
    }

    struct MockPwResetRepo {
        token: Result<PasswordResetToken, AuthError>,
    }

    #[async_trait]
    impl PasswordResetRepository for MockPwResetRepo {
        async fn create_reset_token(
            &self,
            _: Uuid,
            _: String,
            _: chrono::DateTime<Utc>,
        ) -> Result<PasswordResetToken, AuthError> {
            unimplemented!()
        }
        async fn find_by_token_hash(&self, _: &str) -> Result<PasswordResetToken, AuthError> {
            self.token.clone()
        }
        async fn mark_token_used(&self, id: Uuid) -> Result<Uuid, AuthError> {
            Ok(id)
        }
    }

    fn create_token(
        expires_at: chrono::DateTime<Utc>,
        used_at: Option<chrono::DateTime<Utc>>,
    ) -> PasswordResetToken {
        PasswordResetToken {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            token_hash: "hash".to_string(),
            expires_at,
            used_at,
            created_at: Utc::now(),
        }
    }

    fn create_use_case(token: Result<PasswordResetToken, AuthError>) -> ResetPasswordUseCase {
        ResetPasswordUseCase::new(Arc::new(MockUserRepo), Arc::new(MockPwResetRepo { token }))
    }

    #[tokio::test]
    async fn reset_short_password_fails() {
        let token = create_token(Utc::now() + Duration::hours(1), None);
        let uc = create_use_case(Ok(token));
        let result = uc
            .execute(ResetPasswordRequest {
                token: "anytoken".to_string(),
                password: "short".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::Infrastructure(_))));
    }

    #[tokio::test]
    async fn reset_used_token_fails() {
        let token = create_token(Utc::now() + Duration::hours(1), Some(Utc::now()));
        let uc = create_use_case(Ok(token));
        let result = uc
            .execute(ResetPasswordRequest {
                token: "anytoken".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::TokenAlreadyUsed)));
    }

    #[tokio::test]
    async fn reset_expired_token_fails() {
        let token = create_token(Utc::now() - Duration::hours(1), None);
        let uc = create_use_case(Ok(token));
        let result = uc
            .execute(ResetPasswordRequest {
                token: "anytoken".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::TokenExpired)));
    }

    #[tokio::test]
    async fn reset_success() {
        let token = create_token(Utc::now() + Duration::hours(1), None);
        let uc = create_use_case(Ok(token));
        let result = uc
            .execute(ResetPasswordRequest {
                token: "anytoken".to_string(),
                password: "password123".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }
}
