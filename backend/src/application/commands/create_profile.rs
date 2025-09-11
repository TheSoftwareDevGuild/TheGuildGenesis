use crate::application::dtos::profile_dtos::{CreateProfileRequest, ProfileResponse};
use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::value_objects::wallet_address::WalletAddress;
use std::sync::Arc;

pub async fn create_profile(
    profile_repository: Arc<dyn ProfileRepository + 'static>,
    address: String,
    request: CreateProfileRequest,
) -> Result<ProfileResponse, String> {
    let wallet_address = WalletAddress::new(address).map_err(|e| e.to_string())?;

    // Check if profile already exists
    if profile_repository
        .find_by_address(&wallet_address)
        .await
        .map_err(|e| e.to_string())?
        .is_some()
    {
        return Err("Profile already exists for this user".to_string());
    }

    let mut profile = Profile::new(wallet_address.clone());
    profile.update_info(Some(request.name), request.description, request.avatar_url);

    profile_repository
        .create(&profile)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ProfileResponse {
        address: wallet_address,
        name: profile.name.unwrap_or_default(),
        description: profile.description,
        avatar_url: profile.avatar_url,
        created_at: profile.created_at,
        updated_at: profile.updated_at,
    })
}
