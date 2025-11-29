use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;
use crate::domain::value_objects::WalletAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distribution {
    pub id: Uuid,
    pub distribution_id: Uuid,
    pub address: WalletAddress,
    pub badge_name: String,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

impl Distribution {
    pub fn new(distribution_id: Uuid, address: WalletAddress, badge_name: String, metadata: Option<Value>) -> Self {
        Self {
            id: Uuid::new_v4(),
            distribution_id,
            address,
            badge_name,
            metadata,
            created_at: Utc::now(),
        }
    }
}