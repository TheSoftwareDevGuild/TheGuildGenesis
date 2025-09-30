use std::{error::Error, sync::Arc};

use crate::domain::{
    repositories::ethereum_event_repository::EthereumEventRepository,
    services::ethereum_event_polling_service::EthereumEventPollingService,
};

pub async fn poll(
    ethereum_event_polling_service: Arc<dyn EthereumEventPollingService>,
    ethereum_event_repository: Arc<dyn EthereumEventRepository>,
) -> Result<(), Box<dyn Error>> {
    let poll_result = ethereum_event_polling_service.poll().await?;
    ethereum_event_repository.insert_many(poll_result).await
}
