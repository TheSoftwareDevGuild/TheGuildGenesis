use axum::{extract::State, http::StatusCode, Json};

use crate::{
    application::queries::list_events::list_events,
    domain::entities::ethereum_event::EthereumEvent,
    presentation::api::AppState,
};

#[axum::debug_handler]
pub async fn list_events_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<EthereumEvent>>, StatusCode> {
    match list_events(state.ethereum_event_repository.clone()).await {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}