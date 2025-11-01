//! Workspace selection and management

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Workspace provider
pub struct Workspace {
    root: Arc<RwLock<Option<PathBuf>>>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            root: Arc::new(RwLock::new(None)),
        }
    }

    /// Select a workspace (explicit selection required)
    pub async fn select(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            anyhow::bail!("Workspace path does not exist: {}", path.display());
        }

        if !path.is_dir() {
            anyhow::bail!("Workspace path is not a directory: {}", path.display());
        }

        let mut root = self.root.write().await;
        *root = Some(path.to_path_buf());

        tracing::info!("Workspace selected: {}", path.display());
        Ok(())
    }

    /// Get current workspace root
    pub async fn root(&self) -> Option<PathBuf> {
        let root = self.root.read().await;
        root.clone()
    }

    /// Check if a workspace is selected
    pub async fn is_selected(&self) -> bool {
        let root = self.root.read().await;
        root.is_some()
    }

    /// Resolve artifact path within workspace
    pub async fn resolve_artifact_path(&self, kind: &str, name: &str) -> Result<PathBuf> {
        let root = self.root.read().await;
        let root = root.as_ref().ok_or_else(|| {
            anyhow::anyhow!("No workspace selected. Use 'omni:workspace select <path>'")
        })?;

        // Create .omniscient directory in workspace
        let omni_dir = root.join(".omniscient");
        std::fs::create_dir_all(&omni_dir).with_context(|| {
            format!(
                "Failed to create .omniscient directory: {}",
                omni_dir.display()
            )
        })?;

        // Create kind-specific subdirectory
        let kind_dir = omni_dir.join(kind);
        std::fs::create_dir_all(&kind_dir).with_context(|| {
            format!(
                "Failed to create {} directory: {}",
                kind,
                kind_dir.display()
            )
        })?;

        Ok(kind_dir.join(name))
    }

    /// List all artifact types in workspace
    pub async fn list_artifact_types(&self) -> Result<Vec<String>> {
        let root = self.root.read().await;
        let root = root
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No workspace selected"))?;

        let omni_dir = root.join(".omniscient");
        if !omni_dir.exists() {
            return Ok(vec![]);
        }

        let mut types = Vec::new();
        for entry in std::fs::read_dir(&omni_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    types.push(name.to_string());
                }
            }
        }

        Ok(types)
    }

    /// Clear workspace selection
    pub async fn clear(&self) {
        let mut root = self.root.write().await;
        *root = None;
        tracing::info!("Workspace selection cleared");
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_workspace_selection() {
        let workspace = Workspace::new();
        assert!(!workspace.is_selected().await);

        let temp_dir = TempDir::new().unwrap();
        workspace.select(temp_dir.path()).await.unwrap();

        assert!(workspace.is_selected().await);
        assert_eq!(workspace.root().await.unwrap(), temp_dir.path());
    }

    #[tokio::test]
    async fn test_artifact_path_resolution() {
        let workspace = Workspace::new();
        let temp_dir = TempDir::new().unwrap();
        workspace.select(temp_dir.path()).await.unwrap();

        let path = workspace
            .resolve_artifact_path("diff", "test.diff")
            .await
            .unwrap();
        assert!(path
            .to_string_lossy()
            .contains(".omniscient/diff/test.diff"));
    }

    #[tokio::test]
    async fn test_no_workspace_selected() {
        let workspace = Workspace::new();
        let result = workspace.resolve_artifact_path("diff", "test.diff").await;
        assert!(result.is_err());
    }
}
