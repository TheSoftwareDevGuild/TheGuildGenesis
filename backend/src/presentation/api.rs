use std::sync::Arc;

use crate::domain::repositories::ProfileRepository;
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::{
    repositories::PostgresProfileRepository,
    services::ethereum_address_verification_service::EthereumAddressVerificationService,
};
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

use super::handlers::{create_profile_handler, get_profile_handler, update_profile_handler};

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    // Create application services
    let auth_service = EthereumAddressVerificationService::new();
    let profile_repository = PostgresProfileRepository::new(pool);

    let state: AppState = AppState {
        profile_repository: Arc::from(profile_repository),
        auth_service: Arc::from(auth_service),
    };

    let protected = Router::new()
        .route("/profiles/:address", post(create_profile_handler))
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles/:address", put(update_profile_handler));

    Router::new().nest("/", protected).with_state(state).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(Any),
            )
            .layer(DefaultBodyLimit::max(1024 * 1024)),
    )
}

#[derive(Clone)]
pub struct AppState {
    pub profile_repository: Arc<dyn ProfileRepository>,
    pub auth_service: Arc<dyn AuthService>,
}
