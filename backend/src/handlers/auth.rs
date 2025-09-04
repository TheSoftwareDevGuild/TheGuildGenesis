use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

pub async fn get_nonce() -> Result<Json<Value>, StatusCode> {
    // Generate a random nonce for SIWE
    let nonce = uuid::Uuid::new_v4().to_string();
    Ok(Json(json!({
        "nonce": nonce
    })))
}

pub async fn verify_message(
    State(_pool): State<sqlx::PgPool>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement SIWE verification
    // For now, just return success
    let address = payload.get("address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    Ok(Json(json!({
        "success": true,
        "address": address
    })))
}
