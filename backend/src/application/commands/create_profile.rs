use crate::application::dtos::profile_dtos::{CreateProfileRequest, ProfileResponse};
use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::value_objects::wallet_address::WalletAddress;
use regex;
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

    if let Some(ref account) = request.linkedin_account {
        let trimmed = account.trim();

        if trimmed.is_empty() {
            profile.linkedin_account = None;
        } else {
            let valid_format = regex::Regex::new(r"^[a-zA-Z0-9-]{3,100}$").unwrap();
            if !valid_format.is_match(trimmed) {
                return Err("Invalid LinkedIn account format".to_string());
            }

            let normalized = trimmed.to_lowercase();
            if profile_repository
                .find_by_linkedin_account(normalized.as_str())
                .await
                .map_err(|e| e.to_string())?
                .is_some()
            {
                return Err("LinkedIn account already taken".to_string());
            }

            profile.linkedin_account = Some(trimmed.to_string());
        }
    }

    profile_repository
        .create(&profile)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ProfileResponse {
        address: wallet_address,
        name: profile.name.unwrap_or_default(),
        description: profile.description,
        avatar_url: profile.avatar_url,
        github_login: profile.github_login,
        twitter_handle: profile.twitter_handle,
        linkedin_account: profile.linkedin_account,
        created_at: profile.created_at,
        updated_at: profile.updated_at,
    })
}
