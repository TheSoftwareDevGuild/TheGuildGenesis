use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use super::handlers::list_events_handler::list_events_handler;
use super::handlers::poll_handler::poll_handler;
use crate::domain::repositories::ethereum_event_repository::EthereumEventRepository;
use crate::domain::services::ethereum_event_polling_service::EthereumEventPollingService;
use crate::infrastructure::repositories::postgres_ethereum_event_repository::PostgresEthereumEventRepository;
use crate::infrastructure::services::alloy_ethereum_event_polling_service::AlloyEthereumEventPollingService;

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    let postgres_ethereum_event_repository = PostgresEthereumEventRepository::new(pool);
    let alloy_ethereum_event_polling_service = AlloyEthereumEventPollingService::new();

    let state: AppState = AppState {
        ethereum_event_repository: Arc::from(postgres_ethereum_event_repository),
        ethereum_event_polling_service: Arc::from(alloy_ethereum_event_polling_service),
    };

    let router = Router::new()
        .route("/poll/", post(poll_handler))
        .route("/events/", get(list_events_handler))
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
    pub ethereum_event_repository: Arc<dyn EthereumEventRepository>,
    pub ethereum_event_polling_service: Arc<dyn EthereumEventPollingService>,
}
