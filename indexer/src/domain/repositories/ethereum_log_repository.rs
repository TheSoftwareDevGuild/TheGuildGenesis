use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{EthereumLog, LogFilter};

#[async_trait]
pub trait EthereumLogRepository: Send + Sync {
    async fn create_log(&self, log: &EthereumLog) -> Result<Uuid, Box<dyn std::error::Error>>;
    async fn create_logs_batch(&self, logs: &[EthereumLog]) -> Result<Vec<Uuid>, Box<dyn std::error::Error>>;
    async fn get_logs_by_filter(&self, filter: &LogFilter) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>>;
    async fn get_logs_by_address(&self, address: &str, limit: Option<i64>) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>>;
    async fn get_logs_by_block_range(&self, from_block: u64, to_block: u64) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>>;
    async fn get_latest_logs(&self, limit: i64) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait IndexingProgressRepository: Send + Sync {
    async fn get_progress(&self, chain_id: i32) -> Result<Option<crate::domain::entities::IndexingProgress>, Box<dyn std::error::Error>>;
    async fn update_progress(&self, chain_id: i32, last_indexed_block: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn create_progress(&self, chain_id: i32, last_indexed_block: i64, rpc_url: &str) -> Result<Uuid, Box<dyn std::error::Error>>;
    async fn set_error(&self, chain_id: i32, error_message: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn clear_error(&self, chain_id: i32) -> Result<(), Box<dyn std::error::Error>>;
}