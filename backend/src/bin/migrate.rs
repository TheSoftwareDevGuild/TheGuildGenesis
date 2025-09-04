use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://guild_user:guild_password@localhost:5432/guild_genesis".to_string());
    
    println!("🔌 Connecting to database: {}", database_url);
    
    let pool = PgPool::connect(&database_url).await?;
    
    println!("🏃‍♂️ Running migrations...");
    
    // Run migrations from the migrations directory
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    println!("✅ Migrations completed successfully");
    Ok(())
}
