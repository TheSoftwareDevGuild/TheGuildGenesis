use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/guild_genesis".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    Ok(pool)
}
