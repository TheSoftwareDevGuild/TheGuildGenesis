use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::{
    application::{
        commands::{
            create_profile::create_profile, get_profile::get_profile,
            get_all_profiles::get_all_profiles,
            update_profile::update_profile,
        },
        dtos::{CreateProfileRequest, ProfileResponse, UpdateProfileRequest},
    },
    domain::value_objects::WalletAddress,
};

use super::{api::AppState, middlewares::VerifiedWallet};

pub async fn create_profile_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
    Json(payload): Json<CreateProfileRequest>,
) -> StatusCode {
    create_profile(state.profile_repository, wallet, payload)
        .await
        .unwrap();
    StatusCode::CREATED
}

pub async fn get_profile_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
) -> Json<ProfileResponse> {
    Json(get_profile(state.profile_repository, wallet).await.unwrap())
}

pub async fn get_all_profiles_handler(
    State(state): State<AppState>,
) -> Json<Vec<ProfileResponse>> {
    Json(get_all_profiles(state.profile_repository).await.unwrap())
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
    Json(payload): Json<UpdateProfileRequest>,
) -> StatusCode {
    update_profile(state.profile_repository, wallet, payload)
        .await
        .unwrap();
    StatusCode::CREATED
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
