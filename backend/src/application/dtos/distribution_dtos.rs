use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDistributionRequest {
    pub items: Vec<CreateDistributionDto>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDistributionDto {
    pub address: String,
    pub badge_name: String,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct DistributionResponse {
    pub id: Uuid,
    pub distribution_id: Uuid,
    pub address: String,
    pub badge_name: String,
    pub metadata: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
}