use std::sync::Arc;

use crate::{
    domain::{
        repositories::{EthereumLogRepository, IndexingProgressRepository},
        services::IndexingService,
    },
    infrastructure::repositories::postgres_ethereum_log_repository::PostgresEthereumLogRepository,
};

pub mod api;
pub mod handlers;
pub mod middlewares;

pub struct AppState {
    pub log_repository: Arc<dyn EthereumLogRepository>,
    pub progress_repository: Arc<dyn IndexingProgressRepository>,
    pub indexing_service: Arc<dyn IndexingService>,
}

impl AppState {
    pub fn new(
        log_repository: Arc<dyn EthereumLogRepository>,
        progress_repository: Arc<dyn IndexingProgressRepository>,
        indexing_service: Arc<dyn IndexingService>,
    ) -> Self {
        Self {
            log_repository,
            progress_repository,
            indexing_service,
        }
    }
}
