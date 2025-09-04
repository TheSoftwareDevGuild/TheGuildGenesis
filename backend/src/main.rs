use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use guild_backend::{create_app, database::create_pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "guild_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // For development, create a mock pool or skip database
    // In production, you would create a real database connection
    let pool = create_pool().await.unwrap_or_else(|_| {
        tracing::warn!("Could not connect to database, running without database");
        // Return a mock pool or handle this differently
        panic!("Database connection required");
    });

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Create the app
    let app = create_app(pool).await;

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}