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
                id: row.id,
                event_type: row.event_type.clone(),
                timestamp: row.timestamp.clone(),
                created_at: row.created_at.clone(),
            })
            .collect::<Vec<_>>())
    }

    async fn insert_many(&self, ethereum_events: Vec<EthereumEvent>) -> Result<(), Box<dyn Error>> {
        if ethereum_events.is_empty() {
            return Ok(());
        }

        let ids: Vec<Uuid> = ethereum_events
            .iter()
            .map(|e| Uuid::parse_str(&e.id).unwrap())
            .collect();
        let event_types: Vec<String> = ethereum_events
            .iter()
            .map(|e| format!("{:?}", e.event_type))
            .collect();
        let timestamps: Vec<chrono::DateTime<chrono::Utc>> =
            ethereum_events.iter().map(|e| e.timestamp).collect();
        let created_ats: Vec<chrono::DateTime<chrono::Utc>> =
            ethereum_events.iter().map(|e| e.created_at).collect();

        let rows = sqlx::query_as!(
            EthereumEvent,
            r#"
            INSERT INTO ethereum_events (id, event_type, timestamp, created_at)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::timestamptz[], $4::timestamptz[])
            RETURNING id, event_type, timestamp, created_at
            "#,
            &ids,
            &event_types,
            &timestamps,
            &created_ats
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }
}
