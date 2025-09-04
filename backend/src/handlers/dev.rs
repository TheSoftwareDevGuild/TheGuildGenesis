use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};

pub async fn verify_message_dev(Json(payload): Json<Value>) -> Result<Json<Value>, StatusCode> {
    let address = payload.get("address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    Ok(Json(json!({
        "success": true,
        "address": address
    })))
}

pub async fn get_profiles_dev() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "profiles": [
            {
                "address": "0x1234567890123456789012345678901234567890",
                "name": "Alice Developer",
                "description": "Full-stack developer passionate about Web3 and Rust",
                "avatar_url": "https://example.com/avatar.jpg",
                "badges": [
                    {
                        "id": "1",
                        "name": "Rust",
                        "description": "Rust programming",
                        "issuer": "0xabcd1234567890123456789012345678901234567890"
                    },
                    {
                        "id": "2",
                        "name": "React",
                        "description": "React development",
                        "issuer": "0xefgh1234567890123456789012345678901234567890"
                    }
                ]
            }
        ]
    })))
}

pub async fn get_profile_dev(Path(address): Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "address": address,
        "name": "Alice Developer",
        "description": "Full-stack developer passionate about Web3 and Rust",
        "avatar_url": "https://example.com/avatar.jpg",
        "badges": []
    })))
}

pub async fn create_profile_dev(Json(payload): Json<Value>) -> Result<Json<Value>, StatusCode> {
    let address = payload.get("address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    Ok(Json(json!({
        "success": true,
        "address": address
    })))
}

pub async fn update_profile_dev(
    Path(address): Path<String>,
    Json(_payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "success": true,
        "address": address
    })))
}

pub async fn get_badges_dev() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "badges": [
            {
                "id": "1",
                "name": "Rust",
                "description": "Rust programming language",
                "issuer_address": "0xabcd1234567890123456789012345678901234567890"
            },
            {
                "id": "2",
                "name": "React",
                "description": "React development",
                "issuer_address": "0xefgh1234567890123456789012345678901234567890"
            }
        ]
    })))
}

pub async fn get_badge_dev(Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "id": id,
        "name": "Rust",
        "description": "Rust programming language",
        "issuer_address": "0xabcd1234567890123456789012345678901234567890"
    })))
}

pub async fn create_badge_dev(Json(payload): Json<Value>) -> Result<Json<Value>, StatusCode> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    Ok(Json(json!({
        "success": true,
        "name": name
    })))
}

pub async fn give_badge_dev(
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let profile_address = payload.get("profile_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    Ok(Json(json!({
        "success": true,
        "badge_id": id,
        "profile_address": profile_address
    })))
}



