use async_trait::async_trait;
use ethers::core::utils::hash_message;
use ethers::types::{Address, Signature};
use std::str::FromStr;

use crate::domain::services::auth_service::{AuthChallenge, AuthResult, AuthService};
use crate::domain::value_objects::WalletAddress;

pub struct EthereumAddressVerificationService {}

impl EthereumAddressVerificationService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EthereumAddressVerificationService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthService for EthereumAddressVerificationService {
    async fn verify_signature(
        &self,
        challenge: &AuthChallenge,
        signature: &str,
    ) -> Result<Option<AuthResult>, Box<dyn std::error::Error>> {
        const EXPECTED_MSG: &str = "LOGIN_NONCE"; // or whatever constant string you are signing

        // EIP-191 prefix + keccak256
        let msg_hash = hash_message(EXPECTED_MSG);

        // Parse signature and expected address
        let sig = Signature::from_str(signature)?;
        let expected: Address = challenge.address.parse()?;

        // Recover signer from signature
        let recovered = sig.recover(msg_hash)?;

        if recovered == expected {
            Ok(Some(AuthResult {
                wallet_address: WalletAddress(challenge.address.clone()),
            }))
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
