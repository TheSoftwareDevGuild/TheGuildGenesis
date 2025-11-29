use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use uuid::Uuid;
use crate::{
    application::{
        commands::{create_profile::create_profile, update_profile::update_profile},
        commands::{create_distribution::CreateDistribution},
        dtos::{CreateProfileRequest, NonceResponse, ProfileResponse, UpdateProfileRequest},
        dtos::{CreateDistributionRequest, DistributionResponse},
        queries::{
            get_all_profiles::get_all_profiles, get_login_nonce::get_login_nonce,
            get_profile::get_profile,
        },
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
        Ok(profile) => (StatusCode::OK, axum::Json(profile)).into_response(),
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

pub async fn get_nonce_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    match get_login_nonce(state.profile_repository, address.clone()).await {
        Ok(nonce) => Json(NonceResponse { nonce, address }).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub async fn create_distribution_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateDistributionRequest>,
) -> impl IntoResponse {

    let items_for_cmd = payload
        .items
        .into_iter()
        .map(|it| (it.address, it.badge_name, it.metadata))
        .collect::<Vec<_>>();

    let distribution_repo = state.distribution_repository.clone();
    let cmd = CreateDistribution::new(distribution_repo.as_ref());

    match cmd.execute(items_for_cmd, payload.metadata).await {
        Ok(distribution_id) => {
            let items = distribution_repo.get_by_distribution_id(distribution_id).await.unwrap();

            let body = items.into_iter().map(|i| DistributionResponse {
                id: i.id,
                distribution_id: i.distribution_id,
                address: i.address.as_str().to_string(),
                badge_name: i.badge_name,
                metadata: i.metadata.expect("Metadata missing"),
                created_at: i.created_at.naive_utc(),
            }).collect::<Vec<_>>();

            (StatusCode::CREATED, Json(body)).into_response()
        }
        Err(e) => {
            tracing::error!("create_distribution failed: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create distribution"}))).into_response()
        }
    }
}


pub async fn get_distribution_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let repo = state.distribution_repository.clone();

    match repo.get_by_distribution_id(id).await {
        Ok(items) => {
            let response = items
                .into_iter()
                .map(|i| DistributionResponse {
                    id: i.id,
                    distribution_id: i.distribution_id,
                    address: i.address.as_str().to_string(),
                    badge_name: i.badge_name,
                    metadata: i.metadata.expect("Metadata missing"),
                    created_at: i.created_at.naive_utc(),
                })
                .collect::<Vec<_>>();

            Json(response).into_response()
        }
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}

pub async fn get_distributions_by_address_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    let repo = state.distribution_repository.clone();

    match repo.get_by_address(&address).await {
        Ok(items) => {
            let response = items
                .into_iter()
                .map(|i| DistributionResponse {
                    id: i.id,
                    distribution_id: i.distribution_id,
                    address: i.address.as_str().to_string(),
                    badge_name: i.badge_name,
                    metadata: i.metadata.expect("Metadata missing"),
                    created_at: i.created_at.naive_utc(),
                })
                .collect::<Vec<_>>();

            Json(response).into_response()
        }
        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": err.to_string() })),
        )
            .into_response(),
    }
}