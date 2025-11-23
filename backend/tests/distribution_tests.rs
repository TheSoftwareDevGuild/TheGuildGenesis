use uuid::Uuid;
use serde_json::json;
use crate::helpers::{spawn_app, get_pg_pool}; // adapt to your test helpers

#[tokio::test]
async fn create_distribution_inserts_rows() {
    // spawn_app should start server + return test client and DB pool
    let app = spawn_app().await; // adapt to your test harness
    let pool = &app.pg_pool;

    let payload = json!({
        "items": [
            { "address": "0x1111111111111111111111111111111111111111", "badge_name": "contributor", "metadata": { "reason": "helped" } },
            { "address": "0x2222222222222222222222222222222222222222", "badge_name": "builder", "metadata": null }
        ],
        "metadata": { "campaign": "nov-2025" }
    });

    let response = app.post("/distributions").json(&payload).send().await.unwrap();
    assert_eq!(response.status(), 201);

    let body: serde_json::Value = response.json().await.unwrap();
    let distribution_id = body["distribution_id"].as_str().unwrap();
    let uuid = Uuid::parse_str(distribution_id).unwrap();

    // Query DB directly
    let rows = sqlx::query!("SELECT count(*) FROM distributions WHERE distribution_id = $1", uuid)
        .fetch_one(pool)
        .await
        .unwrap();

    let count: i64 = rows.count.unwrap_or(0);
    assert_eq!(count, 2);
}
