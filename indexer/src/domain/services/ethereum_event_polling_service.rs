use async_trait::async_trait;
use std::error::Error;

use crate::domain::entities::ethereum_event::EthereumEvent;

#[async_trait]
pub trait EthereumEventPollingService: Send + Sync {
    async fn poll(&self) -> Result<Vec<EthereumEvent>, Box<dyn Error>>;
}
