use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("encryption failed")]
    Encrypt,
    #[error("decryption failed")]
    Decrypt,
    #[error("invalid key length")]
    InvalidKeyLength,
    #[error("invalid nonce length")]
    InvalidNonceLength,
}

const XCHACHA_NONCE_LEN: usize = 24;
const AESGCM_NONCE_LEN: usize = 12;

/// Encrypt data using XChaCha20-Poly1305.
pub fn xchacha20_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CipherError> {
    let cipher =
        XChaCha20Poly1305::new_from_slice(key).map_err(|_| CipherError::InvalidKeyLength)?;
    let nonce_bytes = generate_nonce::<XCHACHA_NONCE_LEN>();
    let nonce = XNonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CipherError::Encrypt)?;
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypt data using XChaCha20-Poly1305.
pub fn xchacha20_decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, CipherError> {
    if data.len() < XCHACHA_NONCE_LEN {
        return Err(CipherError::InvalidNonceLength);
    }
    let (nonce_bytes, ciphertext) = data.split_at(XCHACHA_NONCE_LEN);
    let cipher =
        XChaCha20Poly1305::new_from_slice(key).map_err(|_| CipherError::InvalidKeyLength)?;
    let nonce = XNonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CipherError::Decrypt)
}

/// Encrypt data using AES-256-GCM.
pub fn aes256gcm_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CipherError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CipherError::InvalidKeyLength)?;
    let nonce_bytes = generate_nonce::<AESGCM_NONCE_LEN>();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CipherError::Encrypt)?;
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypt data using AES-256-GCM.
pub fn aes256gcm_decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, CipherError> {
    if data.len() < AESGCM_NONCE_LEN {
        return Err(CipherError::InvalidNonceLength);
    }
    let (nonce_bytes, ciphertext) = data.split_at(AESGCM_NONCE_LEN);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CipherError::InvalidKeyLength)?;
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CipherError::Decrypt)
}

fn generate_nonce<const N: usize>() -> [u8; N] {
    let mut nonce = [0u8; N];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xchacha20_encrypt_then_decrypt_returns_original() {
        let key = [0x42u8; 32];
        let plaintext = b"Hello, Midnight Notes!";
        let encrypted = xchacha20_encrypt(&key, plaintext).unwrap();
        let decrypted = xchacha20_decrypt(&key, &encrypted).unwrap();
        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn xchacha20_with_wrong_key_fails_decryption() {
        let key1 = [0x42u8; 32];
        let key2 = [0x24u8; 32];
        let plaintext = b"secret data";
        let encrypted = xchacha20_encrypt(&key1, plaintext).unwrap();
        assert!(xchacha20_decrypt(&key2, &encrypted).is_err());
    }

    #[test]
    fn xchacha20_produces_different_ciphertext_for_same_plaintext() {
        let key = [0x42u8; 32];
        let plaintext = b"same data";
        let e1 = xchacha20_encrypt(&key, plaintext).unwrap();
        let e2 = xchacha20_encrypt(&key, plaintext).unwrap();
        assert_ne!(e1, e2);
    }

    #[test]
    fn aes256gcm_encrypt_then_decrypt_returns_original() {
        let key = [0x42u8; 32];
        let plaintext = b"Hello, AES-256-GCM!";
        let encrypted = aes256gcm_encrypt(&key, plaintext).unwrap();
        let decrypted = aes256gcm_decrypt(&key, &encrypted).unwrap();
        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn encrypting_and_decrypting_empty_data_succeeds() {
        let key = [0x42u8; 32];
        let encrypted = xchacha20_encrypt(&key, b"").unwrap();
        let decrypted = xchacha20_decrypt(&key, &encrypted).unwrap();
        assert_eq!(decrypted.len(), 0);
    }
}
