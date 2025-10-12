use std::{error::Error, sync::Arc};

use crate::domain::{
    entities::ethereum_event::EthereumEvent,
    repositories::ethereum_event_repository::EthereumEventRepository,
};

pub async fn list_events(
    ethereum_event_repository: Arc<dyn EthereumEventRepository>,
) -> Result<Vec<EthereumEvent>, Box<dyn Error>> {
    ethereum_event_repository.list().await
}
