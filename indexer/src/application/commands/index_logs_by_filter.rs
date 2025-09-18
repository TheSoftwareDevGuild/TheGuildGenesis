use eyre::Result;

use crate::{
    application::dtos::indexing_dtos::{LogFilterRequest, LogsResponse},
    domain::{
        entities::LogFilter,
        repositories::EthereumLogRepository,
        services::IndexingService,
    },
};

pub struct IndexLogsByFilterCommand {
    indexing_service: Box<dyn IndexingService>,
    log_repository: Box<dyn EthereumLogRepository>,
}

impl IndexLogsByFilterCommand {
    pub fn new(
        indexing_service: Box<dyn IndexingService>,
        log_repository: Box<dyn EthereumLogRepository>,
    ) -> Self {
        Self {
            indexing_service,
            log_repository,
        }
    }

    pub async fn execute(&self, request: LogFilterRequest) -> Result<LogsResponse> {
        let filter = LogFilter {
            from_block: request.from_block,
            to_block: request.to_block,
            address: request.address,
            topics: request.topics,
            event_signature: request.event_signature,
        };

        // Index logs using the service
        let indexed_count = self.indexing_service.index_logs_by_filter(
            &request.rpc_url,
            &filter,
        ).await?;

        // Get the indexed logs
        let logs = self.log_repository.get_logs_by_filter(&filter).await?;

        Ok(LogsResponse {
            logs,
            total_count: indexed_count as usize,
            success: true,
        })
    }
}
