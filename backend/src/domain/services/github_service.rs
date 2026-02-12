use async_trait::async_trait;
use serde::Deserialize;

/// Raw issue data returned from the GitHub API
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiIssue {
    pub id: i64,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub html_url: String,
    pub labels: Vec<GitHubApiLabel>,
    pub assignees: Vec<GitHubApiUser>,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub updated_at: String,
    pub pull_request: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiLabel {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiUser {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiRepo {
    pub id: i64,
}

#[async_trait]
pub trait GithubService: Send + Sync {
    /// Fetch issues from a GitHub repository via REST API.
    /// `repo` is the repository name (e.g. "TheGuildGenesis"); owner comes from GITHUB_OWNER env var.
    /// `since` is an optional ISO 8601 timestamp to filter issues updated since that time.
    async fn fetch_issues(
        &self,
        repo: &str,
        since: Option<&str>,
    ) -> Result<(i64, Vec<GitHubApiIssue>), Box<dyn std::error::Error>>;
}
