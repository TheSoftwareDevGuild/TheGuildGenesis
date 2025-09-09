use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;

#[derive(Clone)]
pub struct PostgresProfileRepository {
    pool: PgPool,
}

impl PostgresProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepository for PostgresProfileRepository {
    async fn create(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (id, user_id, name, description, avatar_url, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            profile.id,
            profile.user_id,
            profile.name,
            profile.description,
            profile.avatar_url,
            profile.created_at,
            profile.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, name, description, avatar_url, created_at, updated_at
            FROM profiles
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.map(|r| Profile {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            description: r.description,
            avatar_url: r.avatar_url,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, name, description, avatar_url, created_at, updated_at
            FROM profiles
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.map(|r| Profile {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            description: r.description,
            avatar_url: r.avatar_url,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_all(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, name, description, avatar_url, created_at, updated_at
            FROM profiles
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let profiles = rows
            .into_iter()
            .map(|r| Profile {
                id: r.id,
                user_id: r.user_id,
                name: r.name,
                description: r.description,
                avatar_url: r.avatar_url,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect();

        Ok(profiles)
    }

    async fn update(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE profiles
            SET name = $2, description = $3, avatar_url = $4, updated_at = $5
            WHERE id = $1
            "#,
            profile.id,
            profile.name,
            profile.description,
            profile.avatar_url,
            profile.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM profiles
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }
}
