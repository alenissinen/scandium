mod routes;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::routes::health::health_check;

#[tokio::main]
async fn main() {
    // Initialize tracing with default level set to info and allow debug level messages from tower
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    // Build axum application with routes
    let app: Router = Router::new()
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http());

    // Create listener at port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    // Create the HTTP server
    axum::serve(listener, app).await.unwrap();
}
