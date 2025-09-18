use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingRequest {
    pub rpc_url: String,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub chain_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingResponse {
    pub indexed_count: u64,
    pub from_block: u64,
    pub to_block: u64,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilterRequest {
    pub rpc_url: String,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub address: Option<String>,
    pub topics: Option<Vec<String>>,
    pub event_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsResponse {
    pub logs: Vec<crate::domain::entities::EthereumLog>,
    pub total_count: usize,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingStatusResponse {
    pub chain_id: i32,
    pub last_indexed_block: i64,
    pub status: String,
    pub error_message: Option<String>,
    pub success: bool,
}
