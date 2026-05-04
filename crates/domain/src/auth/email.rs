use async_trait::async_trait;

use crate::auth::error::AuthError;

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_password_reset(&self, recipient: &str, reset_link: &str)
    -> Result<(), AuthError>;
}
