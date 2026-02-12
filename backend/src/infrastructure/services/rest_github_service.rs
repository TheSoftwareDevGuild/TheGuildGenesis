use async_trait::async_trait;

use crate::domain::services::github_service::{GitHubApiIssue, GitHubApiRepo, GithubService};

pub struct RestGithubService {
    client: reqwest::Client,
    api_url: String,
    owner: String,
}

impl Default for RestGithubService {
    fn default() -> Self {
        Self::new()
    }
}

impl RestGithubService {
    pub fn new() -> Self {
        let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
        let api_url = std::env::var("GITHUB_API_URL")
            .unwrap_or_else(|_| "https://api.github.com".to_string());
        let owner = std::env::var("GITHUB_OWNER").unwrap_or_default();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            "application/vnd.github+json".parse().unwrap(),
        );
        if !token.is_empty() {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").parse().unwrap(),
            );
        }

        Self {
            client: reqwest::Client::builder()
                .user_agent("guild-backend")
                .default_headers(headers)
                .build()
                .expect("Failed to build HTTP client"),
            api_url,
            owner,
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
        let repo_url = format!("{}/repos/{}/{}", self.api_url, self.owner, repo);
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
            "{}/repos/{}/{}/issues?state=all&per_page=100",
            self.api_url, self.owner, repo
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
