use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::badge::Badge;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBadge {
    pub id: Uuid,
    pub user_id: Uuid,
    pub badge_id: Uuid,
    pub awarded_at: chrono::DateTime<chrono::Utc>,
    pub awarded_by: String,
}

#[async_trait]
pub trait BadgeRepository: Send + Sync {
    async fn create(&self, badge: &Badge) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Badge>, Box<dyn std::error::Error>>;
    async fn find_all(&self) -> Result<Vec<Badge>, Box<dyn std::error::Error>>;
    async fn update(&self, badge: &Badge) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>>;
    
    // User badge operations
    async fn award_badge(&self, user_badge: &UserBadge) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_user_badges(&self, user_id: &Uuid) -> Result<Vec<UserBadge>, Box<dyn std::error::Error>>;
    async fn find_badge_users(&self, badge_id: &Uuid) -> Result<Vec<UserBadge>, Box<dyn std::error::Error>>;
}
