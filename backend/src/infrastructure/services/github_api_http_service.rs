use anyhow::Result;
use async_trait::async_trait;

use crate::domain::services::github_api_service::{GithubApiService, GithubIssueApi, GithubRepoApi};

#[derive(Clone)]
pub struct GithubApiHttpService {
    client: reqwest::Client,
}

impl GithubApiHttpService {
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
    }
}

#[async_trait]
impl GithubApiService for GithubApiHttpService {
    async fn get_repo(&self, repo_full: &str) -> Result<GithubRepoApi> {
        let repo_api: GithubRepoApi = self.client
            .get(format!("https://api.github.com/repos/{}", repo_full))
            .header("User-Agent", "guild-backend")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(repo_api)
    }

    async fn list_issues(&self, repo_full: &str, since: Option<&str>) -> Result<Vec<GithubIssueApi>> {
        let mut url = format!(
            "https://api.github.com/repos/{}/issues?state=all",
            repo_full
        );
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        let issues_api: Vec<GithubIssueApi> = self.client
            .get(url)
            .header("User-Agent", "guild-backend")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(issues_api)
    }
}