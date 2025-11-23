use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::domain::entities::distribution::Distribution;

#[async_trait]
pub trait DistributionRepository: Send + Sync {
    async fn insert_batch(&self, distribution_id: Uuid, items: Vec<Distribution>) -> Result<()>;

    async fn get_by_distribution_id(&self, distribution_id: Uuid) -> Result<Vec<Distribution>>;

    async fn get_by_address(&self, address: &str) -> Result<Vec<Distribution>>;
}