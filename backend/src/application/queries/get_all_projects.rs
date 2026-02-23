use std::sync::Arc;

use crate::{
    application::dtos::project_dtos::ProjectResponse,
    domain::{
        entities::projects::ProjectStatus, repositories::project_repository::ProjectRepository,
        value_objects::WalletAddress,
    },
};

pub async fn get_all_projects(
    repository: Arc<dyn ProjectRepository>,
    status: Option<String>,
    creator: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<ProjectResponse>, String> {
    let status_filter = if let Some(status_str) = status {
        Some(
            status_str
                .parse::<ProjectStatus>()
                .map_err(|e| format!("Invalid status: {}", e))?,
        )
    } else {
        None
    };

    let creator_filter = if let Some(creator_str) = creator {
        Some(
            WalletAddress::new(creator_str)
                .map_err(|e| format!("Invalid creator address: {}", e))?,
        )
    } else {
        None
    };

    let limit = limit.map(|l| l.clamp(1, 100));
    let offset = offset.map(|o| o.max(0));

    let projects = repository
        .find_all(status_filter, creator_filter.as_ref(), limit, offset)
        .await
        .map_err(|e| e.to_string())?;

    Ok(projects
        .into_iter()
        .map(|project| {
            let creator_str = project.creator.to_string();
            ProjectResponse {
                id: project.id.value().to_string(),
                name: project.name,
                description: project.description,
                status: project.status,
                owner_address: creator_str.clone(),
                creator: creator_str,
                created_at: project.created_at,
                updated_at: project.updated_at,
            }
        })
        .collect())
}
