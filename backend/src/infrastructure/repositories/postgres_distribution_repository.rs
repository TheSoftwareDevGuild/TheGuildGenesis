use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::repositories::distribution_repository::{
    DistributionRecord, DistributionRepository,
};

#[derive(Clone)]
pub struct PostgresDistributionRepository {
    pool: PgPool,
}

impl PostgresDistributionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DistributionRepository for PostgresDistributionRepository {
    async fn create_many(
        &self,
        records: &[DistributionRecord],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;

        for record in records {
            sqlx::query(
                r#"
                INSERT INTO distributions (address, badge_name, distribution_id)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(&record.address)
            .bind(&record.badge_name)
            .bind(&record.distribution_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn list(
        &self,
        distribution_id: Option<&str>,
    ) -> Result<Vec<DistributionRecord>, Box<dyn std::error::Error>> {
        let rows: Vec<(String, String, String)> = if let Some(id) = distribution_id {
            sqlx::query_as(
                r#"
                SELECT address, badge_name, distribution_id
                FROM distributions
                WHERE distribution_id = $1
                ORDER BY address ASC
                "#,
            )
            .bind(id)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                r#"
                SELECT address, badge_name, distribution_id
                FROM distributions
                ORDER BY distribution_id DESC, address ASC
                "#,
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(rows
            .into_iter()
            .map(
                |(address, badge_name, distribution_id)| DistributionRecord {
                    address,
                    badge_name,
                    distribution_id,
                },
            )
            .collect())
    }
}
