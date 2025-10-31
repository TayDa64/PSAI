//! Configuration schema v0.1 with validation and hot-reload support

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Main configuration structure (schema v0.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub workspace: WorkspaceConfig,
    pub graphics: GraphicsConfig,
    pub layout: LayoutConfig,
    pub theme: ThemeConfig,
    pub agents: AgentsConfig,
    pub retention: RetentionConfig,
    #[serde(default)]
    pub oauth: OAuthConfig,
    pub vault: VaultConfig,
    pub notifications: NotificationsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub detection: String, // "explicit"
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default = "default_true")]
    pub auto_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub preferred: String, // "notcurses", "kitty", "overlay"
    pub fallback: Vec<String>,
    #[serde(default = "default_true")]
    pub auto_benchmark: bool,
    #[serde(default)]
    pub legacy_support: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub default: DefaultLayoutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultLayoutConfig {
    pub preset: String, // "dashboard"
    pub panes: Vec<String>, // ["shell", "agent", "preview", "log"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String, // "NeoCyan"
    pub background: String, // "#0b0e10"
    pub foreground: String, // "#c9d1d9"
    pub accent: String, // "#00d1ff"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub enabled: Vec<String>,
    pub sandbox_default: String, // "wasm" or "native"
    #[serde(default)]
    pub native_allowed: Vec<String>,
    pub policy: String, // "user-choice"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionConfig {
    pub always_persist: Vec<String>,
    pub ephemeral: Vec<String>,
    pub days: u32,
    pub max_mb: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthConfig {
    #[serde(default)]
    pub providers: std::collections::HashMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub flow: String, // "device_code" or "pkce"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub backend: String, // "os_keychain" or "encrypted_sqlite"
    #[serde(default = "default_auto_lock")]
    pub auto_lock_minutes: u32,
    pub key_derivation: String, // "argon2id"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsConfig {
    pub profile: String, // "minimal"
    pub channels: Vec<String>, // ["tui", "system"]
}

fn default_true() -> bool {
    true
}

fn default_auto_lock() -> u32 {
    10
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: "0.1".to_string(),
            workspace: WorkspaceConfig {
                detection: "explicit".to_string(),
                root: None,
                auto_save: true,
            },
            graphics: GraphicsConfig {
                preferred: "notcurses".to_string(),
                fallback: vec!["kitty".to_string(), "overlay".to_string()],
                auto_benchmark: true,
                legacy_support: vec![],
            },
            layout: LayoutConfig {
                default: DefaultLayoutConfig {
                    preset: "dashboard".to_string(),
                    panes: vec![
                        "shell".to_string(),
                        "agent".to_string(),
                        "preview".to_string(),
                        "log".to_string(),
                    ],
                },
            },
            theme: ThemeConfig {
                name: "NeoCyan".to_string(),
                background: "#0b0e10".to_string(),
                foreground: "#c9d1d9".to_string(),
                accent: "#00d1ff".to_string(),
            },
            agents: AgentsConfig {
                enabled: vec![],
                sandbox_default: "wasm".to_string(),
                native_allowed: vec![],
                policy: "user-choice".to_string(),
            },
            retention: RetentionConfig {
                always_persist: vec!["diff".to_string(), "log".to_string()],
                ephemeral: vec!["preview".to_string(), "scratch".to_string()],
                days: 30,
                max_mb: 1024,
            },
            oauth: OAuthConfig::default(),
            vault: VaultConfig {
                backend: "os_keychain".to_string(),
                auto_lock_minutes: 10,
                key_derivation: "argon2id".to_string(),
            },
            notifications: NotificationsConfig {
                profile: "minimal".to_string(),
                channels: vec!["tui".to_string()],
            },
        }
    }
}

/// Get the default config path
pub fn default_config_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".omniscient").join("config.toml")
}

/// Load configuration from the default path or create default
pub fn load_config() -> Result<Config> {
    let path = default_config_path();
    load_config_from(&path)
}

/// Load configuration from a specific path
pub fn load_config_from(path: &Path) -> Result<Config> {
    if !path.exists() {
        // Create default config
        let config = Config::default();
        save_config(&config, path)?;
        return Ok(config);
    }

    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: Config = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

    // Validate version
    if config.version != "0.1" {
        anyhow::bail!(
            "Unsupported config version: {}. Expected 0.1. Please update your config file at: {}",
            config.version,
            path.display()
        );
    }

    Ok(config)
}

/// Save configuration to a specific path
pub fn save_config(config: &Config, path: &Path) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let contents = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(path, contents)
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, "0.1");
        assert_eq!(config.graphics.preferred, "notcurses");
        assert_eq!(config.theme.name, "NeoCyan");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.version, parsed.version);
    }
}
