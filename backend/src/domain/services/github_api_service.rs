use async_trait::async_trait;
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize)]
pub struct GithubRepoApi {
    pub id: i64,
    pub full_name: String, // org/repo
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubLabel {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubAssignee {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubIssueApi {
    pub id: i64,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub html_url: String,
    pub pull_request: Option<serde_json::Value>,
    pub labels: Vec<GithubLabel>,
    pub assignees: Vec<GithubAssignee>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>, 
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait GithubApiService: Send + Sync {
    async fn get_repo(&self, repo_full: &str) -> anyhow::Result<GithubRepoApi>;
    async fn list_issues(&self, repo_full: &str, since: Option<&str>) -> anyhow::Result<Vec<GithubIssueApi>>;
}
