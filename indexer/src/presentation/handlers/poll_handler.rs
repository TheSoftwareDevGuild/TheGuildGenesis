use axum::{extract::State, http::StatusCode};

use crate::{application::commands::poll::poll, presentation::api::AppState};

#[axum::debug_handler]
pub async fn poll_handler(State(state): State<AppState>) -> StatusCode {
    poll(
        state.ethereum_event_polling_service,
        state.ethereum_event_repository,
    )
    .await
    .unwrap();
    StatusCode::CREATED
}
