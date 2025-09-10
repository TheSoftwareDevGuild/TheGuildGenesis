use crate::application::dtos::profile_dtos::ProfileResponse;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::services::AuthService;
use crate::domain::value_objects::wallet_address::WalletAddress;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProfileApplicationService {
    profile_repository: Arc<dyn ProfileRepository + Send + Sync>,
    auth_service: Arc<dyn AuthService + Send + Sync>,
}

pub async fn get_profile(
    profile_repository: Arc<dyn ProfileRepository + 'static>,
    address: String,
) -> Result<ProfileResponse, String> {
    let wallet_address = WalletAddress::new(address).map_err(|e| e.to_string())?;

    let profile = profile_repository
        .find_by_address(&wallet_address)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Profile not found")?;

    Ok(ProfileResponse {
        address: wallet_address,
        name: profile.name.unwrap_or_default(),
        description: profile.description,
        avatar_url: profile.avatar_url,
        created_at: profile.created_at,
        updated_at: profile.updated_at,
    })
}
