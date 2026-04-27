use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found: {0}")]
    NotFound(String),

    #[error("Email already registered to an account: {0}")]
    EmailTaken(String),

    #[error("Username already registered to an account: {0}")]
    UsernameTaken(String),

    #[error("Invalid email format")]
    InvalidEmail,

    #[error("Username contains invalid characters")]
    InvalidUsername,

    #[error("Username has to be at least 3 characters long")]
    UsernameTooShort,

    #[error("Password has to be at least 8 characters long")]
    PasswordTooShort,

    #[error("Account is deactivated")]
    AccountDeactivated,

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Invalid credentials")]
    InvalidCredentials,
}
