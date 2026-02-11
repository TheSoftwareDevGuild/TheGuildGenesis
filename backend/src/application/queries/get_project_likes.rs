use std::sync::Arc;

use crate::{
    application::dtos::like_dtos::{ProjectLikeResponse, ProjectLikesResponse},
    domain::{
        entities::projects::ProjectId,
        repositories::{ProjectLikeRepository, ProjectRepository},
    },
};

pub async fn get_project_likes(
    project_repository: Arc<dyn ProjectRepository>,
    like_repository: Arc<dyn ProjectLikeRepository>,
    project_id: String,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<ProjectLikesResponse, String> {
    let uuid = uuid::Uuid::parse_str(&project_id).map_err(|_| "Invalid project id".to_string())?;
    let project_id = ProjectId::from_uuid(uuid);

    let exists = project_repository
        .exists(&project_id)
        .await
        .map_err(|e| e.to_string())?;
    if !exists {
        return Err("Project not found".to_string());
    }

    let limit = limit.unwrap_or(50).clamp(1, 100);
    let offset = offset.unwrap_or(0).max(0);

    let total = like_repository
        .count_by_project(&project_id)
        .await
        .map_err(|e| e.to_string())?;
    let likes = like_repository
        .list_by_project(&project_id, limit, offset)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|l| ProjectLikeResponse {
            user_address: l.user_address.to_string(),
            created_at: l.created_at,
        })
        .collect();

    Ok(ProjectLikesResponse {
        project_id: project_id.value().to_string(),
        total,
        likes,
    })
}

