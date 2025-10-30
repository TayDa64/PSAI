//! Media preview adapters

use anyhow::Result;
use std::path::Path;

/// Preview adapter for media files
pub struct PreviewAdapter {
    // Configuration
}

impl PreviewAdapter {
    pub fn new() -> Self {
        PreviewAdapter {}
    }

    /// Generate preview for file
    pub async fn generate_preview(&self, _input: &Path) -> Result<Vec<u8>> {
        // Stub implementation
        tracing::info!("Generating preview (stub)");
        Ok(vec![])
    }

    /// Check if file type is supported
    pub fn supports(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            matches!(ext.to_str(), Some("jpg") | Some("png") | Some("gif") | Some("mp4") | Some("webm"))
        } else {
            false
        }
    }
}

impl Default for PreviewAdapter {
    fn default() -> Self {
        Self::new()
    }
}
