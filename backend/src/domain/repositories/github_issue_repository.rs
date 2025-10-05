use async_trait::async_trait;

use crate::domain::entities::github_issue::GithubIssue;

#[async_trait]
pub trait GithubIssueRepository: Send + Sync {
    async fn upsert_issues(&self, issues: &[GithubIssue]) -> anyhow::Result<()>;
}
