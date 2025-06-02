use argon2::Argon2;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use password_hash::{SaltString, rand_core::OsRng};

pub fn hash_password(password: &str) -> Result<String, bool> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(false),
    }
}

#[allow(dead_code)]
pub fn verify_password(
    password: &str,
    hash: &str,
) -> Result<bool, password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = password_hash::PasswordHash::new(hash)?;
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
