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
use crate::application::{
    commands::{
        create_project::create_project, delete_project::delete_project,
        update_project::update_project,
    },
    dtos::project_dtos::{CreateProjectRequest, UpdateProjectRequest},
    queries::{
        get_all_projects::get_all_projects, get_project::get_project,
        get_projects_by_creator::get_projects_by_creator,
    },
};

// GitHub sync imports
use crate::application::{
    commands::sync_github_issues::sync_github_issues,
    dtos::github_dtos::{GithubIssuesQuery, GithubSyncRequest, GithubSyncResponse},
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

/// GET /projects - List all projects with optional filters
pub async fn list_projects_handler(
    State(state): State<AppState>,
    Query(params): Query<ListProjectsQuery>,
) -> impl IntoResponse {
    match get_all_projects(
        state.project_repository.clone(),
        params.status,
        params.creator,
        params.limit,
        params.offset,
    )
    .await
    {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(e) => {
            let status = if e.contains("Invalid status") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(serde_json::json!({"error": e}))).into_response()
        }
    }
}

// GET /projects/:id - Get a single project by ID
pub async fn get_project_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match get_project(state.project_repository.clone(), id).await {
        Ok(project) => (StatusCode::OK, Json(project)).into_response(),
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::BAD_REQUEST
            };
            (status, Json(serde_json::json!({"error": e}))).into_response()
        }
    }
}

/// GET /users/:address/projects - Get all projects by a creator
pub async fn get_user_projects_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> impl IntoResponse {
    match get_projects_by_creator(state.project_repository.clone(), address).await {
        Ok(projects) => (StatusCode::OK, Json(projects)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

/// POST /projects - Create a new project (Protected)
pub async fn create_project_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(verified_wallet)): Extension<VerifiedWallet>,
    Json(request): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    match create_project(state.project_repository.clone(), verified_wallet, request).await {
        Ok(project) => (StatusCode::CREATED, Json(project)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

/// PATCH /projects/:id - Update a project (Protected, creator only)
pub async fn update_project_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(verified_wallet)): Extension<VerifiedWallet>,
    Path(id): Path<String>,
    Json(request): Json<UpdateProjectRequest>,
) -> impl IntoResponse {
    match update_project(
        state.project_repository.clone(),
        verified_wallet,
        id,
        request,
    )
    .await
    {
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

/// DELETE /projects/:id - Delete a project (Protected, creator only)
pub async fn delete_project_handler(
    State(state): State<AppState>,
    Extension(VerifiedWallet(verified_wallet)): Extension<VerifiedWallet>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match delete_project(state.project_repository.clone(), verified_wallet, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
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

// ============================================================================
// Admin Handlers
// ============================================================================

/// DELETE /admin/profiles/:address - Admin delete a profile (bypasses ownership check)
/// Used for deleting deprecated profiles or spam that we don't have access to anymore.
pub async fn admin_delete_profile_handler(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> axum::response::Response {
    let wallet_address = WalletAddress(address.clone());

    // Check if profile exists first
    let profile_exists = match state
        .profile_repository
        .find_by_address(&wallet_address)
        .await
    {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            tracing::error!("Error finding profile {}: {}", address, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Error finding profile: {}", e)})),
            )
                .into_response();
        }
    };

    if !profile_exists {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Profile not found"})),
        )
            .into_response();
    }

    // Profile exists, delete it
    match state.profile_repository.delete(&wallet_address).await {
        Ok(_) => {
            tracing::info!("Admin deleted profile: {}", address);
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            tracing::error!("Failed to delete profile {}: {}", address, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Failed to delete profile: {}", e)})),
            )
                .into_response()
        }
    }
}

// ============================================================================
// GitHub Sync Handlers
// ============================================================================

/// POST /admin/github/sync - Sync GitHub issues for specified repos (Admin only)
pub async fn github_sync_handler(
    State(state): State<AppState>,
    Json(request): Json<GithubSyncRequest>,
) -> impl IntoResponse {
    if request.repos.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "repos must not be empty"})),
        )
            .into_response();
    }

    match sync_github_issues(
        state.github_service.clone(),
        state.github_issue_repository.clone(),
        request.repos.clone(),
        request.since,
    )
    .await
    {
        Ok(synced) => (
            StatusCode::OK,
            Json(GithubSyncResponse {
                synced,
                repos: request.repos,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

/// GET /github/issues?repo=<name>&state=<open|closed> - List synced GitHub issues (Public)
pub async fn list_github_issues_handler(
    State(state): State<AppState>,
    Query(params): Query<GithubIssuesQuery>,
) -> impl IntoResponse {
    match state
        .github_issue_repository
        .list_by_repo(&params.repo, params.state.as_deref())
        .await
    {
        Ok(issues) => (StatusCode::OK, Json(issues)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to fetch issues: {e}")})),
        )
            .into_response(),
    }
}
