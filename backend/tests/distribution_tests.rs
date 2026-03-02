use guild_backend::infrastructure::repositories::postgres_distribution_repository::PostgresDistributionRepository;
use guild_backend::infrastructure::repositories::postgres_github_issue_repository::PostgresGithubIssueRepository;
use guild_backend::infrastructure::repositories::postgres_project_repository::PostgresProjectRepository;
use guild_backend::infrastructure::services::rest_github_service::RestGithubService;
use guild_backend::presentation::api::{test_api, AppState};
use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::test]
async fn register_distribution_list_succeeds() {
    std::env::set_var("TEST_MODE", "1");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://guild_user:guild_password@localhost:5432/guild_genesis".to_string()
    });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();

    let profile_repository = std::sync::Arc::new(
        guild_backend::infrastructure::repositories::PostgresProfileRepository::new(pool.clone()),
    );
    let auth_service = guild_backend::infrastructure::services::ethereum_address_verification_service::EthereumAddressVerificationService::new(profile_repository.clone());
    let project_repository = Arc::from(PostgresProjectRepository::new(pool.clone()));
    let distribution_repository = Arc::from(PostgresDistributionRepository::new(pool.clone()));
    let github_issue_repository = Arc::from(PostgresGithubIssueRepository::new(pool.clone()));
    let github_service: Arc<dyn guild_backend::domain::services::github_service::GithubService> =
        Arc::from(RestGithubService::new());

    let state = AppState {
        profile_repository,
        project_repository,
        distribution_repository,
        auth_service: std::sync::Arc::new(auth_service),
        github_issue_repository,
        github_service,
    };
    let app = test_api(state);

    let server = axum::serve(listener, app);
    tokio::spawn(async move { server.await.unwrap() });

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS distributions (
            address TEXT NOT NULL,
            badge_name TEXT NOT NULL,
            distribution_id TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query("DELETE FROM distributions WHERE distribution_id = $1")
        .bind("dist-test-001")
        .execute(&pool)
        .await
        .unwrap();

    let response = client
        .post(format!("{}/distributions", base))
        .header(
            "x-eth-address",
            "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
        )
        .json(&json!({
            "distributions": [
                {
                    "address": "0x1234567890123456789012345678901234567890",
                    "badgeName": "Contributor",
                    "distributionId": "dist-test-001"
                },
                {
                    "address": "0x1234567890123456789012345678901234567891",
                    "badgeName": "Reviewer",
                    "distributionId": "dist-test-001"
                }
            ]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let inserted = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM distributions WHERE distribution_id = $1",
    )
    .bind("dist-test-001")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(inserted, 2);
}
