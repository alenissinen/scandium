use axum::{http::StatusCode, Json};
use shared::types::HealthResponse;

pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    let response = HealthResponse {
        status: "ok ".to_string(),
    };

    (StatusCode::OK, Json(response))
}
