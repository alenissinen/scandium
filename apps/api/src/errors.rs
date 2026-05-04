use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::{auth::error::AuthError, user::error::UserError};
use serde_json::json;

pub enum ApiError {
    User(UserError),
    Auth(AuthError),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // Internal error
            ApiError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            // UserError
            ApiError::User(e) => match e {
                UserError::NotFound(_) => (StatusCode::NOT_FOUND, e.to_string()),
                UserError::EmailTaken(_) | UserError::UsernameTaken(_) => {
                    (StatusCode::CONFLICT, e.to_string())
                }
                UserError::InvalidEmail
                | UserError::InvalidUsername
                | UserError::UsernameTooShort
                | UserError::PasswordTooShort => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
                UserError::AccountDeactivated => (StatusCode::FORBIDDEN, e.to_string()),
                UserError::InvalidCredentials => (StatusCode::UNAUTHORIZED, e.to_string()),
                UserError::Infrastructure(e) => {
                    tracing::error!("Infrastructure error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )
                }
            },
            // AuthError
            ApiError::Auth(e) => match e {
                AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, e.to_string()),
                AuthError::TokenAlreadyUsed => (StatusCode::UNAUTHORIZED, e.to_string()),
                AuthError::TokenNotFound => (StatusCode::NOT_FOUND, e.to_string()),
                AuthError::Infrastructure(e) => {
                    tracing::error!("Infrastructure error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )
                }
            },
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<UserError> for ApiError {
    fn from(e: UserError) -> Self {
        ApiError::User(e)
    }
}

impl From<AuthError> for ApiError {
    fn from(e: AuthError) -> Self {
        ApiError::Auth(e)
    }
}
