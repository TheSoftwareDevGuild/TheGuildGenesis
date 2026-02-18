use async_trait::async_trait;

use crate::domain::entities::github_issue::GithubIssue;

#[async_trait]
pub trait GithubIssueRepository: Send + Sync {
    /// Upsert a GitHub issue (insert or update based on composite key repo_id + github_issue_id)
    async fn upsert(&self, issue: &GithubIssue) -> Result<(), Box<dyn std::error::Error>>;

    /// Find an issue by its composite key
    async fn find_by_key(
        &self,
        repo_id: i64,
        github_issue_id: i64,
    ) -> Result<Option<GithubIssue>, Box<dyn std::error::Error>>;

    /// List issues filtered by repo name and optional state
    async fn list_by_repo(
        &self,
        repo: &str,
        state: Option<&str>,
    ) -> Result<Vec<GithubIssue>, Box<dyn std::error::Error>>;
}
