use std::sync::Arc;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::services::auth_service::AuthService as DomainAuthService;
use crate::domain::value_objects::wallet_address::WalletAddress;
use crate::application::dtos::auth_dtos::{NonceResponse, VerifyMessageRequest, VerifyMessageResponse};

#[derive(Clone)]
pub struct AuthApplicationService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    auth_service: DomainAuthService,
}

impl AuthApplicationService {
    pub fn new(user_repository: Box<dyn UserRepository + Send + Sync>) -> Self {
        Self {
            user_repository: Arc::from(user_repository),
            auth_service: DomainAuthService,
        }
    }
    
    pub async fn generate_nonce(&self) -> Result<NonceResponse, String> {
        let nonce = self.auth_service.generate_nonce();
        Ok(NonceResponse {
            nonce: nonce.to_string(),
        })
    }
    
    pub async fn verify_message(&self, request: VerifyMessageRequest) -> Result<VerifyMessageResponse, String> {
        // Validate wallet address
        let wallet_address = self.auth_service.validate_wallet_address(&request.address)?;
        
        // Verify SIWE message
        let is_valid = self.auth_service.verify_siwe_message(
            &request.message,
            &request.signature,
            &wallet_address,
        )?;
        
        if !is_valid {
            return Err("Invalid signature".to_string());
        }
        
        // Create or find user
        let user = match self.user_repository.find_by_wallet_address(&wallet_address).await? {
            Some(user) => user,
            None => {
                let new_user = crate::domain::entities::user::User::new(wallet_address.to_string());
                self.user_repository.create(new_user).await?
            }
        };
        
        Ok(VerifyMessageResponse {
            success: true,
            address: wallet_address.to_string(),
        })
    }
}
