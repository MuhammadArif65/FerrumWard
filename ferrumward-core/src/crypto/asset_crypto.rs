use crate::error::{FerrumWardError, Result};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::{rngs::OsRng, Rng};
use sha2::Sha256;

use hkdf::Hkdf;

/// Derives a 256-bit encryption key from the given machine ID and public key.
pub fn derive_asset_key(machine_id: &str, public_key: &[u8]) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(Some(public_key), machine_id.as_bytes());
    let mut okm = [0u8; 32];
    hk.expand(b"ferrumward-asset-key", &mut okm)
        .expect("32 is a valid length for Sha256");
    okm
}

/// Encrypts an asset (plaintext bytes) using the derived key.
/// Returns the IV + Ciphertext.
pub fn encrypt_asset(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| FerrumWardError::TamperDetected)?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| FerrumWardError::TamperDetected)?;

    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypts an asset (IV + Ciphertext bytes) using the derived key.
pub fn decrypt_asset(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    if encrypted_data.len() < 12 {
        return Err(FerrumWardError::TamperDetected);
    }

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| FerrumWardError::TamperDetected)?;

    let nonce = Nonce::from_slice(&encrypted_data[..12]);
    let plaintext = cipher
        .decrypt(nonce, &encrypted_data[12..])
        .map_err(|_| FerrumWardError::TamperDetected)?;

    Ok(plaintext)
}
