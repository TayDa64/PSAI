//! Agent registry for discovering and managing agents

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agents::manifest::{Manifest, SandboxMode};

/// Agent information
#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub manifest: Manifest,
    pub base_dir: PathBuf,
    pub enabled: bool,
}

/// Agent registry
pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<String, AgentInfo>>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        AgentRegistry {
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an agent from a directory
    pub async fn register(&self, agent_dir: &Path) -> Result<()> {
        let manifest_path = agent_dir.join("manifest.toml");

        if !manifest_path.exists() {
            anyhow::bail!("No manifest.toml found in {}", agent_dir.display());
        }

        let manifest = Manifest::load(&manifest_path).with_context(|| {
            format!("Failed to load agent manifest from {}", agent_dir.display())
        })?;

        let agent_info = AgentInfo {
            manifest: manifest.clone(),
            base_dir: agent_dir.to_path_buf(),
            enabled: true,
        };

        let mut agents = self.agents.write().await;
        agents.insert(manifest.name.clone(), agent_info);

        tracing::info!("Registered agent: {} v{}", manifest.name, manifest.version);
        Ok(())
    }

    /// Get an agent by name
    pub async fn get(&self, name: &str) -> Option<AgentInfo> {
        let agents = self.agents.read().await;
        agents.get(name).cloned()
    }

    /// List all registered agents
    pub async fn list(&self) -> Vec<AgentInfo> {
        let agents = self.agents.read().await;
        agents.values().cloned().collect()
    }

    /// Enable or disable an agent
    pub async fn set_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        let mut agents = self.agents.write().await;
        if let Some(info) = agents.get_mut(name) {
            info.enabled = enabled;
            Ok(())
        } else {
            anyhow::bail!("Agent not found: {}", name)
        }
    }

    /// Discover agents from a directory
    pub async fn discover(&self, agents_dir: &Path) -> Result<()> {
        if !agents_dir.exists() {
            tracing::warn!("Agents directory does not exist: {}", agents_dir.display());
            return Ok(());
        }

        let entries = std::fs::read_dir(agents_dir).with_context(|| {
            format!("Failed to read agents directory: {}", agents_dir.display())
        })?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("manifest.toml");
                if manifest_path.exists() {
                    match self.register(&path).await {
                        Ok(()) => {}
                        Err(e) => {
                            tracing::warn!("Failed to register agent in {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get agents by sandbox mode
    pub async fn get_by_sandbox(&self, mode: SandboxMode) -> Vec<AgentInfo> {
        let agents = self.agents.read().await;
        agents
            .values()
            .filter(|info| info.manifest.sandbox == mode && info.enabled)
            .cloned()
            .collect()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_basic() {
        let registry = AgentRegistry::new();
        let agents = registry.list().await;
        assert_eq!(agents.len(), 0);
    }
}
