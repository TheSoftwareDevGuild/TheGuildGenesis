use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::github_issue::GithubIssue;
use crate::domain::repositories::github_issue_repository::GithubIssueRepository;

#[derive(Clone)]
pub struct PostgresGithubIssueRepository {
    pool: PgPool,
}

impl PostgresGithubIssueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl GithubIssueRepository for PostgresGithubIssueRepository {
    async fn upsert_issues(&self, issues: &[GithubIssue]) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        for issue in issues {
            sqlx::query(
                r#"
                INSERT INTO github_issues (
                    repo, repo_id, github_issue_id, number, title, state, labels, points,
                    assignee_logins, html_url, created_at, closed_at, rewarded, distribution_id, updated_at
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8,
                    $9, $10, $11, $12, $13, $14, $15
                )
                ON CONFLICT (repo_id, github_issue_id) DO UPDATE SET
                    repo = EXCLUDED.repo,
                    number = EXCLUDED.number,
                    title = EXCLUDED.title,
                    state = EXCLUDED.state,
                    labels = EXCLUDED.labels,
                    points = EXCLUDED.points,
                    assignee_logins = EXCLUDED.assignee_logins,
                    html_url = EXCLUDED.html_url,
                    created_at = EXCLUDED.created_at,
                    closed_at = EXCLUDED.closed_at,
                    rewarded = github_issues.rewarded,
                    distribution_id = COALESCE(EXCLUDED.distribution_id, github_issues.distribution_id),
                    updated_at = EXCLUDED.updated_at
                "#,
            )
            .bind(&issue.repo)
            .bind(issue.repo_id)
            .bind(issue.github_issue_id)
            .bind(issue.number)
            .bind(&issue.title)
            .bind(&issue.state)
            .bind(&issue.labels)
            .bind(issue.points)
            .bind(&issue.assignee_logins)
            .bind(&issue.html_url)
            .bind(issue.created_at)
            .bind(issue.closed_at)
            .bind(issue.rewarded)
            .bind(issue.distribution_id.as_deref())
            .bind(issue.updated_at)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
