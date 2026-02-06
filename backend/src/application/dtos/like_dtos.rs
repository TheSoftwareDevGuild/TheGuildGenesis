use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLikeResponse {
    pub user_address: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLikesResponse {
    pub project_id: String,
    pub total: i64,
    pub likes: Vec<ProjectLikeResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLikeCreatedResponse {
    pub project_id: String,
    pub user_address: String,
}
