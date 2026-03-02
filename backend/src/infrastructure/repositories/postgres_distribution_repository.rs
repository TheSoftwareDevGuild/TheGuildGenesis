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
}
