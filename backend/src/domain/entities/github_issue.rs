use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubIssue {
    pub repo_id: i64,
    pub github_issue_id: i64,
    pub repo: String,
    pub number: i32,
    pub title: String,
    pub state: String,
    pub labels: serde_json::Value,
    pub points: i32,
    pub assignee_logins: serde_json::Value,
    pub html_url: String,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub rewarded: bool,
    pub distribution_id: Option<String>,
    pub updated_at: DateTime<Utc>,
}
