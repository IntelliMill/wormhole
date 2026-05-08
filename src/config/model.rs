use serde::{Deserialize, Serialize};

/// A named group that can contain multiple hosts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: u32,
}

/// SSH authentication method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum AuthType {
    Password,
    #[default]
    Key,
    Interactive,
}


/// A single SSH host entry with connection details and display metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub id: String,
    pub display_name: String,
    pub group_id: Option<String>,
    pub address: String,
    pub port: u16,
    pub username: String,
    pub auth_type: AuthType,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub key_path: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub extra_ssh_args: Option<Vec<String>>,
    pub notes: Option<String>,
    #[serde(default)]
    pub sort_order: u32,
}

/// User-configurable application preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub default_port: u16,
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_lang() -> String {
    "zh".to_string()
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "catppuccin_mocha".to_string(),
            default_port: 22,
            lang: default_lang(),
        }
    }
}

/// Root configuration persisted to disk as TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct Config {
    pub master_password_hash: Option<String>,
    pub vault_salt: Option<String>,
    #[serde(default)]
    pub groups: Vec<Group>,
    #[serde(default)]
    pub hosts: Vec<Host>,
    #[serde(default)]
    pub settings: AppSettings,
}


impl Group {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            color: None,
            icon: None,
            sort_order: 0,
        }
    }
}

impl Host {
    pub fn new(display_name: String, address: String, username: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            display_name,
            group_id: None,
            address,
            port: 22,
            username,
            auth_type: AuthType::default(),
            password: None,
            key_path: None,
            color: None,
            icon: None,
            extra_ssh_args: None,
            notes: None,
            sort_order: 0,
        }
    }
}
