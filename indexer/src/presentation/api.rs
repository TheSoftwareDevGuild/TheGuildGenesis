use axum::{
    routing::{get, post},
    Router,
};

use crate::presentation::handlers;

pub fn create_router() -> Router<crate::presentation::AppState> {
    Router::new()
        .route("/health", get(handlers::health_check_handler))
        .route("/index/logs", post(handlers::index_logs_handler))
        .route("/index/logs/filter", post(handlers::index_logs_by_filter_handler))
        .route("/status/:chain_id", get(handlers::get_indexing_status_handler))
}
