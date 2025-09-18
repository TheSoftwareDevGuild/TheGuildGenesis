use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EthereumLog {
    pub id: Uuid,
    pub block_number: i64,
    pub block_hash: String,
    pub transaction_hash: String,
    pub transaction_index: i32,
    pub log_index: i32,
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
    pub removed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IndexingProgress {
    pub id: Uuid,
    pub chain_id: i32,
    pub last_indexed_block: i64,
    pub rpc_url: String,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub address: Option<String>,
    pub topics: Option<Vec<String>>,
    pub event_signature: Option<String>,
}