pub mod application;
pub mod domain;
pub mod infrastructure;

use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{get, post, put},
    Router,
};
use domain::repositories::profile_repository;
use infrastructure::{
    repositories::PostgresProfileRepository,
    services::ethereum_address_verification_service::EthereumAddressVerificationService,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::application::services::ProfileApplicationService;

pub async fn create_app(_pool: sqlx::PgPool) -> Router {
    // Create application services
    let auth_service = EthereumAddressVerificationService::new();
    let profile_repository = PostgresProfileRepository::new();

    Router::new()
        .route("/profiles/:address", post(handle_create_profile))
        .route("/profiles/:address", get(handle_get_profile))
        .route("/profiles/:address", put(handle_update_profile))
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
}

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthApplicationService,
    pub profile_service: ProfileApplicationService,
    pub badge_service: BadgeApplicationService,
}
