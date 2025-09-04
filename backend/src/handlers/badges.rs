use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

pub async fn get_badges(State(_pool): State<sqlx::PgPool>) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement database query
    Ok(Json(json!({
        "badges": []
    })))
}

pub async fn get_badge(
    State(_pool): State<sqlx::PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement database query
    Ok(Json(json!({
        "id": id,
        "name": "",
        "description": "",
        "issuer_address": ""
    })))
}

pub async fn create_badge(
    State(_pool): State<sqlx::PgPool>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // TODO: Implement database insert
    Ok(Json(json!({
        "success": true,
        "name": name
    })))
}

pub async fn give_badge(
    State(_pool): State<sqlx::PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let profile_address = payload.get("profile_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // TODO: Implement database insert
    Ok(Json(json!({
        "success": true,
        "badge_id": id,
        "profile_address": profile_address
    })))
}
