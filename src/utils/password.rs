use argon2::{ Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString };
use rand::rngs::OsRng;

use crate::utils::AppError;

pub fn hash_password(plan_password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(plan_password.as_bytes(), &salt)
        .map(|text| text.to_string())
        .map_err(|e| AppError::Internal(format!("has password {e}")))
}

pub fn verify_password(plain_password: &str, hash: &str) -> Result<bool, AppError> {
    let parshed = PasswordHash::new(hash).map_err(|e| AppError::Internal(format!("bad hash {e}")))?;

    Ok(Argon2::default().verify_password(plain_password.as_bytes(), &parshed).is_ok())
}
