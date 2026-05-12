mod errors;
mod middleware;
mod routes;
mod state;

use std::sync::Arc;

use application::{
    auth::{
        forgot_password::ForgotPasswordUseCase, login::LoginUseCase, register::RegisterUseCase,
        reset_password::ResetPasswordUseCase, verify_reset_token::VerifyResetTokenUseCase,
    },
    listing::create_listing::CreateListingUseCase,
    user::get_user::GetUserUseCase,
};
use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{get, post},
};
use infrastructure::{
    email::resend::ResendEmailService,
    jwt::JwtService,
    kafka::producer::KafkaProducer,
    postgres::{
        listing_repository::PgListingRepository,
        password_reset_repository::PgPasswordResetRepository, user_repository::PgUserRepository,
    },
};
use jsonwebtoken::crypto::aws_lc;
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use routes::health::health_check;

use crate::{
    middleware::auth::jwt_auth_middleware,
    state::{AppState, AuthContainer, ListingContainer, UserContainer},
};

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

    // Env variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let allowed_origin =
        std::env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let app_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let app_email = std::env::var("APP_EMAIL").expect("APP_EMAIL must be set");
    let resend_api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
    let kafka_brokers =
        std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());

    // Create postgres pool
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    // Dependency injection
    let user_repository = Arc::new(PgUserRepository::new(pool.clone()));
    let pw_reset_repository = Arc::new(PgPasswordResetRepository::new(pool.clone()));
    let email_service = Arc::new(ResendEmailService::new(resend_api_key, app_email));
    let listing_repository = Arc::new(PgListingRepository::new(pool.clone()));
    let kafka_producer = Arc::new(KafkaProducer::new(&kafka_brokers));

    let state = AppState {
        auth: Arc::new(AuthContainer {
            register: RegisterUseCase::new(user_repository.clone()),
            login: LoginUseCase::new(user_repository.clone()),
            jwt: JwtService::new(&jwt_secret),
            forgot_password: ForgotPasswordUseCase::new(
                user_repository.clone(),
                pw_reset_repository.clone(),
                email_service.clone(),
                app_url,
            ),
            verify_reset_token: VerifyResetTokenUseCase::new(pw_reset_repository.clone()),
            reset_password: ResetPasswordUseCase::new(
                user_repository.clone(),
                pw_reset_repository.clone(),
            ),
        }),
        users: Arc::new(UserContainer {
            get: GetUserUseCase::new(user_repository.clone()),
        }),
        listings: Arc::new(ListingContainer {
            create: CreateListingUseCase::new(listing_repository.clone(), kafka_producer.clone()),
        }),
    };

    // Cors rules
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    // Create router with jwt auth middleware
    let protected: Router<AppState> = Router::new()
        .route("/auth/me", get(routes::auth::me))
        .route("/listings", post(routes::listing::create_listing))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            jwt_auth_middleware,
        ));

    // Create v1 api router
    let v1: Router<AppState> = Router::new()
        .route("/auth/register", post(routes::auth::register))
        .route("/auth/login", post(routes::auth::login))
        .route("/auth/forgot-password", post(routes::auth::forgot_password))
        .route(
            "/auth/verify-reset-token",
            post(routes::auth::verify_reset_token),
        )
        .route("/auth/reset-password", post(routes::auth::reset_password))
        .route("/users/{id}", get(routes::users::get_user))
        .merge(protected);

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
