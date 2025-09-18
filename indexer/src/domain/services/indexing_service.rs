use alloy::{
    primitives::{address, b256},
    providers::{Provider, ProviderBuilder},
    rpc::types::Filter,
};
use async_trait::async_trait;
use eyre::Result;

use crate::domain::{
    entities::{EthereumLog, LogFilter},
    repositories::{EthereumLogRepository, IndexingProgressRepository},
};

#[async_trait]
pub trait IndexingService: Send + Sync {
    async fn index_logs(&self, rpc_url: &str, from_block: Option<u64>, to_block: Option<u64>) -> Result<u64>;
    async fn index_logs_by_filter(&self, rpc_url: &str, filter: &LogFilter) -> Result<u64>;
    async fn get_latest_block(&self, rpc_url: &str) -> Result<u64>;
}

pub struct EthereumIndexingService {
    log_repository: Box<dyn EthereumLogRepository>,
    progress_repository: Box<dyn IndexingProgressRepository>,
}

impl EthereumIndexingService {
    pub fn new(
        log_repository: Box<dyn EthereumLogRepository>,
        progress_repository: Box<dyn IndexingProgressRepository>,
    ) -> Self {
        Self {
            log_repository,
            progress_repository,
        }
    }

    async fn create_provider(rpc_url: &str) -> Result<impl Provider> {
        let rpc_url = rpc_url.parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);
        Ok(provider)
    }

    async fn convert_alloy_log_to_entity(
        log: &alloy::rpc::types::Log,
        block_number: u64,
        block_hash: &str,
    ) -> Result<EthereumLog> {
        use chrono::Utc;
        use uuid::Uuid;

        let topics: Vec<String> = log.topics.iter().map(|t| format!("{:#x}", t)).collect();

        Ok(EthereumLog {
            id: Uuid::new_v4(),
            block_number: block_number as i64,
            block_hash: block_hash.to_string(),
            transaction_hash: format!("{:#x}", log.transaction_hash),
            transaction_index: log.transaction_index as i32,
            log_index: log.log_index as i32,
            address: format!("{:#x}", log.address),
            data: format!("{:#x}", log.data),
            topics,
            removed: log.removed.unwrap_or(false),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[async_trait]
impl IndexingService for EthereumIndexingService {
    async fn index_logs(&self, rpc_url: &str, from_block: Option<u64>, to_block: Option<u64>) -> Result<u64> {
        let provider = Self::create_provider(rpc_url).await?;
        
        // Get latest block if not specified
        let from_block = match from_block {
            Some(block) => block,
            None => {
                let latest = provider.get_block_number().await?;
                latest
            }
        };

        let to_block = to_block.unwrap_or(from_block);

        // Create filter for the block range
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block);

        // Get logs
        let logs = provider.get_logs(&filter).await?;
        
        if logs.is_empty() {
            return Ok(0);
        }

        // Get block hashes for the logs
        let mut ethereum_logs = Vec::new();
        for log in logs {
            let block_hash = provider.get_block(log.block_number.unwrap(), false).await?;
            let block_hash_str = format!("{:#x}", block_hash.unwrap().hash.unwrap());
            
            let ethereum_log = Self::convert_alloy_log_to_entity(
                &log,
                log.block_number.unwrap(),
                &block_hash_str,
            ).await?;
            
            ethereum_logs.push(ethereum_log);
        }

        // Batch insert logs
        let _inserted_ids = self.log_repository.create_logs_batch(&ethereum_logs).await?;
        
        Ok(ethereum_logs.len() as u64)
    }

    async fn index_logs_by_filter(&self, rpc_url: &str, filter: &LogFilter) -> Result<u64> {
        let provider = Self::create_provider(rpc_url).await?;
        
        let mut alloy_filter = Filter::new();
        
        if let Some(from_block) = filter.from_block {
            alloy_filter = alloy_filter.from_block(from_block);
        }
        
        if let Some(to_block) = filter.to_block {
            alloy_filter = alloy_filter.to_block(to_block);
        }
        
        if let Some(address) = &filter.address {
            let addr = address.parse::<alloy::primitives::Address>()?;
            alloy_filter = alloy_filter.address(addr);
        }
        
        if let Some(event_signature) = &filter.event_signature {
            let signature = b256!(event_signature.as_str());
            alloy_filter = alloy_filter.event_signature(signature);
        }
        
        if let Some(topics) = &filter.topics {
            for topic in topics {
                let topic_hash = b256!(topic.as_str());
                alloy_filter = alloy_filter.topic0(topic_hash);
            }
        }

        // Get logs
        let logs = provider.get_logs(&alloy_filter).await?;
        
        if logs.is_empty() {
            return Ok(0);
        }

        // Convert and store logs
        let mut ethereum_logs = Vec::new();
        for log in logs {
            let block_hash = provider.get_block(log.block_number.unwrap(), false).await?;
            let block_hash_str = format!("{:#x}", block_hash.unwrap().hash.unwrap());
            
            let ethereum_log = Self::convert_alloy_log_to_entity(
                &log,
                log.block_number.unwrap(),
                &block_hash_str,
            ).await?;
            
            ethereum_logs.push(ethereum_log);
        }

        // Batch insert logs
        let _inserted_ids = self.log_repository.create_logs_batch(&ethereum_logs).await?;
        
        Ok(ethereum_logs.len() as u64)
    }

    async fn get_latest_block(&self, rpc_url: &str) -> Result<u64> {
        let provider = Self::create_provider(rpc_url).await?;
        let latest_block = provider.get_block_number().await?;
        Ok(latest_block)
    }
}