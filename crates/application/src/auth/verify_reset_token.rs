use std::sync::Arc;

use chrono::Utc;
use domain::auth::{error::AuthError, repository::PasswordResetRepository};
use sha2::{Digest, Sha256};

pub struct VerifyResetTokenRequest {
    pub token: String,
}

#[derive(Clone)]
pub struct VerifyResetTokenUseCase {
    pw_reset_repo: Arc<dyn PasswordResetRepository>,
}

impl VerifyResetTokenUseCase {
    pub fn new(pw_reset_repo: Arc<dyn PasswordResetRepository>) -> Self {
        Self { pw_reset_repo }
    }

    pub async fn execute(&self, req: VerifyResetTokenRequest) -> Result<(), AuthError> {
        let token_hash = hex::encode(Sha256::digest(req.token.as_bytes()));

        let token = self.pw_reset_repo.find_by_token_hash(&token_hash).await?;

        if token.used_at.is_some() {
            return Err(AuthError::TokenAlreadyUsed);
        }

        if token.expires_at < Utc::now() {
            return Err(AuthError::TokenExpired);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    use domain::auth::{
        entity::PasswordResetToken, error::AuthError, repository::PasswordResetRepository,
    };
    use uuid::Uuid;

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

    fn create_use_case(token: Result<PasswordResetToken, AuthError>) -> VerifyResetTokenUseCase {
        VerifyResetTokenUseCase::new(Arc::new(MockPwResetRepo { token }))
    }

    #[tokio::test]
    async fn verify_valid_token_ok() {
        let token = create_token(Utc::now() + Duration::hours(1), None);
        let uc = create_use_case(Ok(token));

        assert!(
            uc.execute(VerifyResetTokenRequest {
                token: "anytoken".to_string()
            })
            .await
            .is_ok()
        );
    }

    #[tokio::test]
    async fn verify_expired_token_fails() {
        let token = create_token(Utc::now() - Duration::hours(1), None);
        let uc = create_use_case(Ok(token));
        let result = uc
            .execute(VerifyResetTokenRequest {
                token: "anytoken".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::TokenExpired)));
    }

    #[tokio::test]
    async fn verify_used_token_fails() {
        let token = create_token(Utc::now() + Duration::hours(1), Some(Utc::now()));
        let uc = create_use_case(Ok(token));
        let result = uc
            .execute(VerifyResetTokenRequest {
                token: "anytoken".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::TokenAlreadyUsed)));
    }

    #[tokio::test]
    async fn verify_not_found_propagates_error() {
        let uc = create_use_case(Err(AuthError::TokenNotFound));
        let result = uc
            .execute(VerifyResetTokenRequest {
                token: "anytoken".to_string(),
            })
            .await;

        assert!(matches!(result, Err(AuthError::TokenNotFound)));
    }
}
