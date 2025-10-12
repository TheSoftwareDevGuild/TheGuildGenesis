use alloy::primitives::{Address, U256};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthereumEventType {
    ActivityTokenMinted { recipient: Address, amount: U256 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumEvent {
    pub id: String,
    pub event_type: EthereumEventType,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl EthereumEvent {
    fn new(
        id: String,
        event_type: EthereumEventType,
        timestamp: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            event_type,
            timestamp,
            created_at,
        }
    }

    pub fn new_activity_token_minted_from_transfer_data(
        tx_hash: String,
        log_index: String,
        recipient: Address,
        amount: U256,
    ) -> Self {
        let now = Utc::now();
        Self::new(
            format!("{tx_hash}-{log_index}"),
            EthereumEventType::ActivityTokenMinted { recipient, amount },
            now,
            now,
        )
    }
}
