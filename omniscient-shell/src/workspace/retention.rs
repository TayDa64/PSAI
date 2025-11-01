//! Retention policies for artifacts

use anyhow::Result;
use std::collections::HashSet;

/// Retention policy
#[derive(Debug, Clone)]
pub struct RetentionPolicy {
    pub always_persist: HashSet<String>,
    pub ephemeral: HashSet<String>,
    pub days: u32,
    pub max_mb: u32,
}

impl RetentionPolicy {
    pub fn new(
        always_persist: Vec<String>,
        ephemeral: Vec<String>,
        days: u32,
        max_mb: u32,
    ) -> Self {
        RetentionPolicy {
            always_persist: always_persist.into_iter().collect(),
            ephemeral: ephemeral.into_iter().collect(),
            days,
            max_mb,
        }
    }

    /// Check if a kind should always persist
    pub fn should_persist(&self, kind: &str) -> bool {
        self.always_persist.contains(kind)
    }

    /// Check if a kind is ephemeral
    pub fn is_ephemeral(&self, kind: &str) -> bool {
        self.ephemeral.contains(kind)
    }

    /// Get TTL in days for a kind
    pub fn ttl(&self, kind: &str) -> Option<u32> {
        if self.should_persist(kind) {
            None // Never expires
        } else {
            Some(self.days)
        }
    }

    /// Prune artifacts based on strategy
    pub async fn prune(&self, strategy: PruneStrategy) -> Result<Vec<String>> {
        // Placeholder for pruning logic
        // Real implementation would:
        // 1. Query artifact index
        // 2. Filter by strategy (size/time)
        // 3. Exclude bookmarks
        // 4. Delete files
        // 5. Return pruned artifact IDs

        tracing::info!("Pruning artifacts with strategy: {:?}", strategy);
        Ok(vec![])
    }
}

/// Pruning strategy
#[derive(Debug, Clone)]
pub enum PruneStrategy {
    ByAge,
    BySize,
    Both,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        RetentionPolicy::new(
            vec!["diff".to_string(), "log".to_string()],
            vec!["preview".to_string(), "scratch".to_string()],
            30,
            1024,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_policy() {
        let policy = RetentionPolicy::default();

        assert!(policy.should_persist("diff"));
        assert!(policy.should_persist("log"));
        assert!(policy.is_ephemeral("preview"));
        assert!(policy.is_ephemeral("scratch"));

        assert_eq!(policy.ttl("diff"), None);
        assert_eq!(policy.ttl("preview"), Some(30));
    }

    #[test]
    fn test_custom_policy() {
        let policy = RetentionPolicy::new(
            vec!["important".to_string()],
            vec!["temp".to_string()],
            7,
            512,
        );

        assert_eq!(policy.days, 7);
        assert_eq!(policy.max_mb, 512);
    }
}
