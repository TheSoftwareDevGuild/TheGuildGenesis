use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::domain::services::auth_service::AuthChallenge;

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

    let nonce = "NONCE";

    state
        .auth_service
        .verify_signature(
            &AuthChallenge {
                address: address.clone().to_string(),
                nonce: nonce.to_string(),
            },
            &signature,
        ) // define the signature you like
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

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
