pub mod domain;
pub mod application;
pub mod infrastructure;

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

use crate::infrastructure::repositories::{
    PostgresUserRepository, PostgresProfileRepository, PostgresBadgeRepository,
};
use crate::application::services::{
    AuthApplicationService, ProfileApplicationService, BadgeApplicationService,
};

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    // Create repositories
    let user_repository = Box::new(PostgresUserRepository::new(pool.clone()));
    let profile_repository = Box::new(PostgresProfileRepository::new(pool.clone()));
    let badge_repository = Box::new(PostgresBadgeRepository::new(pool.clone()));
    
    // Create application services
    let auth_service = AuthApplicationService::new(user_repository.clone());
    let profile_service = ProfileApplicationService::new(user_repository.clone(), profile_repository);
    let badge_service = BadgeApplicationService::new(user_repository, badge_repository);
    
    Router::new()
        .route("/health", get(health_check))
        .route("/siwe/nonce", get(handle_get_nonce))
        .route("/siwe/verify", post(handle_verify_message))
        .route("/profiles", get(handle_get_profiles))
        .route("/profiles", post(handle_create_profile))
        .route("/profiles/:address", get(handle_get_profile))
        .route("/profiles/:address", put(handle_update_profile))
        .route("/badges", get(handle_get_badges))
        .route("/badges", post(handle_create_badge))
        .route("/badges/:id", get(handle_get_badge))
        .route("/badges/:id/give", post(handle_give_badge))
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
        .with_state(AppState {
            auth_service,
            profile_service,
            badge_service,
        })
}

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthApplicationService,
    pub profile_service: ProfileApplicationService,
    pub badge_service: BadgeApplicationService,
}

async fn health_check() -> &'static str {
    "OK"
}

// Handler functions will be implemented in separate files
async fn handle_get_nonce() -> &'static str {
    "Not implemented yet"
}

async fn handle_verify_message() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_profiles() -> &'static str {
    "Not implemented yet"
}

async fn handle_create_profile() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_profile() -> &'static str {
    "Not implemented yet"
}

async fn handle_update_profile() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_badges() -> &'static str {
    "Not implemented yet"
}

async fn handle_create_badge() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_badge() -> &'static str {
    "Not implemented yet"
}

async fn handle_give_badge() -> &'static str {
    "Not implemented yet"
}