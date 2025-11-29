use crate::domain::entities::distribution::Distribution;
use crate::domain::repositories::distribution_repository::DistributionRepository;
use async_trait::async_trait;
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::value_objects::WalletAddress;

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
    async fn insert_batch(
        &self,
        distribution_id: Uuid,
        items: Vec<Distribution>,
    ) -> Result<(), anyhow::Error> {
        for item in items {
            let metadata_json = item.metadata.clone();

            sqlx::query!(
                r#"
                INSERT INTO distributions (id, distribution_id, address, badge_name, metadata, created_at)
                VALUES ($1, $2, $3, $4, $5::jsonb, $6)
                "#,
                item.id,
                distribution_id,
                item.address.as_str(),
                item.badge_name,
                metadata_json,
                item.created_at
            )
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::Error::new(e))?;
        }

        Ok(())
    }

    async fn get_by_distribution_id(&self, distribution_id: Uuid) -> Result<Vec<Distribution>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, distribution_id, address, badge_name, metadata, created_at
            FROM distributions
            WHERE distribution_id = $1
            ORDER BY created_at ASC
            "#,
            distribution_id
        )
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|r| {
                Distribution {
                    id: r.id,
                    distribution_id: r.distribution_id,
                    address: WalletAddress::new(r.address).expect("Invalid wallet address"),
                    badge_name: r.badge_name,
                    metadata: r.metadata,
                    created_at: r.created_at.expect("created_at should not be null"),
                }
            })
            .collect();

        Ok(items)
    }

    async fn get_by_address(&self, address: &str) -> Result<Vec<Distribution>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, distribution_id, address, badge_name, metadata, created_at
            FROM distributions
            WHERE address = $1
            ORDER BY created_at DESC
            "#,
            address
        )
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|r| {
                Distribution {
                    id: r.id,
                    distribution_id: r.distribution_id,
                    address: WalletAddress::new(r.address).expect("Invalid wallet address"),
                    badge_name: r.badge_name,
                    metadata: r.metadata,
                    created_at: r.created_at.expect("created_at should not be null"),
                }
            })
            .collect();

        Ok(items)
    }
}