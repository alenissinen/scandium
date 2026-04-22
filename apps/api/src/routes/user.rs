use application::user::create_user::CreateUserRequest;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use domain::user::entity::PublicUser;
use serde::Deserialize;
use sqlx::types::Uuid;

use crate::{errors::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<(StatusCode, Json<PublicUser>), ApiError> {
    let user = state
        .create_user
        .execute(CreateUserRequest {
            email: body.email,
            username: body.username,
            display_name: body.display_name,
        })
        .await?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

// Doesn't need status code since axum Json<T> defaults to 200
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PublicUser>, ApiError> {
    let user = state.get_user.execute(id).await?;

    Ok(Json(user.into()))
}
