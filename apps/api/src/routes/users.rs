use axum::{
    Json,
    extract::{Path, State}
};
use domain::user::entity::PublicUser;
use sqlx::types::Uuid;

use crate::{errors::ApiError, state::AppState};

// Doesn't need status code since axum Json<T> defaults to 200
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PublicUser>, ApiError> {
    let user = state.users.get.execute(id).await?;

    Ok(Json(user.into()) )
}
