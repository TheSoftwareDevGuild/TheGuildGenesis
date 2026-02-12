use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub address: String,
    pub exp: usize,
}

pub struct JwtManager {
    secret: String,
    expiration: usize,
}

impl JwtManager {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expiration: usize = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string())
            .parse()
            .expect("JWT_EXPIRATION must be a valid number (seconds)");

        JwtManager { secret, expiration }
    }

    pub fn generate_token(&self, address: &str) -> Result<String, String> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            address: address.to_string(),
            exp: now + self.expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| format!("Failed to generate token: {}", e))
    }

    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, String> {
        decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {}", e))
    }
}

impl Default for JwtManager {
    fn default() -> Self {
        Self::new()
    }
}
