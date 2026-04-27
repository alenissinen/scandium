use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::user::error::UserError;
use serde_json::json;

pub enum ApiError {
    User(UserError),
    Internal(String)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
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
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<UserError> for ApiError {
    fn from(e: UserError) -> Self {
        ApiError::User(e)
    }
}
