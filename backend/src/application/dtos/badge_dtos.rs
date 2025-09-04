use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct BadgeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub issuer_address: String,
    pub image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBadgeRequest {
    pub name: String,
    pub description: String,
    pub issuer_address: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GiveBadgeRequest {
    pub profile_address: String,
}

#[derive(Debug, Serialize)]
pub struct BadgeListResponse {
    pub badges: Vec<BadgeResponse>,
}

#[derive(Debug, Serialize)]
pub struct UserBadgeResponse {
    pub id: Uuid,
    pub badge: BadgeResponse,
    pub awarded_at: String,
    pub awarded_by: String,
}

