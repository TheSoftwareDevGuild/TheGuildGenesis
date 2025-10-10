use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::value_objects::WalletAddress;

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
    async fn find_by_address(
        &self,
        address: &WalletAddress,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT address, name, description, avatar_url, github_login, created_at, updated_at
            FROM profiles
            WHERE address = $1
            "#,
            address.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.map(|r| Profile {
            address: WalletAddress(r.address),
            name: r.name,
            description: r.description,
            avatar_url: r.avatar_url,
            github_login: r.github_login,
            login_nonce: 0, // Not needed for regular profile queries
            created_at: r.created_at.unwrap(),
            updated_at: r.updated_at.unwrap(),
        }))
    }

    async fn find_all(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT address, name, description, avatar_url, github_login, created_at, updated_at
            FROM profiles
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(rows
            .into_iter()
            .map(|r| Profile {
                address: WalletAddress(r.address),
                name: r.name,
                description: r.description,
                avatar_url: r.avatar_url,
                github_login: r.github_login,
                login_nonce: 0, // Not needed for regular profile queries
                created_at: r.created_at.unwrap(),
                updated_at: r.updated_at.unwrap(),
            })
            .collect())
    }

    async fn create(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (address, name, description, avatar_url, github_login, login_nonce, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            profile.address.as_str(),
            profile.name,
            profile.description,
            profile.avatar_url,
            profile.github_login,
            profile.login_nonce,
            profile.created_at,
            profile.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn update(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE profiles
            SET name = $2, description = $3, avatar_url = $4, github_login = $5, updated_at = $6
            WHERE address = $1
            "#,
            profile.address.as_str(),
            profile.name,
            profile.description,
            profile.avatar_url,
            profile.github_login,
            profile.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn delete(&self, address: &WalletAddress) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM profiles
            WHERE address = $1
            "#,
            address.as_str()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }

    async fn find_by_github_login(
        &self,
        github_login: &str,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT address, name, description, avatar_url, github_login, created_at, updated_at
            FROM profiles
            WHERE LOWER(github_login) = LOWER($1)
            "#,
            github_login
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Ok(row.map(|r| Profile {
            address: WalletAddress(r.address),
            name: r.name,
            description: r.description,
            avatar_url: r.avatar_url,
            github_login: r.github_login,
            login_nonce: 0, // Not needed for regular profile queries
            created_at: r.created_at.unwrap(),
            updated_at: r.updated_at.unwrap(),
        }))
    }

    async fn get_login_nonce_by_wallet_address(
        &self,
        address: &WalletAddress,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT login_nonce
            FROM profiles
            WHERE address = $1
            "#,
            address.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(row.map(|r| r.login_nonce))
    }

    async fn increment_login_nonce(
        &self,
        address: &WalletAddress,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE profiles
            SET login_nonce = login_nonce + 1
            WHERE address = $1
            "#,
            address.as_str()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }
}
