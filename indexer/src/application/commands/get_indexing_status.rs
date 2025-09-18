use eyre::Result;

use crate::{
    application::dtos::indexing_dtos::IndexingStatusResponse,
    domain::repositories::IndexingProgressRepository,
};

pub struct GetIndexingStatusCommand {
    progress_repository: Box<dyn IndexingProgressRepository>,
}

impl GetIndexingStatusCommand {
    pub fn new(progress_repository: Box<dyn IndexingProgressRepository>) -> Self {
        Self {
            progress_repository,
        }
    }

    pub async fn execute(&self, chain_id: i32) -> Result<IndexingStatusResponse> {
        match self.progress_repository.get_progress(chain_id).await? {
            Some(progress) => Ok(IndexingStatusResponse {
                chain_id: progress.chain_id,
                last_indexed_block: progress.last_indexed_block,
                status: progress.status,
                error_message: progress.error_message,
                success: true,
            }),
            None => Ok(IndexingStatusResponse {
                chain_id,
                last_indexed_block: 0,
                status: "not_started".to_string(),
                error_message: None,
                success: false,
            }),
        }
    }
}
