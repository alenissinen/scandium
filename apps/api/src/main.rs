mod errors;
mod routes;
mod state;

use std::sync::Arc;

use application::{auth::register::RegisterUseCase, user::get_user::GetUserUseCase};
use axum::{
    Router, http::{HeaderValue, Method, header::{AUTHORIZATION, CONTENT_TYPE}}, routing::{get, post}
};
use infrastructure::{jwt::JwtService, postgres::user_repository::PgUserRepository};
use jsonwebtoken::crypto::aws_lc;
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::{CorsLayer}, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use routes::health::health_check;

use crate::state::{AppState, AuthContainer, UserContainer};

#[tokio::main]
async fn main() {
    // Set JWT crypto provider
    aws_lc::DEFAULT_PROVIDER.install_default().unwrap();

    // Fetch env values
    dotenvy::dotenv().ok();

    // Initialize tracing with default level set to info and allow debug level messages from tower
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL env variable must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let allowed_origin = std::env::var("ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());

    // Create postgres pool
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    // Dependency injection
    let user_repository = Arc::new(PgUserRepository::new(pool));

    let state = AppState {
        auth: Arc::new(AuthContainer {
            register: RegisterUseCase::new(user_repository.clone()),
            jwt: JwtService::new(&jwt_secret)
        }),
        users: Arc::new(UserContainer {
            get: GetUserUseCase::new(user_repository.clone())
        })
    };

    // Cors rules
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    // Create v1 api router
    let v1: Router<AppState> = Router::new()
        .route("/auth/register", post(routes::auth::register))
        .route("/users/{id}", get(routes::users::get_user));

    // Build axum application
    let app: Router = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", v1)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Create listener
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Listening on {}", addr);

    // Create the HTTP server
    axum::serve(listener, app).await.unwrap();
}
