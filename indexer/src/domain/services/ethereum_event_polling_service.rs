use async_trait::async_trait;
use std::error::Error;

use crate::domain::entities::ethereum_event::EthereumEvent;

#[async_trait]
pub trait EthereumEventPollingService {
    async fn poll() -> Result<Vec<EthereumEvent>, Box<dyn Error>>;
}
