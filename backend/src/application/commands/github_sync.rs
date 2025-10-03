use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::domain::{
    entities::github_issue::GithubIssue,
    repositories::GithubIssueRepository,
};

#[derive(Debug, Deserialize)]
pub struct GithubLabel {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubAssignee {
    pub login: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct GithubRepoApi {
    pub id: i64,
    pub full_name: String, // org/repo
}

pub fn derive_points(labels: &[GithubLabel]) -> i32 {
    for l in labels {
        let name = l.name.to_lowercase();
        if let Some(rest) = name.strip_prefix("points:") {
            if let Ok(v) = rest.trim().parse::<i32>() {
                return v.max(0);
            }
        }
    }
    0
}

pub fn transform_issue(repo_api: &GithubRepoApi, ia: &GithubIssueApi) -> Option<GithubIssue> {
    // Ignore PRs
    if ia.pull_request.is_some() {
        return None;
    }
    let points = derive_points(&ia.labels);

    let label_names: Vec<String> = ia
        .labels
        .iter()
        .map(|l| l.name.to_lowercase())
        .collect();
    let assignee_logins: Vec<String> = ia
        .assignees
        .iter()
        .map(|a| a.login.clone())
        .collect();

    Some(GithubIssue {
        repo: repo_api.full_name.clone(),
        repo_id: repo_api.id,
        github_issue_id: ia.id,
        number: ia.number,
        title: ia.title.clone(),
        state: ia.state.clone(),
        labels: serde_json::Value::from(label_names),
        points,
        assignee_logins: serde_json::Value::from(assignee_logins),
        html_url: ia.html_url.clone(),
        created_at: ia.created_at,
        closed_at: ia.closed_at,
        rewarded: false,
        distribution_id: None,
        updated_at: ia.updated_at,
    })
}

pub async fn sync_github_issues<T: GithubIssueRepository + ?Sized>(
    repo: &T,
    repos: &[String],
    since: Option<String>,
) -> Result<()> {
    let client = reqwest::Client::new();

    let mut all_issues: Vec<GithubIssue> = Vec::new();

    for repo_full in repos {
        // Get repository metadata to obtain repo_id
        let repo_api: GithubRepoApi = client
            .get(format!("https://api.github.com/repos/{}", repo_full))
            .header("User-Agent", "guild-backend")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        // Fetch issues (no pagination per scope). Filter by since if provided.
        let mut url = format!(
            "https://api.github.com/repos/{}/issues?state=all",
            repo_full
        );
        if let Some(s) = since.as_ref() {
            url.push_str(&format!("&since={}", s));
        }

        let issues_api: Vec<GithubIssueApi> = client
            .get(url)
            .header("User-Agent", "guild-backend")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        for ia in issues_api.iter() {
            if let Some(issue) = transform_issue(&repo_api, ia) {
                all_issues.push(issue);
            }
        }
    }

    repo.upsert_issues(&all_issues).await?;
    Ok(())
}