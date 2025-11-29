// src/application/commands/create_distribution.rs
use uuid::Uuid;
use anyhow::Result;
use crate::domain::repositories::distribution_repository::DistributionRepository;
use crate::domain::entities::distribution::Distribution;
use crate::domain::value_objects::WalletAddress;
use serde_json::Value;

pub struct CreateDistribution<'a> {
    pub repo: &'a dyn DistributionRepository,
}

impl<'a> CreateDistribution<'a> {
    pub fn new(repo: &'a dyn DistributionRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        items: Vec<(String, String, Option<Value>)>,
        _batch_metadata: Option<Value>,
    ) -> Result<Uuid> {
        let distribution_id = Uuid::new_v4();
        let mut domain_items = Vec::with_capacity(items.len());

        for (address_str, badge_name, metadata) in items {
            let wallet_address = WalletAddress::new(address_str)
                .map_err(|e| anyhow::anyhow!(e))?;
            let item = Distribution::new(distribution_id, wallet_address, badge_name, metadata);
            domain_items.push(item);
        }

        self.repo.insert_batch(distribution_id, domain_items).await?;

        Ok(distribution_id)
    }
}