use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::badge::{Badge, UserBadge};
use crate::domain::repositories::badge_repository::BadgeRepository;

#[derive(Clone)]
pub struct PostgresBadgeRepository {
    pool: PgPool,
}

impl PostgresBadgeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BadgeRepository for PostgresBadgeRepository {
    async fn create(&self, badge: &Badge) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO badges (id, name, description, issuer_address, image_url, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            badge.id,
            badge.name,
            badge.description,
            badge.issuer_address,
            badge.image_url,
            badge.created_at,
            badge.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Badge>, Box<dyn std::error::Error>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, issuer_address, image_url, created_at, updated_at
            FROM badges
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(row.map(|r| Badge {
            id: r.id,
            name: r.name,
            description: r.description,
            issuer_address: r.issuer_address,
            image_url: r.image_url,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }
    
    async fn find_all(&self) -> Result<Vec<Badge>, Box<dyn std::error::Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, issuer_address, image_url, created_at, updated_at
            FROM badges
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let badges = rows
            .into_iter()
            .map(|r| Badge {
                id: r.id,
                name: r.name,
                description: r.description,
                issuer_address: r.issuer_address,
                image_url: r.image_url,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect();
        
        Ok(badges)
    }
    
    async fn update(&self, badge: &Badge) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE badges
            SET name = $2, description = $3, issuer_address = $4, image_url = $5, updated_at = $6
            WHERE id = $1
            "#,
            badge.id,
            badge.name,
            badge.description,
            badge.issuer_address,
            badge.image_url,
            badge.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(())
    }
    
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            DELETE FROM badges
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(())
    }
    
    async fn find_badge_users(&self, badge_id: &Uuid) -> Result<Vec<crate::domain::repositories::badge_repository::UserBadge>, Box<dyn std::error::Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT ub.id, ub.user_id, ub.badge_id, ub.awarded_at, ub.awarded_by
            FROM user_badges ub
            WHERE ub.badge_id = $1
            ORDER BY ub.awarded_at DESC
            "#,
            badge_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let user_badges = rows
            .into_iter()
            .map(|r| crate::domain::repositories::badge_repository::UserBadge {
                id: r.id,
                user_id: r.user_id,
                badge_id: r.badge_id,
                awarded_at: r.awarded_at,
                awarded_by: r.awarded_by,
            })
            .collect();
        
        Ok(user_badges)
    }
    
    async fn award_badge(&self, user_badge: &crate::domain::repositories::badge_repository::UserBadge) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO user_badges (id, user_id, badge_id, awarded_at, awarded_by)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user_badge.id,
            user_badge.user_id,
            user_badge.badge_id,
            user_badge.awarded_at,
            user_badge.awarded_by
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(())
    }
    
    async fn find_user_badges(&self, user_id: &Uuid) -> Result<Vec<crate::domain::repositories::badge_repository::UserBadge>, Box<dyn std::error::Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT ub.id, ub.user_id, ub.badge_id, ub.awarded_at, ub.awarded_by
            FROM user_badges ub
            WHERE ub.user_id = $1
            ORDER BY ub.awarded_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let user_badges = rows
            .into_iter()
            .map(|r| crate::domain::repositories::badge_repository::UserBadge {
                id: r.id,
                user_id: r.user_id,
                badge_id: r.badge_id,
                awarded_at: r.awarded_at,
                awarded_by: r.awarded_by,
            })
            .collect();
        
        Ok(user_badges)
    }
}
