use std::sync::Arc;

use crate::{
    application::dtos::distribution_dtos::DistributionResponse,
    domain::repositories::distribution_repository::DistributionRepository,
};

pub async fn list_distributions(
    repository: Arc<dyn DistributionRepository>,
    distribution_id: Option<String>,
) -> Result<Vec<DistributionResponse>, String> {
    repository
        .list(distribution_id.as_deref())
        .await
        .map(|records| {
            records
                .into_iter()
                .map(|record| DistributionResponse {
                    address: record.address,
                    badge_name: record.badge_name,
                    distribution_id: record.distribution_id,
                })
                .collect()
        })
        .map_err(|e| e.to_string())
}
