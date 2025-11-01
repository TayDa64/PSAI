//! Artifact management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Artifact metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub kind: String,
    pub path: PathBuf,
    pub created_at: SystemTime,
    pub size_bytes: u64,
    pub bookmarked: bool,
}

impl Artifact {
    pub fn new(id: String, kind: String, path: PathBuf) -> Self {
        let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        Artifact {
            id,
            kind,
            path,
            created_at: SystemTime::now(),
            size_bytes,
            bookmarked: false,
        }
    }

    /// Check if artifact should persist based on retention policy
    pub fn should_persist(&self, policy: &crate::workspace::retention::RetentionPolicy) -> bool {
        if self.bookmarked {
            return true; // Bookmarks always persist
        }

        policy.should_persist(&self.kind)
    }

    /// Check if artifact is expired based on retention policy
    pub fn is_expired(&self, policy: &crate::workspace::retention::RetentionPolicy) -> bool {
        if self.bookmarked {
            return false; // Bookmarks never expire
        }

        if policy.should_persist(&self.kind) {
            return false; // Always-persist types never expire
        }

        // Check age
        let age = SystemTime::now()
            .duration_since(self.created_at)
            .unwrap_or_default();

        age.as_secs() > (policy.days * 24 * 3600) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new(
            "test-id".to_string(),
            "diff".to_string(),
            PathBuf::from("/tmp/test.diff"),
        );

        assert_eq!(artifact.kind, "diff");
        assert!(!artifact.bookmarked);
    }
}
