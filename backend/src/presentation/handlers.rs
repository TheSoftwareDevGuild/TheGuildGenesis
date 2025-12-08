use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;

// Profile imports
use crate::{
    application::{
        commands::{create_profile::create_profile, login::login, update_profile::update_profile},
        dtos::{
            AuthTokenResponse, CreateProfileRequest, NonceResponse, ProfileResponse,
            UpdateProfileRequest,
        },
        queries::{
            get_all_profiles::get_all_profiles, get_login_nonce::get_login_nonce,
            get_profile::get_profile,
        },
    },
    domain::value_objects::WalletAddress,
};

// Project imports
use crate::{
    application::projects::{
        commands::{
            create_project::create_project,
            delete_project::delete_project,
            update_project::update_project,
        },
        dtos::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest},
        queries::{
            get_all_projects::get_all_projects,
            get_project::get_project,
            get_projects_by_creator::get_projects_by_creator,
        },
    },
};

use super::{api::AppState, middlewares::VerifiedWallet};

/// Query parameters for listing projects
#[derive(Debug, Deserialize)]
pub struct ListProjectsQuery {
    pub status: Option<String>,
    pub creator: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

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

pub async fn login_handler(
    Extension(VerifiedWallet(address)): Extension<VerifiedWallet>,
) -> impl IntoResponse {
    match login(address.clone()).await {
        Ok(token) => (StatusCode::OK, Json(AuthTokenResponse { token, address })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}
pub async fn create_project_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    match create_project(state.project_repository, wallet, payload).await {
        Ok(project) => (StatusCode::CREATED, Json(project)).into_response(),
        Err(e) => {
            let status = if e.contains("profiles") {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::BAD_REQUEST
            };
            (status, Json(serde_json::json!({"error": e}))).into_response()
        }
    }
}

pub async fn get_project_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match get_project(state.project_repository, id).await {
        Ok(project) => Json(project).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

pub async fn get_all_projects_handler(
    State(state): State<AppState>,
    Query(query): Query<ListProjectsQuery>,
) -> impl IntoResponse {
    match get_all_projects(
        state.project_repository,
        query.status,
        query.creator,
        query.limit,
        query.offset,
    )
    .await
    {
        Ok(projects) => Json(projects).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,


pub async fn get_projects_by_creator_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    match get_projects_by_creator(state.project_repository, address).await {
        Ok(projects) => Json(projects).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

pub async fn update_project_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProjectRequest>,
) -> impl IntoResponse {
    match update_project(state.project_repository, wallet, id, payload).await {
        Ok(project) => (StatusCode::OK, Json(project)).into_response(),
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.contains("Only the creator") {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::BAD_REQUEST
            };
            (status, Json(serde_json::json!({"error": e}))).into_response()
        }
    }
}

pub async fn delete_project_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(wallet)): Extension<VerifiedWallet>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match delete_project(state.project_repository, wallet, id).await {
        Ok(_) => StatusCode::ACCEPTED.into_response(),
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.contains("Only the creator") {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::BAD_REQUEST
            };
            (status, Json(serde_json::json!({"error": e}))).into_response()
        }
    }
}
