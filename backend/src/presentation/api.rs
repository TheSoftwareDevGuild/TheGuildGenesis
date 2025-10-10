use std::sync::Arc;

use crate::domain::repositories::{GithubIssueRepository, ProfileRepository};
use crate::domain::services::auth_service::AuthService;
use crate::domain::services::github_api_service::GithubApiService;
use crate::infrastructure::{
    repositories::{PostgresGithubIssueRepository, PostgresProfileRepository},
    services::ethereum_address_verification_service::EthereumAddressVerificationService,
    services::github_api_http_service::GithubApiHttpService,
};
use axum::middleware::from_fn_with_state;
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
    github_sync_handler, update_profile_handler,
};

use super::middlewares::eth_auth_layer;

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    let auth_service = EthereumAddressVerificationService::new();
    let profile_repository = PostgresProfileRepository::new(pool.clone());
    let github_issue_repository = PostgresGithubIssueRepository::new(pool.clone());
    let github_api_service = GithubApiHttpService::new();

    let state: AppState = AppState {
        profile_repository: Arc::from(profile_repository),
        github_issue_repository: Arc::from(github_issue_repository),
        github_api_service: Arc::from(github_api_service),
        auth_service: Arc::from(auth_service),
    };

    let protected = Router::new()
        .route("/profiles", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .with_state(state.clone())
        .layer(from_fn_with_state(state.clone(), eth_auth_layer));

    let public = Router::new()
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles", get(get_all_profiles_handler))
        .route("/admin/github/sync", post(github_sync_handler))
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

// Helper for tests: build the same router using a provided AppState
pub fn test_api(state: AppState) -> Router {
    let protected = Router::new()
        .route("/profiles", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .with_state(state.clone())
        .layer(from_fn_with_state(state.clone(), eth_auth_layer));

    let public = Router::new()
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles", get(get_all_profiles_handler))
        .route("/admin/github/sync", post(github_sync_handler))
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
    pub github_issue_repository: Arc<dyn GithubIssueRepository>,
    pub github_api_service: Arc<dyn GithubApiService>,
    pub auth_service: Arc<dyn AuthService>,
}
