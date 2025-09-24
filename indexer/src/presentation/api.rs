use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use super::handlers::poll_handler::poll_handler;
use crate::domain::repositories::ethereum_event_repository::EthereumEventRepository;
use crate::infrastructure::repositories::postgres_ethereum_event_repository::PostgresEthereumEventRepository;

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    let ethereum_logs_repository = PostgresEthereumEventRepository::new(pool);

    let state: AppState = AppState {
        ethereum_logs_repository: Arc::from(ethereum_logs_repository),
    };

    let router = Router::new()
        .route("/poll/", post(poll_handler))
        .with_state(state.clone());

    router.with_state(state.clone()).layer(
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
    pub ethereum_logs_repository: Arc<dyn EthereumEventRepository>,
}
