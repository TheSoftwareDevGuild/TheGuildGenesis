use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::value_objects::wallet_address::WalletAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthChallenge {
    pub nonce: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthResult {
    pub user_id: Uuid,
    pub wallet_address: WalletAddress,
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn create_challenge(&self, wallet_address: &WalletAddress) -> Result<AuthChallenge, Box<dyn std::error::Error>>;
    async fn verify_signature(&self, challenge: &AuthChallenge, signature: &str) -> Result<AuthResult, Box<dyn std::error::Error>>;
}
