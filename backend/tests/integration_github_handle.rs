use guild_backend::application::dtos::profile_dtos::ProfileResponse;
use guild_backend::presentation::api::{test_api, AppState};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::test]
async fn valid_github_handle_works() {
    std::env::set_var("TEST_MODE", "1");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://guild_user:guild_password@localhost:5433/guild_genesis".to_string()
    });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let profile_repository =
        guild_backend::infrastructure::repositories::PostgresProfileRepository::new(pool.clone());
    let auth_service = guild_backend::infrastructure::services::ethereum_address_verification_service::EthereumAddressVerificationService::new();
    let state = AppState {
        profile_repository: std::sync::Arc::new(profile_repository),
        auth_service: std::sync::Arc::new(auth_service),
    };
    let app = test_api(state);

    let server = axum::serve(listener, app);
    tokio::spawn(async move { server.await.unwrap() });

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    let address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
    sqlx::query("DELETE FROM profiles WHERE address = $1")
        .bind(address)
        .execute(&pool)
        .await
        .unwrap();

    // Create profile
    let create_resp = client
        .post(&format!("{}/profiles/", base))
        .header("x-eth-address", address)
        .json(&json!({
            "name": "Alice",
            "description": null,
            "avatar_url": null
        }))
        .send()
        .await
        .unwrap();

    // Accept either 200 or 201
    assert_eq!(create_resp.status(), reqwest::StatusCode::CREATED);

    let body = create_resp.json::<serde_json::Value>().await.unwrap();
    let address = body["address"].as_str().unwrap();

    // Update with valid GitHub handle
    let update_resp = client
        .put(&format!("{}/profiles/{}", base, address))
        .header("x-eth-address", address)
        .json(&json!({
            "github_login": "ValidUser123test"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(update_resp.status(), 200);
    let updated: ProfileResponse = update_resp.json().await.unwrap();
    assert_eq!(updated.github_login, Some("ValidUser123test".to_string()));
}

#[tokio::test]
async fn invalid_format_rejected() {
    std::env::set_var("TEST_MODE", "1");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://guild_user:guild_password@localhost:5433/guild_genesis".to_string()
    });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let profile_repository =
        guild_backend::infrastructure::repositories::PostgresProfileRepository::new(pool.clone());
    let auth_service = guild_backend::infrastructure::services::ethereum_address_verification_service::EthereumAddressVerificationService::new();
    let state = AppState {
        profile_repository: std::sync::Arc::new(profile_repository),
        auth_service: std::sync::Arc::new(auth_service),
    };
    let app = test_api(state);

    let server = axum::serve(listener, app);
    tokio::spawn(async move { server.await.unwrap() });

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    let address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44f";
    sqlx::query("DELETE FROM profiles WHERE address = $1")
        .bind(address)
        .execute(&pool)
        .await
        .unwrap();

    // Create profile
    let create_resp = client
        .post(&format!("{}/profiles/", base))
        .header("x-eth-address", address)
        .json(&json!({
            "name": "Bob",
            "description": null,
            "avatar_url": null
        }))
        .send()
        .await
        .unwrap();
    // Similar acceptance for create
    assert_eq!(
        create_resp.status(),
        reqwest::StatusCode::CREATED,
        "Create failed: {}",
        create_resp.status()
    );
    let create_body = create_resp.json::<serde_json::Value>().await.unwrap();
    let address = create_body["address"].as_str().unwrap();

    // Update with invalid handle
    let update_resp = client
        .put(&format!("{}/profiles/{}", base, address))
        .header("x-eth-address", address)
        .json(&json!({
            "github_login": "bad@name"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(update_resp.status(), 400);

    // Optionally, try parse message if provided
    if let Ok(err_json) = update_resp.json::<serde_json::Value>().await {
        let msg = err_json["error"].as_str().unwrap_or("");
        assert!(msg.contains("Invalid GitHub handle"));
    }
}

#[tokio::test]
async fn conflict_case_insensitive() {
    std::env::set_var("TEST_MODE", "1");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://guild_user:guild_password@localhost:5433/guild_genesis".to_string()
    });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let profile_repository =
        guild_backend::infrastructure::repositories::PostgresProfileRepository::new(pool.clone());
    let auth_service = guild_backend::infrastructure::services::ethereum_address_verification_service::EthereumAddressVerificationService::new();
    let state = AppState {
        profile_repository: std::sync::Arc::new(profile_repository),
        auth_service: std::sync::Arc::new(auth_service),
    };
    let app = test_api(state);

    let server = axum::serve(listener, app);
    tokio::spawn(async move { server.await.unwrap() });

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    let addr1 = "0x742d35Cc6634C0532925a3b844Bc454e4438f44g";
    let addr2 = "0x742d35Cc6634C0532925a3b844Bc454e4438f44h";
    sqlx::query("DELETE FROM profiles WHERE address = $1 OR address = $2")
        .bind(addr1)
        .bind(addr2)
        .execute(&pool)
        .await
        .unwrap();

    // Create first profile
    let create1 = client
        .post(&format!("{}/profiles/", base))
        .header("x-eth-address", addr1)
        .json(&json!({
            "name": "Carol",
            "description": null,
            "avatar_url": null
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(
        create1.status(),
        reqwest::StatusCode::CREATED,
        "First create failed: {}",
        create1.status()
    );
    let body1 = create1.json::<serde_json::Value>().await.unwrap();
    let addr1 = body1["address"].as_str().unwrap();

    // Create second profile
    let create2 = client
        .post(&format!("{}/profiles/", base))
        .header("x-eth-address", addr2)
        .json(&json!({
            "name": "Dave",
            "description": null,
            "avatar_url": null
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(
        create2.status(),
        reqwest::StatusCode::CREATED,
        "Second create failed: {}",
        create2.status()
    );
    let body2 = create2.json::<serde_json::Value>().await.unwrap();
    let addr2 = body2["address"].as_str().unwrap();

    // Update first with "Alice"
    let _ = client
        .put(&format!("{}/profiles/{}", base, addr1))
        .header("x-eth-address", addr1)
        .json(&json!({ "github_login": "Alice" }))
        .send()
        .await
        .unwrap();

    // Update second with "alice" (lowercase) should conflict
    let conflict_resp = client
        .put(&format!("{}/profiles/{}", base, addr2))
        .header("x-eth-address", addr2)
        .json(&json!({ "github_login": "alice" }))
        .send()
        .await
        .unwrap();

    assert_eq!(conflict_resp.status(), 409);

    if let Ok(err_json) = conflict_resp.json::<serde_json::Value>().await {
        let msg = err_json["error"].as_str().unwrap_or("");
        assert!(msg.contains("already taken"));
    }
}
