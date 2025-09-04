use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn find_by_wallet_address(&self, address: &str) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn update(&self, user: &User) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>>;
}
