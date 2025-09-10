use async_trait::async_trait;

use crate::domain::{entities::profile::Profile, value_objects::WalletAddress};

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn find_by_address(
        &self,
        address: &WalletAddress,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error>>;
    async fn create(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>>;
    async fn update(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, address: &WalletAddress) -> Result<(), Box<dyn std::error::Error>>;
}
