use anyhow::Result;
// chrono imports not needed in this module after refactor

use crate::domain::{
    entities::github_issue::GithubIssue,
    repositories::GithubIssueRepository,
    services::github_api_service::{GithubApiService, GithubIssueApi, GithubLabel, GithubRepoApi},
};

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

    let label_names: Vec<String> = ia.labels.iter().map(|l| l.name.to_lowercase()).collect();
    let assignee_logins: Vec<String> = ia.assignees.iter().map(|a| a.login.clone()).collect();

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

pub async fn sync_github_issues<R: GithubIssueRepository + ?Sized, A: GithubApiService + ?Sized>(
    repo: &R,
    api: &A,
    repos: &[String],
    since: Option<String>,
) -> Result<()> {
    let mut all_issues: Vec<GithubIssue> = Vec::new();

    for repo_full in repos {
        let repo_api = api.get_repo(repo_full).await?;
        let issues_api = api.list_issues(repo_full, since.as_deref()).await?;

        for ia in issues_api.iter() {
            if let Some(issue) = transform_issue(&repo_api, ia) {
                all_issues.push(issue);
            }
        }
    }

    repo.upsert_issues(&all_issues).await?;
    Ok(())
}
