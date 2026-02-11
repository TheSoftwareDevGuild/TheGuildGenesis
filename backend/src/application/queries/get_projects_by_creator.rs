use std::sync::Arc;

use crate::{
    application::dtos::project_dtos::ProjectResponse,
    domain::{repositories::project_repository::ProjectRepository, value_objects::WalletAddress},
};

pub async fn get_projects_by_creator(
    repository: Arc<dyn ProjectRepository>,
    creator_address: String,
) -> Result<Vec<ProjectResponse>, String> {
    // Validate creator address
    let creator = WalletAddress::new(creator_address)
        .map_err(|e| format!("Invalid wallet address: {e}"))?;

    // Get projects
    let projects = repository
        .find_by_creator(&creator)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to responses
    Ok(projects
        .into_iter()
        .map(|project| ProjectResponse {
            id: project.id.value().to_string(),
            name: project.name,
            description: project.description,
            status: project.status,
            creator: project.creator.to_string(),
            created_at: project.created_at,
            updated_at: project.updated_at,
        })
        .collect())
}
