use axum::{extract::State, http::StatusCode, Extension, Json};

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

// --- GitHub Sync ---
use crate::application::commands::github_sync::sync_github_issues;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubSyncRequest {
    pub repos: Option<Vec<String>>, // org/repo
    pub since: Option<String>,      // ISO string
}

pub async fn github_sync_handler(
    State(state): State<AppState>,
    Json(payload): Json<GithubSyncRequest>,
) -> StatusCode {
    let repos = payload
        .repos
        .unwrap_or_else(|| vec!["TheGuildGenesis/TheGuildGenesis".to_string()]);

    sync_github_issues(&*state.github_issue_repository, &*state.github_api_service, &repos, payload.since)
        .await
        .unwrap();
    StatusCode::ACCEPTED
}

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

pub async fn get_all_profiles_handler(State(state): State<AppState>) -> Json<Vec<ProfileResponse>> {
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
