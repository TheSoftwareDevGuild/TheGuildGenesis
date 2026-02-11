use std::sync::Arc;

use crate::domain::repositories::{ProfileRepository, ProjectLikeRepository, ProjectRepository};
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::{
    repositories::{
        postgres_project_repository::PostgresProjectRepository, PostgresProfileRepository,
        PostgresProjectLikeRepository,
    },
    services::ethereum_address_verification_service::EthereumAddressVerificationService,
};
use axum::middleware::{from_fn, from_fn_with_state};
use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use super::handlers::{
    // Admin handlers
    admin_delete_profile_handler,
    // Profile handlers
    create_profile_handler,
    // Project handlers
    create_project_handler,
    delete_profile_handler,
    delete_project_handler,
    get_project_likes_handler,
    get_all_profiles_handler,
    get_nonce_handler,
    get_profile_handler,
    get_project_handler,
    get_user_projects_handler,
    like_project_handler,
    list_projects_handler,
    login_handler,
    unlike_project_handler,
    update_profile_handler,
    update_project_handler,
};

use super::middlewares::{admin_auth_layer, eth_auth_layer, test_auth_layer};

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    let profile_repository = Arc::from(PostgresProfileRepository::new(pool.clone()));
    let project_repository = Arc::from(PostgresProjectRepository::new(pool.clone()));
    let project_like_repository = Arc::from(PostgresProjectLikeRepository::new(pool.clone()));
    let auth_service = EthereumAddressVerificationService::new(profile_repository.clone());

    let state: AppState = AppState {
        profile_repository,
        project_repository,
        project_like_repository,
        auth_service: Arc::from(auth_service),
    };

    // Protected routes (require authentication)
    let protected_routes = Router::new()
        // Profile protected routes
        .route("/profiles", post(create_profile_handler))
        .route("/profiles/", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .route("/auth/login", post(login_handler))
        // Project protected routes
        .route("/projects", post(create_project_handler))
        .route("/projects/:id", patch(update_project_handler))
        .route("/projects/:id", delete(delete_project_handler))
        .route("/projects/:id/likes", post(like_project_handler))
        .route("/projects/:id/likes", delete(unlike_project_handler))
        .with_state(state.clone());

    let protected_with_auth = if std::env::var("TEST_MODE").is_ok() {
        protected_routes.layer(from_fn(test_auth_layer))
    } else {
        protected_routes.layer(from_fn_with_state(state.clone(), eth_auth_layer))
    };

    // Admin routes (require admin authentication via SIWE with admin wallet)
    let admin_routes = Router::new()
        .route(
            "/admin/profiles/:address",
            delete(admin_delete_profile_handler),
        )
        .with_state(state.clone());

    let admin_with_auth = if std::env::var("TEST_MODE").is_ok() {
        // In test mode, still check x-eth-address header but skip signature verification
        admin_routes.layer(from_fn(test_auth_layer))
    } else {
        admin_routes.layer(from_fn_with_state(state.clone(), admin_auth_layer))
    };

    // Public routes (no authentication)
    let public_routes = Router::new()
        // Profile public routes
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles", get(get_all_profiles_handler))
        .route("/auth/nonce/:address", get(get_nonce_handler))
        // Project public routes
        .route("/projects", get(list_projects_handler))
        .route("/projects/:id", get(get_project_handler))
        .route("/projects/:id/likes", get(get_project_likes_handler))
        .route("/users/:address/projects", get(get_user_projects_handler))
        .with_state(state.clone());

    Router::new()
        .merge(protected_with_auth)
        .merge(admin_with_auth)
        .merge(public_routes)
        .with_state(state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::PATCH,
                            Method::DELETE,
                        ])
                        .allow_headers(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024)),
        )
}

#[derive(Clone)]
pub struct AppState {
    pub profile_repository: Arc<dyn ProfileRepository>,
    pub project_repository: Arc<dyn ProjectRepository>,
    pub project_like_repository: Arc<dyn ProjectLikeRepository>,
    pub auth_service: Arc<dyn AuthService>,
}

pub fn test_api(state: AppState) -> Router {
    // Protected routes (require authentication)
    let protected_routes = Router::new()
        // Profile protected routes
        .route("/profiles", post(create_profile_handler))
        .route("/profiles/:address", put(update_profile_handler))
        .route("/profiles/:address", delete(delete_profile_handler))
        .route("/auth/login", post(login_handler))
        // Project protected routes
        .route("/projects", post(create_project_handler))
        .route("/projects/:id", patch(update_project_handler))
        .route("/projects/:id", delete(delete_project_handler))
        .route("/projects/:id/likes", post(like_project_handler))
        .route("/projects/:id/likes", delete(unlike_project_handler))
        .with_state(state.clone())
        .layer(from_fn(test_auth_layer));

    // Admin routes (require admin authentication)
    let admin_routes = Router::new()
        .route(
            "/admin/profiles/:address",
            delete(admin_delete_profile_handler),
        )
        .with_state(state.clone())
        .layer(from_fn(test_auth_layer));

    // Public routes (no authentication)
    let public_routes = Router::new()
        // Profile public routes
        .route("/profiles/:address", get(get_profile_handler))
        .route("/profiles", get(get_all_profiles_handler))
        .route("/auth/nonce/:address", get(get_nonce_handler))
        // Project public routes
        .route("/projects", get(list_projects_handler))
        .route("/projects/:id", get(get_project_handler))
        .route("/projects/:id/likes", get(get_project_likes_handler))
        .route("/users/:address/projects", get(get_user_projects_handler))
        .with_state(state.clone());

    Router::new()
        .merge(protected_routes)
        .merge(admin_routes)
        .merge(public_routes)
        .with_state(state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::PATCH,
                            Method::DELETE,
                        ])
                        .allow_headers(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024)),
        )
}
