use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::dtos::project_dtos::ProjectResponse,
    domain::{entities::projects::ProjectId, repositories::project_repository::ProjectRepository},
};

pub async fn get_project(
    repository: Arc<dyn ProjectRepository>,
    project_id: String,
) -> Result<ProjectResponse, String> {
    // Parse project ID
    let id = Uuid::parse_str(&project_id)
        .map_err(|_| "Invalid project ID".to_string())?;
    let project_id = ProjectId::from_uuid(id);

    // Get project
    let project = repository
        .find_by_id(&project_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Project not found".to_string())?;

    // Return response
    Ok(ProjectResponse {
        id: project.id.value().to_string(),
        name: project.name,
        description: project.description,
        status: project.status,
        creator: project.creator.to_string(),
        created_at: project.created_at,
        updated_at: project.updated_at,
    })
}