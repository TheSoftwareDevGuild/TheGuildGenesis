use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NonceResponse {
    pub nonce: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub address: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyMessageResponse {
    pub success: bool,
    pub address: String,
}
