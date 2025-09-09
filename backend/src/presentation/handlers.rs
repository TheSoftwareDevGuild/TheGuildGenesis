use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{application::dtos::CreateProfileRequest, domain::value_objects::WalletAddress};

use super::{api::AppState, middlewares::VerifiedWallet};

pub async fn create_profile_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProfileRequest>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
) -> StatusCode {
    state.profile_repository.create(payload);
    StatusCode::CREATED
}

pub async fn get_profile_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
) -> StatusCode {
    state.profile_repository.get(&WalletAddress(wallet));
    StatusCode::NO_CONTENT
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateProfileRequest>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
) -> StatusCode {
    state.profile_repository.update(payload);
    StatusCode::ACCEPTED
}

pub async fn delete_profile_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
) -> StatusCode {
    state
        .profile_repository
        .delete(&WalletAddress(wallet))
        .await
        .unwrap();
    StatusCode::ACCEPTED
}
