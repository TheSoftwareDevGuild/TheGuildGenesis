use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::api::AppState;

// What you’ll inject for downstream handlers:
#[derive(Clone, Debug)]
pub struct VerifiedWallet(pub String);

// Expect headers like:
//   X-Eth-Address: 0xabc...
//   X-Eth-Signature: 0xsig...
//   X-SIWE-Message: (optional) full EIP-4361 message string
async fn eth_auth_layer(
    state: AppState,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();

    let addr = headers
        .get("x-eth-address")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sig = headers
        .get("x-eth-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // If you send a SIWE message in a header:
    let siwe_msg = headers.get("x-siwe-message").and_then(|v| v.to_str().ok());

    // Or if you prefer POST body for the login route, you can read it:
    // let bytes: Bytes = axum::body::to_bytes(req.body_mut(), 1024 * 64).await
    //     .map_err(|_| StatusCode::BAD_REQUEST)?;
    // let body_str = std::str::from_utf8(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?;
    // req = Request::from_parts(req.into_parts().0, Body::from(bytes)); // put body back

    // Verify via your AuthService (e.g., SIWE checks: domain, nonce, expiry, signature)
    state
        .auth_service
        .verify(addr, sig, siwe_msg) // define the signature you like
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Inject identity for handlers:
    req.extensions_mut()
        .insert(VerifiedWallet(addr.to_string()));

    Ok(next.run(req).await)
}
