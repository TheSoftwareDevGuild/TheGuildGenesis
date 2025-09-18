use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use eyre::Result;

use crate::{
    application::{
        commands::{get_indexing_status::GetIndexingStatusCommand, index_logs::IndexLogsCommand, index_logs_by_filter::IndexLogsByFilterCommand},
        dtos::indexing_dtos::{IndexingRequest, IndexingResponse, IndexingStatusResponse, LogFilterRequest, LogsResponse},
    },
    presentation::AppState,
};

pub async fn index_logs_handler(
    State(state): State<AppState>,
    Json(request): Json<IndexingRequest>,
) -> Result<Json<IndexingResponse>, StatusCode> {
    let command = IndexLogsCommand::new(
        state.indexing_service.clone(),
        state.progress_repository.clone(),
    );

    match command.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Failed to index logs: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn index_logs_by_filter_handler(
    State(state): State<AppState>,
    Json(request): Json<LogFilterRequest>,
) -> Result<Json<LogsResponse>, StatusCode> {
    let command = IndexLogsByFilterCommand::new(
        state.indexing_service.clone(),
        state.log_repository.clone(),
    );

    match command.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Failed to index logs by filter: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_indexing_status_handler(
    State(state): State<AppState>,
    axum::extract::Path(chain_id): axum::extract::Path<i32>,
) -> Result<Json<IndexingStatusResponse>, StatusCode> {
    let command = GetIndexingStatusCommand::new(state.progress_repository.clone());

    match command.execute(chain_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Failed to get indexing status: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn health_check_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "guild-indexer"
    })))
}
