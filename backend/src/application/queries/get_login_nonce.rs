use std::sync::Arc;

use crate::domain::{repositories::ProfileRepository, value_objects::WalletAddress};

pub async fn get_login_nonce(
    profile_repository: Arc<dyn ProfileRepository>,
    address: String,
) -> Result<i64, String> {
    let wallet_address = WalletAddress(address);

    match profile_repository
        .get_login_nonce_by_wallet_address(&wallet_address)
        .await
    {
        Ok(Some(nonce)) => Ok(nonce),
        Ok(None) => Err("Profile not found".to_string()),
        Err(e) => Err(format!("Error fetching nonce: {}", e)),
    }
}
