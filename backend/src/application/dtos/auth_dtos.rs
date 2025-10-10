use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyMessageRequest {
    pub address: String,
    pub nonce: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyMessageResponse {
    pub success: bool,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonceResponse {
    pub nonce: i64,
    pub address: String,
}
