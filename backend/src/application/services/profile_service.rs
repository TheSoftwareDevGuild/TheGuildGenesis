use std::sync::Arc;
use uuid::Uuid;
use crate::domain::repositories::{user_repository::UserRepository, profile_repository::ProfileRepository};
use crate::domain::value_objects::wallet_address::WalletAddress;
use crate::application::dtos::profile_dtos::{
    ProfileResponse, CreateProfileRequest, UpdateProfileRequest, ProfileListResponse
};

#[derive(Clone)]
pub struct ProfileApplicationService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    profile_repository: Arc<dyn ProfileRepository + Send + Sync>,
}

impl ProfileApplicationService {
    pub fn new(
        user_repository: Box<dyn UserRepository + Send + Sync>,
        profile_repository: Box<dyn ProfileRepository + Send + Sync>,
    ) -> Self {
        Self {
            user_repository: Arc::from(user_repository),
            profile_repository: Arc::from(profile_repository),
        }
    }
    
    pub async fn get_profiles(&self) -> Result<ProfileListResponse, String> {
        let profiles = self.profile_repository.find_all().await.map_err(|e| e.to_string())?;
        let profile_responses: Vec<ProfileResponse> = profiles
            .into_iter()
            .map(|profile| ProfileResponse {
                id: profile.id,
                user_id: profile.user_id,
                name: profile.name.unwrap_or_default(),
                description: profile.description,
                avatar_url: profile.avatar_url,
                created_at: profile.created_at,
                updated_at: profile.updated_at,
            })
            .collect();
        
        Ok(ProfileListResponse {
            profiles: profile_responses,
        })
    }
    
    pub async fn get_profile(&self, address: String) -> Result<ProfileResponse, String> {
        let wallet_address = WalletAddress::new(address).map_err(|e| e.to_string())?;
        let user = self.user_repository
            .find_by_wallet_address(&wallet_address.to_string())
            .await.map_err(|e| e.to_string())?
            .ok_or("User not found")?;
        
        let profile = self.profile_repository
            .find_by_user_id(&user.id)
            .await.map_err(|e| e.to_string())?
            .ok_or("Profile not found")?;
        
        Ok(ProfileResponse {
            id: profile.id,
            user_id: profile.user_id,
            name: profile.name.unwrap_or_default(),
            description: profile.description,
            avatar_url: profile.avatar_url,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        })
    }
    
    pub async fn create_profile(&self, request: CreateProfileRequest) -> Result<ProfileResponse, String> {
        let wallet_address = WalletAddress::new(request.address).map_err(|e| e.to_string())?;
        let user = self.user_repository
            .find_by_wallet_address(&wallet_address.to_string())
            .await.map_err(|e| e.to_string())?
            .ok_or("User not found")?;
        
        // Check if profile already exists
        if self.profile_repository.find_by_user_id(&user.id).await.map_err(|e| e.to_string())?.is_some() {
            return Err("Profile already exists for this user".to_string());
        }
        
        let mut profile = crate::domain::entities::profile::Profile::new(user.id);
        profile.update_info(Some(request.name), request.description, request.avatar_url);
        
        self.profile_repository.create(&profile).await.map_err(|e| e.to_string())?;
        
        Ok(ProfileResponse {
            id: profile.id,
            user_id: profile.user_id,
            name: profile.name.unwrap_or_default(),
            description: profile.description,
            avatar_url: profile.avatar_url,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        })
    }
    
    pub async fn update_profile(&self, address: String, request: UpdateProfileRequest) -> Result<ProfileResponse, String> {
        let wallet_address = WalletAddress::new(address).map_err(|e| e.to_string())?;
        let user = self.user_repository
            .find_by_wallet_address(&wallet_address.to_string())
            .await.map_err(|e| e.to_string())?
            .ok_or("User not found")?;
        
        let mut profile = self.profile_repository
            .find_by_user_id(&user.id)
            .await.map_err(|e| e.to_string())?
            .ok_or("Profile not found")?;
        
        profile.update_info(request.name, request.description, request.avatar_url);
        self.profile_repository.update(&profile).await.map_err(|e| e.to_string())?;
        
        Ok(ProfileResponse {
            id: profile.id,
            user_id: profile.user_id,
            name: profile.name.unwrap_or_default(),
            description: profile.description,
            avatar_url: profile.avatar_url,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        })
    }
}
