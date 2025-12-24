use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::entities::projects::ProjectStatus;

/// Request DTO for creating a project
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
}

/// Request DTO for updating a project
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<ProjectStatus>,
}

/// Response DTO for a project
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub creator: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
