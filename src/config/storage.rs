use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::config::crypto;
use crate::config::model::Config;

/// Returns the `~/.wormhole` directory used for all persistent data.
pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Cannot find home directory")
        .join(".wormhole")
}

/// Path to the TOML config file.
pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

/// Path to the encrypted password vault.
pub fn vault_path() -> PathBuf {
    config_dir().join("vault.enc")
}

/// Path to the plain-text passwords JSON file (used when no master password is set).
pub fn plain_passwords_path() -> PathBuf {
    config_dir().join("passwords.json")
}

/// Creates the config directory if it does not exist.
pub fn ensure_config_dir() -> Result<(), String> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Load config from disk, returning defaults when the file is absent.
pub fn load_config() -> Result<Config, String> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let config: Config = toml::from_str(&content).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Serialize and persist config to disk.
pub fn save_config(config: &Config) -> Result<(), String> {
    ensure_config_dir()?;
    let content = toml::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_path(), content).map_err(|e| e.to_string())?;
    Ok(())
}

/// Decrypt and load the password vault.
pub fn load_vault(key: &[u8; 32]) -> Result<HashMap<String, String>, String> {
    let path = vault_path();
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let data = fs::read(&path).map_err(|e| e.to_string())?;
    if data.is_empty() {
        return Ok(HashMap::new());
    }
    crypto::decrypt_passwords(&data, key)
}

/// Encrypt and persist the password vault.
pub fn save_vault(passwords: &HashMap<String, String>, key: &[u8; 32]) -> Result<(), String> {
    ensure_config_dir()?;
    let data = crypto::encrypt_passwords(passwords, key)?;
    fs::write(vault_path(), data).map_err(|e| e.to_string())?;
    Ok(())
}

/// Load passwords stored as plain-text JSON.
pub fn load_plain_passwords() -> Result<HashMap<String, String>, String> {
    let path = plain_passwords_path();
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

/// Persist passwords as plain-text JSON (no master password).
pub fn save_plain_passwords(passwords: &HashMap<String, String>) -> Result<(), String> {
    ensure_config_dir()?;
    let content = serde_json::to_string_pretty(passwords).map_err(|e| e.to_string())?;
    fs::write(plain_passwords_path(), content).map_err(|e| e.to_string())?;
    Ok(())
}

/// Returns `true` when no config file exists yet.
pub fn is_first_run() -> bool {
    !config_path().exists()
}

/// Delete all persisted data (config, vault, and plain-text passwords).
pub fn reset_all() {
    let _ = fs::remove_file(config_path());
    let _ = fs::remove_file(vault_path());
    let _ = fs::remove_file(plain_passwords_path());
}
