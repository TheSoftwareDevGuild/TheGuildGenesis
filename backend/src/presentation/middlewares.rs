use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::domain::services::auth_service::AuthChallenge;
use crate::infrastructure::jwt::JwtManager;

use super::api::AppState;

#[derive(Clone, Debug)]
pub struct VerifiedWallet(pub String);

pub async fn eth_auth_layer(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    // Bypass auth in test mode
    if std::env::var("TEST_MODE").is_ok() {
        let address = headers
            .get("x-eth-address")
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned)
            .unwrap_or_else(|| "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string());
        req.extensions_mut().insert(VerifiedWallet(address));
        return Ok(next.run(req).await);
    }

    // Try JWT token first
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let jwt_manager = JwtManager::new();
                if let Ok(claims) = jwt_manager.validate_token(token) {
                    req.extensions_mut().insert(VerifiedWallet(claims.address));
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    // Fall back to signature verification
    let address = headers
        .get("x-eth-address")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let signature = headers
        .get("x-eth-signature")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Get the current nonce from the database
    let wallet_address = crate::domain::value_objects::WalletAddress(address.clone());
    let nonce = state
        .profile_repository
        .get_login_nonce_by_wallet_address(&wallet_address)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(1); // Use default nonce if profile doesn't exist

    let result = state
        .auth_service
        .verify_signature(
            &AuthChallenge {
                address: address.clone().to_string(),
                nonce,
            },
            &signature,
        )
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if result.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Inject identity for handlers:
    req.extensions_mut()
        .insert(VerifiedWallet(address.to_string()));

    Ok(next.run(req).await)
}

pub async fn test_auth_layer(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let address = headers
        .get("x-eth-address")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
        .unwrap_or_else(|| "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string());
    req.extensions_mut().insert(VerifiedWallet(address));
    Ok(next.run(req).await)
}

/// Middleware to verify admin wallet addresses.
/// Admin addresses are configured via the ADMIN_ADDRESSES environment variable
/// as a comma-separated list of wallet addresses.
pub async fn admin_auth_layer(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get admin addresses from environment variable
    let admin_addresses = std::env::var("ADMIN_ADDRESSES").unwrap_or_default();
    let admin_list: Vec<&str> = admin_addresses
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if admin_list.is_empty() {
        tracing::warn!("No admin addresses configured. Set ADMIN_ADDRESSES env variable.");
        return Err(StatusCode::FORBIDDEN);
    }

    // First, authenticate the user using existing eth_auth logic
    let headers = req.headers();

    // Try JWT token first
    let verified_address = if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let jwt_manager = JwtManager::new();
                jwt_manager
                    .validate_token(token)
                    .ok()
                    .map(|claims| claims.address)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Fall back to signature verification if no JWT
    let verified_address = match verified_address {
        Some(addr) => addr,
        None => {
            let address = headers
                .get("x-eth-address")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
                .ok_or(StatusCode::UNAUTHORIZED)?;

            let signature = headers
                .get("x-eth-signature")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
                .ok_or(StatusCode::UNAUTHORIZED)?;

            let wallet_address = crate::domain::value_objects::WalletAddress(address.clone());
            let nonce = state
                .profile_repository
                .get_login_nonce_by_wallet_address(&wallet_address)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .unwrap_or(1);

            let result = state
                .auth_service
                .verify_signature(
                    &AuthChallenge {
                        address: address.clone(),
                        nonce,
                    },
                    &signature,
                )
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

            if result.is_none() {
                return Err(StatusCode::UNAUTHORIZED);
            }

            address
        }
    };

    // Check if the verified address is in the admin list (case-insensitive)
    let is_admin = admin_list
        .iter()
        .any(|admin| admin.eq_ignore_ascii_case(&verified_address));

    if !is_admin {
        tracing::warn!("Access denied: {} is not an admin", verified_address);
        return Err(StatusCode::FORBIDDEN);
    }

    req.extensions_mut()
        .insert(VerifiedWallet(verified_address));

    Ok(next.run(req).await)
}
