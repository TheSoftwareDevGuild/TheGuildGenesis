use axum::{
    middleware,
    routing::get,
    Router,
};
use std::{env, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod application;
mod database;
mod domain;
mod infrastructure;
mod presentation;

use crate::{
    database::create_connection_pool,
    domain::services::indexing_service::EthereumIndexingService,
    infrastructure::repositories::postgres_ethereum_log_repository::PostgresEthereumLogRepository,
    presentation::{api, middlewares, AppState},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "guild_indexer=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Create database connection pool
    let pool = create_connection_pool().await?;
    tracing::info!("✅ Connected to database");

    // Create repositories
    let log_repository = Arc::new(PostgresEthereumLogRepository::new(pool.clone()));
    let progress_repository = Arc::new(PostgresEthereumLogRepository::new(pool.clone()));

    // Create services
    let indexing_service = Arc::new(EthereumIndexingService::new(
        log_repository.clone(),
        progress_repository.clone(),
    ));

    // Create app state
    let app_state = AppState::new(
        log_repository,
        progress_repository,
        indexing_service,
    );

    // Create router
    let app = Router::new()
        .merge(api::create_router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(middlewares::logging_middleware))
                .layer(middlewares::create_cors_layer()),
        )
        .with_state(app_state);

    // Start server
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse::<u16>()
        .unwrap_or(3002);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    tracing::info!("🚀 Guild Indexer server running on port {}", port);
    tracing::info!("📊 Health check: http://localhost:{}/health", port);
    tracing::info!("📈 Index logs: POST http://localhost:{}/index/logs", port);
    tracing::info!("🔍 Filter logs: POST http://localhost:{}/index/logs/filter", port);
    tracing::info!("📊 Status: GET http://localhost:{}/status/{{chain_id}}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
