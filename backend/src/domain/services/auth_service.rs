use async_trait::async_trait;

use crate::domain::value_objects::wallet_address::WalletAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthChallenge {
    pub nonce: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthResult {
    pub wallet_address: WalletAddress,
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn verify_signature(
        &self,
        challenge: &AuthChallenge,
        signature: &str,
    ) -> Result<AuthResult, Box<dyn std::error::Error>>;
}
