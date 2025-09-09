use async_trait::async_trait;

use crate::domain::services::AuthService::{self, AuthChallenge, AuthResult};

pub struct EthereumAddressVerificationService {}

impl EthereumAddressVerificationService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthService for EthereumAddressVerificationService {
    async fn verify_signature(
        &self,
        challenge: &AuthChallenge,
        signature: &str,
    ) -> Result<AuthResult, Box<dyn std::error::Error>>;
}
