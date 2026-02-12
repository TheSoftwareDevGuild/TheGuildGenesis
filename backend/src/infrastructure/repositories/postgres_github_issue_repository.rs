use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::github_issue::GithubIssue,
    repositories::github_issue_repository::GithubIssueRepository,
};

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
    async fn upsert(&self, issue: &GithubIssue) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO github_issues (
                repo_id, github_issue_id, repo, issue_number, title, state,
                labels, points, assignee_logins, url,
                created_at, closed_at, rewarded_sepolia, distribution_id, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (repo_id, github_issue_id) DO UPDATE SET
                repo = EXCLUDED.repo,
                issue_number = EXCLUDED.issue_number,
                title = EXCLUDED.title,
                state = EXCLUDED.state,
                labels = EXCLUDED.labels,
                points = EXCLUDED.points,
                assignee_logins = EXCLUDED.assignee_logins,
                url = EXCLUDED.url,
                created_at = EXCLUDED.created_at,
                closed_at = EXCLUDED.closed_at,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(issue.repo_id)
        .bind(issue.github_issue_id)
        .bind(&issue.repo)
        .bind(issue.issue_number)
        .bind(&issue.title)
        .bind(&issue.state)
        .bind(&issue.labels)
        .bind(issue.points)
        .bind(&issue.assignee_logins)
        .bind(&issue.url)
        .bind(issue.created_at)
        .bind(issue.closed_at)
        .bind(issue.rewarded_sepolia)
        .bind(&issue.distribution_id)
        .bind(issue.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn find_by_key(
        &self,
        repo_id: i64,
        github_issue_id: i64,
    ) -> Result<Option<GithubIssue>, Box<dyn std::error::Error>> {
        let row = sqlx::query_as::<
            _,
            (
                i64,
                i64,
                String,
                i32,
                String,
                String,
                serde_json::Value,
                i32,
                serde_json::Value,
                String,
                chrono::DateTime<chrono::Utc>,
                Option<chrono::DateTime<chrono::Utc>>,
                bool,
                Option<String>,
                chrono::DateTime<chrono::Utc>,
            ),
        >(
            r#"
            SELECT repo_id, github_issue_id, repo, issue_number, title, state,
                   labels, points, assignee_logins, url,
                   created_at, closed_at, rewarded_sepolia, distribution_id, updated_at
            FROM github_issues
            WHERE repo_id = $1 AND github_issue_id = $2
            "#,
        )
        .bind(repo_id)
        .bind(github_issue_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.map(|r| GithubIssue {
            repo_id: r.0,
            github_issue_id: r.1,
            repo: r.2,
            issue_number: r.3,
            title: r.4,
            state: r.5,
            labels: r.6,
            points: r.7,
            assignee_logins: r.8,
            url: r.9,
            created_at: r.10,
            closed_at: r.11,
            rewarded_sepolia: r.12,
            distribution_id: r.13,
            updated_at: r.14,
        }))
    }
}
