use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AuthError {
    #[error("Password reset token has expired")]
    TokenExpired,

    #[error("Password reset token has been already used")]
    TokenAlreadyUsed,

    #[error("Password reset token not found")]
    TokenNotFound,

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}
