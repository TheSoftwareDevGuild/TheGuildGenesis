use std::sync::Arc;

use crate::{
    application::dtos::distribution_dtos::RegisterDistributionRequest,
    domain::repositories::distribution_repository::{DistributionRecord, DistributionRepository},
};

pub async fn register_distribution(
    repository: Arc<dyn DistributionRepository>,
    request: RegisterDistributionRequest,
) -> Result<(), String> {
    if request.distributions.is_empty() {
        return Err("distributions must not be empty".to_string());
    }

    let records: Vec<DistributionRecord> = request
        .distributions
        .into_iter()
        .map(|item| DistributionRecord {
            address: item.address,
            badge_name: item.badge_name,
            distribution_id: item.distribution_id,
        })
        .collect();

    repository
        .create_many(&records)
        .await
        .map_err(|e| e.to_string())
}
