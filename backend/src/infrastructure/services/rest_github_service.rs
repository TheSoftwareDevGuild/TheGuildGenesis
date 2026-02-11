use async_trait::async_trait;

use crate::domain::services::github_service::{GitHubApiIssue, GitHubApiRepo, GithubService};

pub struct RestGithubService {
    client: reqwest::Client,
}

impl Default for RestGithubService {
    fn default() -> Self {
        Self::new()
    }
}

impl RestGithubService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("guild-backend")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }
}

#[async_trait]
impl GithubService for RestGithubService {
    async fn fetch_issues(
        &self,
        repo: &str,
        since: Option<&str>,
    ) -> Result<(i64, Vec<GitHubApiIssue>), Box<dyn std::error::Error>> {
        // Fetch repo metadata to get repo_id
        let repo_url = format!("https://api.github.com/repos/{repo}");
        let repo_resp: GitHubApiRepo = self
            .client
            .get(&repo_url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        // Fetch issues (no pagination per scope)
        let mut issues_url = format!(
            "https://api.github.com/repos/{repo}/issues?state=all&per_page=100"
        );
        if let Some(since_val) = since {
            issues_url.push_str(&format!("&since={since_val}"));
        }

        let issues: Vec<GitHubApiIssue> = self
            .client
            .get(&issues_url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok((repo_resp.id, issues))
    }
}
