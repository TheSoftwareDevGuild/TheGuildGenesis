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
                    req.extensions_mut()
                        .insert(VerifiedWallet(claims.address));
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
