mod errors;
mod routes;
mod state;

use std::sync::Arc;

use application::user::{create_user::CreateUserUseCase, get_user::GetUserUseCase};
use axum::{
    Router,
    routing::{get, post},
};
use infrastructure::postgres::user_repository::PgUserRepository;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use routes::health::health_check;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing with default level set to info and allow debug level messages from tower
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL env variable must be set");

    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Dependency injection
    let user_repository = Arc::new(PgUserRepository::new(pool));

    let state = AppState {
        create_user: Arc::new(CreateUserUseCase::new(user_repository.clone())),
        get_user: Arc::new(GetUserUseCase::new(user_repository.clone())),
    };

    // Create v1 api router
    let v1: Router<AppState> = Router::new()
        .route("/user", post(routes::user::create_user))
        .route("/user/{id}", get(routes::user::get_user));

    // Build axum application with versioned api router
    let app: Router = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", v1)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Create listener at port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    // Create the HTTP server
    axum::serve(listener, app).await.unwrap();
}
