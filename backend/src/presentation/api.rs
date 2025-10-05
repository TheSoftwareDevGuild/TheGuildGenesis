use std::sync::Arc;

use crate::domain::repositories::ProfileRepository;
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::{
    repositories::PostgresProfileRepository,
    services::ethereum_address_verification_service::EthereumAddressVerificationService,
};
use axum::middleware::{from_fn, from_fn_with_state};
use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use super::handlers::{
    create_profile_handler, delete_profile_handler, get_all_profiles_handler, get_profile_handler,
    update_profile_handler,
};

use super::middlewares::{eth_auth_layer, test_auth_layer};

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    let auth_service = EthereumAddressVerificationService::new();
    let profile_repository = PostgresProfileRepository::new(pool);

    let state: AppState = AppState {
        profile_repository: Arc::from(profile_repository),
        auth_service: Arc::from(auth_service),
    };

    let protected = Router::new()
        .route("/profiles/", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .with_state(state.clone());

    let protected = if std::env::var("TEST_MODE").is_ok() {
        protected.layer(from_fn(test_auth_layer))
    } else {
        protected.layer(from_fn_with_state(state.clone(), eth_auth_layer))
    };

    let public = Router::new()
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles/", get(get_all_profiles_handler))
        .with_state(state.clone());

    Router::new()
        .nest("/", protected)
        .merge(public)
        .with_state(state.clone())
        .layer(
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

pub fn test_api(state: AppState) -> Router {
    let protected = Router::new()
        .route("/profiles/", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .with_state(state.clone())
        .layer(from_fn(test_auth_layer));

    let public = Router::new()
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles/", get(get_all_profiles_handler))
        .with_state(state.clone());

    Router::new()
        .nest("/", protected)
        .merge(public)
        .with_state(state.clone())
        .layer(
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
