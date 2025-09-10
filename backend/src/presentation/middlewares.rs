use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::domain::services::auth_service::AuthChallenge;

use super::api::AppState;

#[derive(Clone, Debug)]
pub struct VerifiedWallet(pub String);

async fn eth_auth_layer(
    state: AppState,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

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
