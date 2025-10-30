//! Agent manifest schema v0.1

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Agent manifest (schema v0.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: String,
    pub name: String,
    pub version: String,
    pub entry: String,
    pub sandbox: SandboxMode,
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub oauth_scopes: Vec<String>,
    pub resources: ResourceLimits,
    pub ui: UiHints,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SandboxMode {
    Wasm,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu: String,  // e.g., "500m"
    pub mem: String,  // e.g., "512Mi"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiHints {
    pub hints: Vec<String>,  // e.g., ["streaming", "diff", "preview"]
}

impl Manifest {
    /// Load manifest from a file
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest: {}", path.display()))?;

        let manifest: Manifest = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse manifest: {}", path.display()))?;

        manifest.validate()?;
        Ok(manifest)
    }

    /// Validate manifest
    pub fn validate(&self) -> Result<()> {
        // Check schema version
        if self.schema_version != "0.1" {
            anyhow::bail!(
                "Unsupported manifest schema version: {}. Expected 0.1",
                self.schema_version
            );
        }

        // Validate entry point exists
        if self.entry.is_empty() {
            anyhow::bail!("Manifest entry point cannot be empty");
        }

        // Validate capabilities format
        for cap in &self.capabilities {
            if !cap.contains('.') && !cap.contains(':') {
                tracing::warn!("Capability '{}' may not follow standard format", cap);
            }
        }

        Ok(())
    }

    /// Get the full path to the entry point
    pub fn entry_path(&self, base_dir: &Path) -> PathBuf {
        base_dir.join(&self.entry)
    }

    /// Check if the agent requires native execution
    pub fn requires_native(&self) -> bool {
        self.sandbox == SandboxMode::Native
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_manifest_parsing() {
        let manifest_toml = r#"
schema_version = "0.1"
name = "Test Agent"
version = "0.1.0"
entry = "agent.wasm"
sandbox = "wasm"
capabilities = ["files.read", "network"]
oauth_scopes = ["github:repo"]

[resources]
cpu = "500m"
mem = "512Mi"

[ui]
hints = ["streaming"]
"#;

        let manifest: Manifest = toml::from_str(manifest_toml).unwrap();
        assert_eq!(manifest.schema_version, "0.1");
        assert_eq!(manifest.name, "Test Agent");
        assert_eq!(manifest.sandbox, SandboxMode::Wasm);
    }

    #[test]
    fn test_manifest_validation() {
        let manifest = Manifest {
            schema_version: "0.1".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            entry: "test.wasm".to_string(),
            sandbox: SandboxMode::Wasm,
            capabilities: vec!["files.read".to_string()],
            oauth_scopes: vec![],
            resources: ResourceLimits {
                cpu: "500m".to_string(),
                mem: "512Mi".to_string(),
            },
            ui: UiHints {
                hints: vec!["streaming".to_string()],
            },
        };

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_invalid_schema_version() {
        let mut manifest = Manifest {
            schema_version: "0.2".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            entry: "test.wasm".to_string(),
            sandbox: SandboxMode::Wasm,
            capabilities: vec![],
            oauth_scopes: vec![],
            resources: ResourceLimits {
                cpu: "500m".to_string(),
                mem: "512Mi".to_string(),
            },
            ui: UiHints { hints: vec![] },
        };

        assert!(manifest.validate().is_err());
    }
}
