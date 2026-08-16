use chrono::Utc;
use jsonwebtoken::{ Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn create_access_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    ttl_secs: i64
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        iat: now as usize,
        exp: (now + ttl_secs) as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).map_err(|e|
        AppError::Internal(format!("jwt encode {e}"))
    )
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256)
    )
        .map(|d| d.claims)
        .map_err(|e| AppError::Internal(format!("invalid or expired toke")))
}
