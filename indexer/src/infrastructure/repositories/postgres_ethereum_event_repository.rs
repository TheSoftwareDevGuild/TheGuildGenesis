use std::error::Error;

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::ethereum_event::EthereumEvent,
    repositories::ethereum_event_repository::EthereumEventRepository,
};

#[derive(Clone)]
pub struct PostgresEthereumEventRepository {
    pool: PgPool,
}

impl PostgresEthereumEventRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EthereumEventRepository for PostgresEthereumEventRepository {
    async fn list(&self) -> Result<Vec<EthereumEvent>, Box<dyn Error>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, event_type, timestamp, created_at
            FROM ethereum_events
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(rows
            .iter()
            .map(|row| EthereumEvent {
                id: Uuid::parse_str(&(row.id)).unwrap(),
                event_type: row.event_type.clone(),
                timestamp: row.timestamp.clone(),
                created_at: row.created_at.clone(),
            })
            .collect::<Vec<_>>())
    }
}
