use serde::Serialize;

// Health check structure
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}
