use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    application::{
        commands::{
            create_profile::create_profile, get_all_profiles::get_all_profiles,
            get_profile::get_profile, update_profile::update_profile,
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
) -> impl axum::response::IntoResponse {
    match create_profile(state.profile_repository, wallet, payload).await {
        Ok(profile) => (StatusCode::CREATED, Json(profile)).into_response(),
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

pub async fn get_profile_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    match get_profile(state.profile_repository, address).await {
        Ok(profile) => Json(profile).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub async fn get_all_profiles_handler(State(state): State<AppState>) -> Json<Vec<ProfileResponse>> {
    Json(get_all_profiles(state.profile_repository).await.unwrap())
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
    Json(payload): Json<UpdateProfileRequest>,
) -> impl axum::response::IntoResponse {
    match update_profile(state.profile_repository, wallet, payload).await {
        Ok(profile) => axum::Json(profile).into_response(),
        Err(e) => {
            let status = if e.contains("already taken") {
                axum::http::StatusCode::CONFLICT
            } else {
                axum::http::StatusCode::BAD_REQUEST
            };
            (status, axum::Json(serde_json::json!({"error": e}))).into_response()
        }
    }
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
