use crate::application::dtos::profile_dtos::{ProfileResponse, UpdateProfileRequest};
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::value_objects::wallet_address::WalletAddress;
use regex;
use std::sync::Arc;

pub async fn update_profile(
    profile_repository: Arc<dyn ProfileRepository + 'static>,
    address: String,
    request: UpdateProfileRequest,
) -> Result<ProfileResponse, String> {
    let wallet_address = WalletAddress::new(address).map_err(|e| e.to_string())?;

    let mut profile = profile_repository
        .find_by_address(&wallet_address)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Profile not found")?;

    profile.update_info(request.name, request.description, request.avatar_url);
    if let Some(ref handle) = request.github_login {
        let trimmed = handle.trim();

        // Allow empty handles (set to None)
        if trimmed.is_empty() {
            profile.github_login = None;
        } else {
            // Validate format for non-empty handles
            let valid_format = regex::Regex::new(r"^[a-zA-Z0-9-]{1,39}$").unwrap();
            if !valid_format.is_match(trimmed) {
                return Err("Invalid GitHub handle format".to_string());
            }
            if let Some(conflicting_profile) = profile_repository
                .find_by_github_login(trimmed)
                .await
                .map_err(|e| e.to_string())?
            {
                // Only conflict if it's not the current user's profile
                if conflicting_profile.address != wallet_address {
                    return Err("GitHub handle already taken".to_string());
                }
            }
            profile.github_login = Some(trimmed.to_string());
        }
    }
    if let Some(ref handle) = request.twitter_handle {
        let trimmed = handle.trim();

        // Allow empty handles (set to None)
        if trimmed.is_empty() {
            profile.twitter_handle = None;
        } else {
            // Validate format for non-empty handles (Twitter/X handle: 1-15 alphanumeric + underscores)
            let valid_format = regex::Regex::new(r"^[a-zA-Z0-9_]{1,15}$").unwrap();
            if !valid_format.is_match(trimmed) {
                return Err("Invalid Twitter handle format".to_string());
            }
            if let Some(conflicting_profile) = profile_repository
                .find_by_twitter_handle(trimmed)
                .await
                .map_err(|e| e.to_string())?
            {
                // Only conflict if it's not the current user's profile
                if conflicting_profile.address != wallet_address {
                    return Err("Twitter handle already taken".to_string());
                }
            }
            profile.twitter_handle = Some(trimmed.to_string());
        }
    }
    if let Some(ref account) = request.linkedin_account {
        let trimmed = account.trim();

        // Allow empty accounts (set to None)
        if trimmed.is_empty() {
            profile.linkedin_account = None;
        } else {
            // Validate format for non-empty LinkedIn accounts
            let valid_format = regex::Regex::new(r"^[a-zA-Z0-9-]{3,100}$").unwrap();
            if !valid_format.is_match(trimmed) {
                return Err("Invalid LinkedIn account format".to_string());
            }

            let normalized = trimmed.to_lowercase();
            if let Some(conflicting_profile) = profile_repository
                .find_by_linkedin_account(normalized.as_str())
                .await
                .map_err(|e| e.to_string())?
            {
                if conflicting_profile.address != wallet_address {
                    return Err("LinkedIn account already taken".to_string());
                }
            }

            profile.linkedin_account = Some(trimmed.to_string());
        }
    }
    profile_repository
        .update(&profile)
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
