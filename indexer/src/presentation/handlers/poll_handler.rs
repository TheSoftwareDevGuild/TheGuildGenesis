use axum::{extract::State, Json};

use crate::{
    application::queries::poll::poll, domain::entities::ethereum_event::EthereumEvent,
    presentation::api::AppState,
};

#[axum::debug_handler]
pub async fn poll_handler(State(_state): State<AppState>) -> Json<Vec<EthereumEvent>> {
    let results = poll().await.unwrap();
    Json(results)
}
