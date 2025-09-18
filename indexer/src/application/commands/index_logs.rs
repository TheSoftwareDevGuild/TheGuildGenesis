use eyre::Result;

use crate::{
    application::dtos::indexing_dtos::{IndexingRequest, IndexingResponse},
    domain::{
        entities::LogFilter,
        repositories::{EthereumLogRepository, IndexingProgressRepository},
        services::IndexingService,
    },
};

pub struct IndexLogsCommand {
    indexing_service: Box<dyn IndexingService>,
    progress_repository: Box<dyn IndexingProgressRepository>,
}

impl IndexLogsCommand {
    pub fn new(
        indexing_service: Box<dyn IndexingService>,
        progress_repository: Box<dyn IndexingProgressRepository>,
    ) -> Self {
        Self {
            indexing_service,
            progress_repository,
        }
    }

    pub async fn execute(&self, request: IndexingRequest) -> Result<IndexingResponse> {
        let from_block = request.from_block;
        let to_block = request.to_block;

        // Index logs using the service
        let indexed_count = self.indexing_service.index_logs(
            &request.rpc_url,
            from_block,
            to_block,
        ).await?;

        // Update progress if chain_id is provided
        if let Some(chain_id) = request.chain_id {
            if let Some(to_block) = to_block {
                self.progress_repository.update_progress(chain_id, to_block as i64).await?;
            }
        }

        Ok(IndexingResponse {
            indexed_count,
            from_block: from_block.unwrap_or(0),
            to_block: to_block.unwrap_or(0),
            success: true,
            message: format!("Successfully indexed {} logs", indexed_count),
        })
    }
}
