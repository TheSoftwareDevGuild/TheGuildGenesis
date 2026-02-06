use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        project_like::ProjectLike,
        projects::ProjectId,
    },
    repositories::project_like_repository::ProjectLikeRepository,
    value_objects::WalletAddress,
};

#[derive(Clone)]
pub struct PostgresProjectLikeRepository {
    pool: PgPool,
}

impl PostgresProjectLikeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectLikeRepository for PostgresProjectLikeRepository {
    async fn create(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let result = sqlx::query(
            r#"
            INSERT INTO project_likes (project_id, user_address)
            VALUES ($1, $2)
            ON CONFLICT (project_id, user_address) DO NOTHING
            "#,
        )
        .bind(project_id.value())
        .bind(user_address.as_str())
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(result.rows_affected() == 1)
    }

    async fn delete(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let result = sqlx::query(
            r#"
            DELETE FROM project_likes
            WHERE project_id = $1 AND user_address = $2
            "#,
        )
        .bind(project_id.value())
        .bind(user_address.as_str())
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(result.rows_affected() == 1)
    }

    async fn list_by_project(
        &self,
        project_id: &ProjectId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProjectLike>, Box<dyn std::error::Error>> {
        let rows = sqlx::query_as::<_, (Uuid, String, DateTime<Utc>)>(
            r#"
            SELECT project_id, user_address, created_at
            FROM project_likes
            WHERE project_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(project_id.value())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(rows
            .into_iter()
            .map(|(project_id, user_address, created_at)| ProjectLike {
                project_id: ProjectId::from_uuid(project_id),
                user_address: WalletAddress(user_address),
                created_at,
            })
            .collect())
    }

    async fn count_by_project(
        &self,
        project_id: &ProjectId,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM project_likes
            WHERE project_id = $1
            "#,
        )
        .bind(project_id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(count)
    }

    async fn exists(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM project_likes WHERE project_id = $1 AND user_address = $2
            )
            "#,
        )
        .bind(project_id.value())
        .bind(user_address.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(exists)
    }
}
