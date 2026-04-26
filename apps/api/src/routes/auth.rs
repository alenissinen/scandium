use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

use application::auth::register::RegisterRequest;
use domain::user::entity::PublicUser;

use crate::{errors::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct RegisterBody {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: PublicUser,
}

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterBody>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state.auth.register.execute(RegisterRequest {
        email: body.email,
        username: body.username,
        password: body.password,
        display_name: body.display_name,
    }).await?;

    let access_token = state.auth.jwt
        .create_access_token(user.id)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let refresh_token = state.auth.jwt
        .create_refresh_token(user.id)
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    
    // TODO: add some type of environment variable to detect dev/prod build instead of debug_assertations
    #[cfg(debug_assertions)]
    let secure = false;

    #[cfg(not(debug_assertions))]
    let secure = true;

    let access_cookie = Cookie::build(("access_token", access_token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .secure(secure)
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/api/v1/auth/refresh")
        .secure(secure)
        .build();


    let jar = CookieJar::new()
        .add(access_cookie)
        .add(refresh_cookie);

    Ok((StatusCode::CREATED, jar, Json(AuthResponse { user: user.into() })))
}