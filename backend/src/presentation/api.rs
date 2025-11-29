use std::sync::Arc;

use crate::domain::repositories::ProfileRepository;
use crate::domain::repositories::distribution_repository::DistributionRepository;
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::{
    repositories::PostgresProfileRepository,
    services::ethereum_address_verification_service::EthereumAddressVerificationService,
};
use crate::infrastructure::repositories::postgres_distribution_repository::PostgresDistributionRepository;
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
    create_profile_handler, delete_profile_handler, get_all_profiles_handler, get_nonce_handler,
    get_profile_handler, update_profile_handler, get_distribution_by_id_handler, get_distributions_by_address_handler,
};

use super::handlers::{ create_distribution_handler };

use super::middlewares::{eth_auth_layer, test_auth_layer};

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    let pool_distribution = pool.clone();
    let profile_repository = Arc::from(PostgresProfileRepository::new(pool));
    let auth_service = EthereumAddressVerificationService::new(profile_repository.clone());
    let distribution_repository: Arc<dyn DistributionRepository> =
        Arc::new(PostgresDistributionRepository::new(pool_distribution));

    let state: AppState = AppState {
        profile_repository,
        auth_service: Arc::from(auth_service),
        distribution_repository,
    };

    let protected_routes = Router::new()
        .route("/profiles", post(create_profile_handler))
        .route("/profiles/", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .route("/distributions", post(create_distribution_handler))
        .route("/distributions/:id", get(get_distribution_by_id_handler))
        .route("/distributions/address/:address", get(get_distributions_by_address_handler))
        .with_state(state.clone());

    let protected_with_auth = if std::env::var("TEST_MODE").is_ok() {
        protected_routes.layer(from_fn(test_auth_layer))
    } else {
        protected_routes.layer(from_fn_with_state(state.clone(), eth_auth_layer))
    };

    let public_routes = Router::new()
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles", get(get_all_profiles_handler))
        .route("/auth/nonce/:address", get(get_nonce_handler))
        .with_state(state.clone());

    Router::new()
        .merge(protected_with_auth)
        .merge(public_routes)
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
    pub distribution_repository: Arc<dyn DistributionRepository>,
}

pub fn test_api(state: AppState) -> Router {
    let protected_routes = Router::new()
        .route("/profiles", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .route("/distributions", post(create_distribution_handler))
        .with_state(state.clone())
        .layer(from_fn(test_auth_layer));

    let public_routes = Router::new()
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles", get(get_all_profiles_handler))
        .route("/auth/nonce/:address", get(get_nonce_handler))
        .with_state(state.clone());

    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
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
