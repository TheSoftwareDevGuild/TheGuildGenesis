use std::error::Error;

use crate::domain::entities::ethereum_event::EthereumEvent;

pub async fn poll() -> Result<Vec<EthereumEvent>, Box<dyn Error>> {
    Ok(vec![EthereumEvent::new("TEST".to_string())])
}
