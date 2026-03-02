use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct DistributionRecord {
    pub address: String,
    pub badge_name: String,
    pub distribution_id: String,
}

#[async_trait]
pub trait DistributionRepository: Send + Sync {
    async fn create_many(
        &self,
        records: &[DistributionRecord],
    ) -> Result<(), Box<dyn std::error::Error>>;
}
