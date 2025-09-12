use crate::application::dtos::profile_dtos::ProfileResponse;
use crate::domain::repositories::profile_repository::ProfileRepository;
use std::sync::Arc;

pub async fn get_all_profiles(
    profile_repository: Arc<dyn ProfileRepository + 'static>,
) -> Result<Vec<ProfileResponse>, String> {
    let profiles = profile_repository
        .find_all()
        .await
        .map_err(|e| e.to_string())?;
    

    Ok(profiles.into_iter().map(|profile| ProfileResponse {
        address: profile.address,
        name: profile.name.unwrap_or_default(),
        description: profile.description,
        avatar_url: profile.avatar_url,
        created_at: profile.created_at,
        updated_at: profile.updated_at,
    }).collect())
}
