use async_trait::async_trait;
use ethers::core::utils::hash_message;
use ethers::types::{Address, Signature};
use std::str::FromStr;
use std::sync::Arc;

use crate::domain::repositories::ProfileRepository;
use crate::domain::services::auth_service::{AuthChallenge, AuthResult, AuthService};
use crate::domain::value_objects::WalletAddress;

pub struct EthereumAddressVerificationService {
    profile_repository: Arc<dyn ProfileRepository>,
}

impl EthereumAddressVerificationService {
    pub fn new(profile_repository: Arc<dyn ProfileRepository>) -> Self {
        Self { profile_repository }
    }
}

#[async_trait]
impl AuthService for EthereumAddressVerificationService {
    async fn verify_signature(
        &self,
        challenge: &AuthChallenge,
        signature: &str,
    ) -> Result<Option<AuthResult>, Box<dyn std::error::Error>> {
        // Create the message with the nonce
        let message = format!(
            "Sign this message to authenticate with The Guild.\n\nNonce: {}",
            challenge.nonce
        );

        // EIP-191 prefix + keccak256
        let msg_hash = hash_message(message);

        // Parse signature and expected address
        let sig = Signature::from_str(signature)?;
        let expected: Address = challenge.address.parse()?;

        // Recover signer from signature
        let recovered = sig.recover(msg_hash)?;

        if recovered == expected {
            // Increment the nonce after successful verification
            let wallet_address = WalletAddress(challenge.address.clone());
            self.profile_repository
                .increment_login_nonce(&wallet_address)
                .await?;

            Ok(Some(AuthResult { wallet_address }))
        } else {
            Ok(None)
        }
    }
}

// Use the following to bypass signature verification

pub struct MockEthereumAddressVerificationService {}

impl MockEthereumAddressVerificationService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MockEthereumAddressVerificationService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthService for MockEthereumAddressVerificationService {
    async fn verify_signature(
        &self,
        _challenge: &AuthChallenge,
        _signature: &str,
    ) -> Result<Option<AuthResult>, Box<dyn std::error::Error>> {
        Ok(Some(AuthResult {
            wallet_address: WalletAddress("0x2581aAa94299787a8A588B2Fceb161A302939E28".to_string()),
        }))
    }
}
