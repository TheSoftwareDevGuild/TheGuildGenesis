use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::badge::{Badge, UserBadge};

#[async_trait]
pub trait BadgeRepository {
    async fn create_badge(&self, badge: Badge) -> Result<Badge, String>;
    async fn find_badge_by_id(&self, id: Uuid) -> Result<Option<Badge>, String>;
    async fn find_all_badges(&self) -> Result<Vec<Badge>, String>;
    async fn update_badge(&self, badge: Badge) -> Result<Badge, String>;
    async fn delete_badge(&self, id: Uuid) -> Result<(), String>;
    
    async fn award_badge(&self, user_badge: UserBadge) -> Result<UserBadge, String>;
    async fn find_user_badges(&self, user_id: Uuid) -> Result<Vec<UserBadge>, String>;
    async fn find_badge_holders(&self, badge_id: Uuid) -> Result<Vec<UserBadge>, String>;
}
