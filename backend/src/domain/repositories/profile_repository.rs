use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::profile::Profile;

#[async_trait]
pub trait ProfileRepository {
    async fn create(&self, profile: Profile) -> Result<Profile, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Profile>, String>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Profile>, String>;
    async fn find_all(&self) -> Result<Vec<Profile>, String>;
    async fn update(&self, profile: Profile) -> Result<Profile, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
