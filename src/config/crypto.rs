use std::collections::HashMap;

use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;

const ARGON2_M_COST: u32 = 65536;
const ARGON2_T_COST: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;

type Result<T> = std::result::Result<T, String>;

fn make_argon2() -> Result<Argon2<'static>> {
    let params = argon2::Params::new(ARGON2_M_COST, ARGON2_T_COST, ARGON2_PARALLELISM, Some(32))
        .map_err(|e: argon2::Error| format!("{}", e))?;
    Ok(Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params))
}

/// Derive a 256-bit encryption key from a password and salt using Argon2id.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let argon2 = make_argon2()?;
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;
    Ok(key)
}

/// Generate a random 32-byte salt, hex-encoded for storage.
pub fn generate_salt() -> String {
    use aes_gcm::aead::rand_core::RngCore;
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Produce an Argon2id PHC hash of the master password for verification.
pub fn hash_master_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = make_argon2()?;
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;
    Ok(hash.to_string())
}

/// Check a plain-text password against the stored Argon2id hash.
pub fn verify_master_password(password: &str, hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash).map_err(|e| e.to_string())?;
    let argon2 = make_argon2()?;
    Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
}

/// Encrypt bytes with AES-256-GCM, prepending the random nonce.
pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| e.to_string())?;
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypt AES-256-GCM ciphertext (nonce-prefixed).
pub fn decrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    if data.len() < 12 {
        return Err("Invalid encrypted data".into());
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
    Ok(plaintext)
}

/// Serialize a password map to JSON and encrypt it.
pub fn encrypt_passwords(passwords: &HashMap<String, String>, key: &[u8; 32]) -> Result<Vec<u8>> {
    let json = serde_json::to_string(passwords).map_err(|e| e.to_string())?;
    encrypt(json.as_bytes(), key)
}

/// Decrypt ciphertext and deserialize back to a password map.
pub fn decrypt_passwords(data: &[u8], key: &[u8; 32]) -> Result<HashMap<String, String>> {
    let plaintext = decrypt(data, key)?;
    let map = serde_json::from_slice(&plaintext).map_err(|e| e.to_string())?;
    Ok(map)
}
