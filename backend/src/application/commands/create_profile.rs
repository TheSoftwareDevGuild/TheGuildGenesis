use crate::application::dtos::profile_dtos::{
    CreateProfileRequest, ProfileListResponse, ProfileResponse, UpdateProfileRequest,
};
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::services::AuthService;
use crate::domain::value_objects::wallet_address::WalletAddress;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProfileApplicationService {
    profile_repository: Arc<dyn ProfileRepository + Send + Sync>,
    auth_service: Arc<dyn AuthService + Send + Sync>,
}

pub async fn create_profile(
    &self,
    request: CreateProfileRequest,
) -> Result<ProfileResponse, String> {
    let wallet_address = WalletAddress::new(request.address).map_err(|e| e.to_string())?;
    let user = self
        .user_repository
        .find_by_wallet_address(&wallet_address.to_string())
        .await
        .map_err(|e| e.to_string())?
        .ok_or("User not found")?;

    // Check if profile already exists
    if self
        .profile_repository
        .find_by_user_id(&user.id)
        .await
        .map_err(|e| e.to_string())?
        .is_some()
    {
        return Err("Profile already exists for this user".to_string());
    }

    let mut profile = crate::domain::entities::profile::Profile::new(user.id);
    profile.update_info(Some(request.name), request.description, request.avatar_url);

    self.profile_repository
        .create(&profile)
        .await
        .map_err(|e| e.to_string())?;

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
