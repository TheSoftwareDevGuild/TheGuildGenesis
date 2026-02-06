use std::sync::Arc;

use crate::domain::{
    entities::projects::ProjectId,
    repositories::{ProjectLikeRepository, ProjectRepository},
    value_objects::WalletAddress,
};

pub async fn delete_project_like(
    project_repository: Arc<dyn ProjectRepository>,
    like_repository: Arc<dyn ProjectLikeRepository>,
    user_address: String,
    project_id: String,
) -> Result<(), String> {
    let user_address = WalletAddress::new(user_address.to_lowercase())
        .map_err(|e| format!("Invalid wallet address: {}", e))?;

    let uuid = uuid::Uuid::parse_str(&project_id).map_err(|_| "Invalid project id".to_string())?;
    let project_id = ProjectId::from_uuid(uuid);

    let exists = project_repository
        .exists(&project_id)
        .await
        .map_err(|e| e.to_string())?;
    if !exists {
        return Err("Project not found".to_string());
    }

    let deleted = like_repository
        .delete(&project_id, &user_address)
        .await
        .map_err(|e| e.to_string())?;

    if !deleted {
        return Err("Like not found".to_string());
    }

    Ok(())
}

