use serde::{Deserialize, Serialize};

/// Request DTO for POST /admin/github/sync
#[derive(Debug, Deserialize)]
pub struct GithubSyncRequest {
    pub repos: Vec<String>,
    pub since: Option<String>,
}

/// Response DTO for POST /admin/github/sync
#[derive(Debug, Serialize)]
pub struct GithubSyncResponse {
    pub synced: usize,
    pub repos: Vec<String>,
}
