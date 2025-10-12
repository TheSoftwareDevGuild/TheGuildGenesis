use std::error::Error;

use async_trait::async_trait;

use crate::domain::entities::ethereum_event::EthereumEvent;

#[async_trait]
pub trait EthereumEventRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<EthereumEvent>, Box<dyn Error>>;
    async fn insert_many(&self, ethereum_events: Vec<EthereumEvent>) -> Result<(), Box<dyn Error>>;
}
