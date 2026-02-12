use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::projects::{Project, ProjectId, ProjectStatus},
    repositories::project_repository::ProjectRepository,
    value_objects::WalletAddress,
};

#[derive(Clone)]
pub struct PostgresProjectRepository {
    pool: PgPool,
}

impl PostgresProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    async fn create(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO projects (id, name, description, status, creator, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            project.id.value(),
            project.name,
            project.description,
            project.status.as_str(),
            project.creator.as_str(),
            project.created_at,
            project.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ProjectId,
    ) -> Result<Option<Project>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, status, creator, created_at, updated_at
            FROM projects
            WHERE id = $1
            "#,
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.map(|r| {
            let status = r.status.parse().unwrap_or(ProjectStatus::Proposal);
            Project {
                id: ProjectId::from_uuid(r.id),
                name: r.name,
                description: r.description,
                status,
                creator: WalletAddress(r.creator),
                created_at: r.created_at,
                updated_at: r.updated_at,
            }
        }))
    }

    async fn find_all(
        &self,
        status: Option<ProjectStatus>,
        creator: Option<&WalletAddress>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        // Build query dynamically based on filters
        let mut query = String::from(
            "SELECT id, name, description, status, creator, created_at, updated_at FROM projects WHERE 1=1"
        );

        if status.is_some() {
            query.push_str(" AND status = $1");
        }

        if creator.is_some() {
            if status.is_some() {
                query.push_str(" AND creator = $2");
            } else {
                query.push_str(" AND creator = $1");
            }
        }

        query.push_str(" ORDER BY created_at DESC");

        if limit.is_some() {
            let param_num = match (status.is_some(), creator.is_some()) {
                (true, true) => 3,
                (true, false) | (false, true) => 2,
                (false, false) => 1,
            };
            query.push_str(&format!(" LIMIT ${}", param_num));
        }

        if offset.is_some() {
            let param_num = match (status.is_some(), creator.is_some(), limit.is_some()) {
                (true, true, true) => 4,
                (true, true, false) => 3,
                (true, false, true) | (false, true, true) => 3,
                (true, false, false) | (false, true, false) | (false, false, true) => 2,
                (false, false, false) => 1,
            };
            query.push_str(&format!(" OFFSET ${}", param_num));
        }

        let mut query_builder = sqlx::query_as::<
            _,
            (
                sqlx::types::Uuid,
                String,
                String,
                String,
                String,
                Option<chrono::DateTime<chrono::Utc>>,
                Option<chrono::DateTime<chrono::Utc>>,
            ),
        >(&query);

        if let Some(s) = status {
            query_builder = query_builder.bind(s.as_str());
        }

        if let Some(c) = creator {
            query_builder = query_builder.bind(c.as_str());
        }

        if let Some(l) = limit {
            query_builder = query_builder.bind(l);
        }

        if let Some(o) = offset {
            query_builder = query_builder.bind(o);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(rows
            .into_iter()
            .map(
                |(id, name, description, status, creator, created_at, updated_at)| {
                    let status = status.parse().unwrap_or(ProjectStatus::Proposal);
                    Project {
                        id: ProjectId::from_uuid(id),
                        name,
                        description,
                        status,
                        creator: WalletAddress(creator),
                        created_at: created_at.unwrap(),
                        updated_at: updated_at.unwrap(),
                    }
                },
            )
            .collect())
    }

    async fn find_by_creator(
        &self,
        creator: &WalletAddress,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, status, creator, created_at, updated_at
            FROM projects
            WHERE creator = $1
            ORDER BY created_at DESC
            "#,
            creator.as_str()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let status = r.status.parse().unwrap_or(ProjectStatus::Proposal);
                Project {
                    id: ProjectId::from_uuid(r.id),
                    name: r.name,
                    description: r.description,
                    status,
                    creator: WalletAddress(r.creator),
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }
            })
            .collect())
    }

    async fn update(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE projects
            SET name = $2, description = $3, status = $4, updated_at = $5
            WHERE id = $1
            "#,
            project.id.value(),
            project.name,
            project.description,
            project.status.as_str(),
            project.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn delete(&self, id: &ProjectId) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            "#,
            id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn exists(&self, id: &ProjectId) -> Result<bool, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1) as "exists!"
            "#,
            id.value()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.exists)
    }

    async fn profile_exists(
        &self,
        address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM profiles WHERE address = $1) as "exists!"
            "#,
            address.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.exists)
    }
}
