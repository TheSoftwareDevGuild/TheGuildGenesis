use std::error::Error;

use async_trait::async_trait;
use sqlx::{PgPool, Row};
// removed uuid usage as ids are text

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
        let rows = sqlx::query(
            r#"
            SELECT id, event_type, timestamp, created_at
            FROM ethereum_events
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let mut events: Vec<EthereumEvent> = Vec::with_capacity(rows.len());
        for row in rows {
            let id: String = row
                .try_get("id")
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let event_type_str: String = row
                .try_get("event_type")
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let timestamp: chrono::DateTime<chrono::Utc> = row
                .try_get("timestamp")
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            let created_at: chrono::DateTime<chrono::Utc> = row
                .try_get("created_at")
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let event_type = serde_json::from_str(&event_type_str)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            events.push(EthereumEvent {
                id,
                event_type,
                timestamp,
                created_at,
            });
        }

        Ok(events)
    }

    async fn insert_many(&self, ethereum_events: Vec<EthereumEvent>) -> Result<(), Box<dyn Error>> {
        if ethereum_events.is_empty() {
            return Ok(());
        }

        let ids: Vec<String> = ethereum_events.iter().map(|e| e.id.clone()).collect();
        let event_types: Vec<String> = ethereum_events
            .iter()
            .map(|e| serde_json::to_string(&e.event_type).unwrap())
            .collect();
        let timestamps: Vec<chrono::DateTime<chrono::Utc>> =
            ethereum_events.iter().map(|e| e.timestamp).collect();
        let created_ats: Vec<chrono::DateTime<chrono::Utc>> =
            ethereum_events.iter().map(|e| e.created_at).collect();

        sqlx::query(
            r#"
            INSERT INTO ethereum_events (id, event_type, timestamp, created_at)
            SELECT * FROM UNNEST($1::text[], $2::text[], $3::timestamptz[], $4::timestamptz[])
            "#
        )
        .bind(&ids)
        .bind(&event_types)
        .bind(&timestamps)
        .bind(&created_ats)
        .execute(&self.pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(())
    }
}
