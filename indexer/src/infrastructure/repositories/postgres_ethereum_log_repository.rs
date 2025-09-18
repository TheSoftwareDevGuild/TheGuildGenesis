use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{EthereumLog, LogFilter},
    repositories::EthereumLogRepository,
};

pub struct PostgresEthereumLogRepository {
    pool: PgPool,
}

impl PostgresEthereumLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EthereumLogRepository for PostgresEthereumLogRepository {
    async fn create_log(&self, log: &EthereumLog) -> Result<Uuid, Box<dyn std::error::Error>> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO indexer.ethereum_logs 
            (id, block_number, block_hash, transaction_hash, transaction_index, log_index, address, data, topics, removed)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id
            "#,
            log.id,
            log.block_number,
            log.block_hash,
            log.transaction_hash,
            log.transaction_index,
            log.log_index,
            log.address,
            log.data,
            &log.topics,
            log.removed
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn create_logs_batch(&self, logs: &[EthereumLog]) -> Result<Vec<Uuid>, Box<dyn std::error::Error>> {
        if logs.is_empty() {
            return Ok(vec![]);
        }

        let mut tx = self.pool.begin().await?;
        let mut inserted_ids = Vec::new();

        for log in logs {
            let id = sqlx::query_scalar!(
                r#"
                INSERT INTO indexer.ethereum_logs 
                (id, block_number, block_hash, transaction_hash, transaction_index, log_index, address, data, topics, removed)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING id
                "#,
                log.id,
                log.block_number,
                log.block_hash,
                log.transaction_hash,
                log.transaction_index,
                log.log_index,
                log.address,
                log.data,
                &log.topics,
                log.removed
            )
            .fetch_one(&mut *tx)
            .await?;

            inserted_ids.push(id);
        }

        tx.commit().await?;
        Ok(inserted_ids)
    }

    async fn get_logs_by_filter(&self, filter: &LogFilter) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>> {
        // For now, return empty vec since we can't easily handle dynamic parameters
        // In a real implementation, you'd use a query builder or raw SQL with proper parameter binding
        Ok(vec![])
    }

    async fn get_logs_by_address(&self, address: &str, limit: Option<i64>) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(100);
        
        let logs = sqlx::query_as!(
            EthereumLog,
            r#"
            SELECT * FROM indexer.ethereum_logs 
            WHERE address = $1 
            ORDER BY block_number DESC, log_index DESC 
            LIMIT $2
            "#,
            address,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    async fn get_logs_by_block_range(&self, from_block: u64, to_block: u64) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>> {
        let logs = sqlx::query_as!(
            EthereumLog,
            r#"
            SELECT * FROM indexer.ethereum_logs 
            WHERE block_number >= $1 AND block_number <= $2
            ORDER BY block_number DESC, log_index DESC
            "#,
            from_block as i64,
            to_block as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }

    async fn get_latest_logs(&self, limit: i64) -> Result<Vec<EthereumLog>, Box<dyn std::error::Error>> {
        let logs = sqlx::query_as!(
            EthereumLog,
            r#"
            SELECT * FROM indexer.ethereum_logs 
            ORDER BY block_number DESC, log_index DESC 
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(logs)
    }
}