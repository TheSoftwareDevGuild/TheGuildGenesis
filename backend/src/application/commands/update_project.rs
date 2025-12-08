use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::projects::dtos::{ProjectResponse, UpdateProjectRequest},
    domain::{
        entities::projects::ProjectId, repositories::project_repository::ProjectRepository,
        value_objects::WalletAddress,
    },
};

pub async fn update_project(
    repository: Arc<dyn ProjectRepository>,
    requester_address: String,
    project_id: String,
    request: UpdateProjectRequest,
) -> Result<ProjectResponse, String> {
    // Parse project ID
    let id = Uuid::parse_str(&project_id)
        .map_err(|_| "Invalid project ID".to_string())?;
    let project_id = ProjectId::from_uuid(id);

    // Validate requester address
    let requester = WalletAddress::new(requester_address)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;

    // Get existing project
    let mut project = repository
        .find_by_id(&project_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Project not found".to_string())?;

    // Verify requester is the creator
    if requester != project.creator {
        return Err("Only the creator can update this project".to_string());
    }

    // Update project
    project.update_info(request.name, request.description, request.status);

    // Validate updated project
    project.validate()?;

    // Save to repository
    repository
        .update(&project)
        .await
        .map_err(|e| e.to_string())?;

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