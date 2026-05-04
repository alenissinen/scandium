use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use serde_json::json;
use sqlx::types::Uuid;

use crate::state::AppState;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match jar.get("access_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Missing access token"})),
            )
                .into_response();
        }
    };

    let claims = match state.auth.jwt.verify(&token) {
        Ok(claims) => claims,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid or expired token"})),
            )
                .into_response();
        }
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid token subject" })),
            )
                .into_response();
        }
    };

    request
        .extensions_mut()
        .insert(AuthenticatedUser { user_id });

    next.run(request).await
}
