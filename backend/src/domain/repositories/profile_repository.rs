use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::profile::Profile;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Profile>, Box<dyn std::error::Error>>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Option<Profile>, Box<dyn std::error::Error>>;
    async fn find_all(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>>;
    async fn update(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>>;
}
