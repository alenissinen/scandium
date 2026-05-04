use async_trait::async_trait;
use domain::auth::{email::EmailService, error::AuthError};
use resend_rs::{Resend, types::CreateEmailBaseOptions};

pub struct ResendEmailService {
    api_key: String,
    from: String,
}

impl ResendEmailService {
    pub fn new(api_key: String, from: String) -> Self {
        Self { api_key, from }
    }
}

#[async_trait]
impl EmailService for ResendEmailService {
    async fn send_password_reset(
        &self,
        recipient: &str,
        reset_link: &str,
    ) -> Result<(), AuthError> {
        let resend = Resend::new(&self.api_key);

        let email = CreateEmailBaseOptions::new(&self.from, [recipient], "Reset your Scandium password")
            .with_html(&format!(
                "<p>Click the link below to reset your password. The link expires in 1 hour.</p><p><a href=\"{}\">Reset password</a></p>",
                reset_link
            ));

        resend
            .emails
            .send(email)
            .await
            .map_err(|e| AuthError::Infrastructure(e.to_string()))?;

        Ok(())
    }
}
