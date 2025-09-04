pub mod database;
pub mod handlers;
pub mod middleware;
pub mod models;

use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use handlers::{auth, profiles, badges};

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/siwe/nonce", get(auth::get_nonce))
        .route("/siwe/verify", post(auth::verify_message))
        .route("/profiles", get(profiles::get_profiles))
        .route("/profiles", post(profiles::create_profile))
        .route("/profiles/:address", get(profiles::get_profile))
        .route("/profiles/:address", put(profiles::update_profile))
        .route("/badges", get(badges::get_badges))
        .route("/badges", post(badges::create_badge))
        .route("/badges/:id", get(badges::get_badge))
        .route("/badges/:id/give", post(badges::give_badge))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024)), // 1MB limit
        )
        .with_state(pool)
}

async fn health_check() -> &'static str {
    "OK"
}
