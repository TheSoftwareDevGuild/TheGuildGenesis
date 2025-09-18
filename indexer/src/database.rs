use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub async fn create_connection_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await
}
