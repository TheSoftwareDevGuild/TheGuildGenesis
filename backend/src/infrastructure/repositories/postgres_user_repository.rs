use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::value_objects::wallet_address::WalletAddress;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO users (id, wallet_address, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(&user.id)
        .bind(&user.wallet_address)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, wallet_address, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(row.map(|r| User {
            id: r.id,
            wallet_address: r.wallet_address,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }
    
    async fn find_by_wallet_address(&self, address: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, wallet_address, created_at, updated_at
            FROM users
            WHERE wallet_address = $1
            "#,
            address
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(row.map(|r| User {
            id: r.id,
            wallet_address: r.wallet_address,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }
    
    async fn update(&self, user: &User) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE users
            SET wallet_address = $2, updated_at = $3
            WHERE id = $1
            "#,
            user.id,
            user.wallet_address,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(())
    }
    
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM users
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
