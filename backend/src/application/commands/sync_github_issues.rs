use std::sync::Arc;

use regex::Regex;

use crate::domain::{
    entities::github_issue::GithubIssue,
    repositories::github_issue_repository::GithubIssueRepository,
    services::github_service::{GitHubApiIssue, GithubService},
};

/// Derive points from labels matching the pattern `points:N`.
/// Label names are normalized to lower-case.
pub fn derive_points(labels: &[crate::domain::services::github_service::GitHubApiLabel]) -> i32 {
    let re = Regex::new(r"^points:(\d+)$").expect("Invalid regex");
    for label in labels {
        let name = label.name.to_lowercase();
        if let Some(caps) = re.captures(&name) {
            if let Ok(pts) = caps[1].parse::<i32>() {
                return pts;
            }
        }
    }
    0
}

/// Transform a GitHub API issue into a domain GithubIssue entity.
pub fn transform_issue(
    repo: &str,
    repo_id: i64,
    api_issue: &GitHubApiIssue,
) -> Result<GithubIssue, String> {
    let labels_normalized: Vec<serde_json::Value> = api_issue
        .labels
        .iter()
        .map(|l| serde_json::Value::String(l.name.to_lowercase()))
        .collect();

    let assignee_logins: Vec<serde_json::Value> = api_issue
        .assignees
        .iter()
        .map(|a| serde_json::Value::String(a.login.clone()))
        .collect();

    let points = derive_points(&api_issue.labels);

    let created_at = chrono::DateTime::parse_from_rfc3339(&api_issue.created_at)
        .map_err(|e| format!("Invalid created_at: {}", e))?
        .with_timezone(&chrono::Utc);

    let closed_at = api_issue
        .closed_at
        .as_ref()
        .map(|s| chrono::DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&chrono::Utc)))
        .transpose()
        .map_err(|e| format!("Invalid closed_at: {}", e))?;

    let updated_at = chrono::DateTime::parse_from_rfc3339(&api_issue.updated_at)
        .map_err(|e| format!("Invalid updated_at: {}", e))?
        .with_timezone(&chrono::Utc);

    Ok(GithubIssue {
        repo_id,
        github_issue_id: api_issue.id,
        repo: repo.to_string(),
        number: api_issue.number,
        title: api_issue.title.clone(),
        state: api_issue.state.clone(),
        labels: serde_json::Value::Array(labels_normalized),
        points,
        assignee_logins: serde_json::Value::Array(assignee_logins),
        html_url: api_issue.html_url.clone(),
        created_at,
        closed_at,
        rewarded: false,
        distribution_id: None,
        updated_at,
    })
}

/// Sync GitHub issues for the given repos.
pub async fn sync_github_issues(
    github_service: Arc<dyn GithubService>,
    issue_repository: Arc<dyn GithubIssueRepository>,
    repos: Vec<String>,
    since: Option<String>,
) -> Result<usize, String> {
    let mut total_synced: usize = 0;

    for repo in &repos {
        let (repo_id, api_issues) = github_service
            .fetch_issues(repo, since.as_deref())
            .await
            .map_err(|e| format!("Failed to fetch issues for {}: {}", repo, e))?;

        for api_issue in &api_issues {
            // Ignore PRs
            if api_issue.pull_request.is_some() {
                continue;
            }

            let issue = transform_issue(repo, repo_id, api_issue)?;

            issue_repository
                .upsert(&issue)
                .await
                .map_err(|e| format!("Failed to upsert issue {}: {}", api_issue.id, e))?;

            total_synced += 1;
        }
    }

    Ok(total_synced)
}
