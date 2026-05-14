use argon2::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHasher, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("argon2 error: {0}")]
    Argon2(String),
    #[error("password too short")]
    WeakPassword,
}

impl From<argon2::password_hash::Error> for KeychainError {
    fn from(e: argon2::password_hash::Error) -> Self {
        KeychainError::Argon2(e.to_string())
    }
}

impl From<argon2::Error> for KeychainError {
    fn from(e: argon2::Error) -> Self {
        KeychainError::Argon2(e.to_string())
    }
}

const SALT_LENGTH: usize = 16;
const KEY_LENGTH: usize = 32;
const MEMORY_COST: u32 = 65536;
const TIME_COST: u32 = 3;
const PARALLELISM: u32 = 4;

/// Derive a 256-bit encryption key from a master password using Argon2id.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEY_LENGTH], KeychainError> {
    let params = Params::new(MEMORY_COST, TIME_COST, PARALLELISM, Some(KEY_LENGTH))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; KEY_LENGTH];
    let salt_str = SaltString::encode_b64(salt)?;
    let hash = argon2.hash_password(password.as_bytes(), &salt_str)?;
    let hash_bytes = hash.hash.unwrap();
    key.copy_from_slice(hash_bytes.as_bytes());
    Ok(key)
}

/// Generate a random 16-byte salt.
pub fn generate_salt() -> [u8; SALT_LENGTH] {
    let mut salt = [0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Validate password meets minimum length requirement.
pub fn validate_password_strength(password: &str) -> Result<(), KeychainError> {
    if password.len() < 8 {
        return Err(KeychainError::WeakPassword);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_deterministic() {
        let salt = generate_salt();
        let key1 = derive_key("testpassword", &salt).unwrap();
        let key2 = derive_key("testpassword", &salt).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_different_salt() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        let key1 = derive_key("testpassword", &salt1).unwrap();
        let key2 = derive_key("testpassword", &salt2).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_key_length() {
        let salt = generate_salt();
        let key = derive_key("testpassword", &salt).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_generate_salt_length() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 16);
    }

    #[test]
    fn test_generate_salt_unique() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_validate_password_strength_valid() {
        assert!(validate_password_strength("longenoughpassword").is_ok());
    }

    #[test]
    fn test_validate_password_strength_short() {
        assert!(validate_password_strength("short").is_err());
    }
}
