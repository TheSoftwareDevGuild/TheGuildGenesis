use std::sync::Arc;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::auth_service::AuthService as DomainAuthService;
use crate::domain::value_objects::wallet_address::WalletAddress;
use crate::application::dtos::auth_dtos::{NonceResponse, VerifyMessageRequest, VerifyMessageResponse};

// Mock implementation for development
struct MockAuthService;

#[async_trait::async_trait]
impl DomainAuthService for MockAuthService {
    async fn create_challenge(&self, _wallet_address: &WalletAddress) -> Result<crate::domain::services::auth_service::AuthChallenge, Box<dyn std::error::Error>> {
        Ok(crate::domain::services::auth_service::AuthChallenge {
            nonce: "mock-nonce".to_string(),
            message: "mock-message".to_string(),
        })
    }
    
    async fn verify_signature(&self, _challenge: &crate::domain::services::auth_service::AuthChallenge, _signature: &str) -> Result<crate::domain::services::auth_service::AuthResult, Box<dyn std::error::Error>> {
        Ok(crate::domain::services::auth_service::AuthResult {
            user_id: uuid::Uuid::new_v4(),
            wallet_address: WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap(),
        })
    }
}

#[derive(Clone)]
pub struct AuthApplicationService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    auth_service: Arc<dyn DomainAuthService + Send + Sync>,
}

impl AuthApplicationService {
    pub fn new(user_repository: Box<dyn UserRepository + Send + Sync>) -> Self {
        Self {
            user_repository: Arc::from(user_repository),
            auth_service: Arc::new(MockAuthService),
        }
    }
    
    pub async fn generate_nonce(&self) -> Result<NonceResponse, String> {
        let wallet_address = WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        let challenge = self.auth_service.create_challenge(&wallet_address).await.map_err(|e| e.to_string())?;
        Ok(NonceResponse {
            nonce: challenge.nonce,
        })
    }
    
    pub async fn verify_message(&self, request: VerifyMessageRequest) -> Result<VerifyMessageResponse, String> {
        // Validate wallet address
        let wallet_address = WalletAddress::new(request.address.clone()).map_err(|e| e.to_string())?;
        
        // Create challenge and verify signature
        let challenge = crate::domain::services::auth_service::AuthChallenge {
            nonce: request.nonce,
            message: request.message,
        };
        
        let auth_result = self.auth_service.verify_signature(&challenge, &request.signature).await.map_err(|e| e.to_string())?;
        
        // Create or find user
        let user = match self.user_repository.find_by_wallet_address(&wallet_address.to_string()).await.map_err(|e| e.to_string())? {
            Some(user) => user,
            None => {
                let new_user = crate::domain::entities::user::User::new(wallet_address.to_string());
                self.user_repository.create(&new_user).await.map_err(|e| e.to_string())?;
                new_user
            }
        };
        
        Ok(VerifyMessageResponse {
            success: true,
            address: wallet_address.to_string(),
        })
    }
}
