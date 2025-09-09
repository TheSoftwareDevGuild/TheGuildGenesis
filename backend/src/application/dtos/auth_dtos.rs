pub struct VerifyMessageRequest {
    pub address: String,
    pub nonce: String,
    pub message: String,
}

pub struct VerifyMessageResponse {
    pub success: bool,
    pub address: String,
}
