use std::sync::Arc;

use crate::{
    application::dtos::project_dtos::{CreateProjectRequest, ProjectResponse},
    domain::{
        entities::projects::Project, repositories::project_repository::ProjectRepository,
        value_objects::WalletAddress,
    },
};

pub async fn create_project(
    repository: Arc<dyn ProjectRepository>,
    creator_address: String,
    request: CreateProjectRequest,
) -> Result<ProjectResponse, String> {
    // Validate and create WalletAddress
    let creator = WalletAddress::new(creator_address)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;

    // Verify creator has a profile
    if !repository
        .profile_exists(&creator)
        .await
        .map_err(|e| e.to_string())?
    {
        return Err("Only addresses with profiles can create projects".to_string());
    }

    // Create project entity
    let project = Project::new(request.name, request.description, request.status, creator);

    // Validate project
    project.validate()?;

    // Save to repository
    repository
        .create(&project)
        .await
        .map_err(|e| e.to_string())?;

    let creator_str = project.creator.to_string();

    Ok(ProjectResponse {
        id: project.id.value().to_string(),
        name: project.name,
        description: project.description,
        status: project.status,
        owner_address: creator_str.clone(),
        creator: creator_str,
        created_at: project.created_at,
        updated_at: project.updated_at,
    })
}
