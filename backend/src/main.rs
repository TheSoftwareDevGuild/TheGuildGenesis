use presentation::api::create_app;
use std::{env, net::SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load env before reading variables
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "guild_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .unwrap_or_else(|_| {
            tracing::warn!("Could not connect to database, using mock pool");
            panic!("Database connection required");
        });

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = create_app(pool).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
