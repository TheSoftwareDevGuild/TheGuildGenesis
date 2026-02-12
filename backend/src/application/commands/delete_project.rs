use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::projects::ProjectId, repositories::project_repository::ProjectRepository,
    value_objects::WalletAddress,
};
pub async fn delete_project(
    repository: Arc<dyn ProjectRepository>,
    requester_address: String,
    project_id: String,
) -> Result<(), String> {
    // Parse project ID
    let id = Uuid::parse_str(&project_id).map_err(|_| "Invalid project ID".to_string())?;
    let project_id = ProjectId::from_uuid(id);

    // Validate requester address
    let requester = WalletAddress::new(requester_address)
        .map_err(|e| format!("Invalid wallet address: {}", e))?;

    // Get existing project
    let project = repository
        .find_by_id(&project_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Project not found".to_string())?;

    // Verify requester is the creator
    if requester != project.creator {
        return Err("Only the creator can delete this project".to_string());
    }

    // Delete from repository
    repository
        .delete(&project_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
