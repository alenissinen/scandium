use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use infrastructure::jwt::JwtService;
use serde::{Deserialize, Serialize};

use application::auth::{login::LoginRequest, register::RegisterRequest};
use domain::user::entity::PublicUser;
use sqlx::types::Uuid;

use crate::{errors::ApiError, state::AppState};

#[derive(Deserialize)]
pub struct RegisterBody {
    pub email: String,
    pub username: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub login_handle: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: PublicUser,
}

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterBody>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state
        .auth
        .register
        .execute(RegisterRequest {
            email: body.email,
            username: body.username,
            password: body.password,
            display_name: body.display_name,
        })
        .await?;

    let jar = create_auth_tokens(&state.auth.jwt, user.id)?;

    Ok((
        StatusCode::CREATED,
        jar,
        Json(AuthResponse { user: user.into() }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state
        .auth
        .login
        .execute(LoginRequest {
            login_handle: body.login_handle,
            password: body.password,
        })
        .await?;

    let jar = create_auth_tokens(&state.auth.jwt, user.id)?;

    Ok((
        StatusCode::OK,
        jar,
        Json(AuthResponse { user: user.into() }),
    ))
}

// Creates JWT tokens and returns jar with both tokens as cookies
fn create_auth_tokens(jwt: &JwtService, user_id: Uuid) -> Result<CookieJar, ApiError> {
    let access_token = jwt
        .create_access_token(user_id)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let refresh_token = jwt
        .create_refresh_token(user_id)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // TODO: add some type of environment variable to detect build type (dev/prod) instead of debug_assertations
    #[cfg(debug_assertions)]
    let secure = false;

    #[cfg(not(debug_assertions))]
    let secure = true;

    let jar = CookieJar::new()
        .add(
            Cookie::build(("access_token", access_token))
                .http_only(true)
                .same_site(SameSite::Lax)
                .path("/")
                .secure(secure)
                .build(),
        )
        .add(
            Cookie::build(("refresh_token", refresh_token))
                .http_only(true)
                .same_site(SameSite::Lax)
                .path("/api/v1/auth/refresh")
                .secure(secure)
                .build(),
        );

    Ok(jar)
}
