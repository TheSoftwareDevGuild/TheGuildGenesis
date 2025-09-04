use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod database;
mod handlers;
mod middleware;
mod models;

use handlers::{auth, dev};

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

    // Create the app without database for development
    let app = create_dev_app().await;

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn create_dev_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/siwe/nonce", get(auth::get_nonce))
        .route("/siwe/verify", post(dev::verify_message_dev))
        .route("/profiles", get(dev::get_profiles_dev))
        .route("/profiles", post(dev::create_profile_dev))
        .route("/profiles/:address", get(dev::get_profile_dev))
        .route("/profiles/:address", put(dev::update_profile_dev))
        .route("/badges", get(dev::get_badges_dev))
        .route("/badges", post(dev::create_badge_dev))
        .route("/badges/:id", get(dev::get_badge_dev))
        .route("/badges/:id/give", post(dev::give_badge_dev))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024)), // 1MB limit
        )
}

async fn health_check() -> &'static str {
    "OK"
}
