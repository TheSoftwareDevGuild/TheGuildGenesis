use axum::{extract::State, http::StatusCode, Json};

use crate::application::dtos::CreateProfileRequest;

use super::api::AppState;

pub async fn create_profile_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProfileRequest>,
) -> StatusCode {
    StatusCode::CREATED
}

pub async fn get_profile_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProfileRequest>,
) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProfileRequest>,
) -> StatusCode {
    StatusCode::ACCEPTED
}
