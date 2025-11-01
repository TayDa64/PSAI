//! Capability-based security model

use anyhow::Result;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Capability identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Capability {
    pub scope: String,  // e.g., "files", "network", "oauth"
    pub action: String, // e.g., "read", "write", "exec"
}

impl Capability {
    pub fn new(scope: impl Into<String>, action: impl Into<String>) -> Self {
        Capability {
            scope: scope.into(),
            action: action.into(),
        }
    }

    /// Parse capability from string (e.g., "files.read")
    pub fn parse(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid capability format: {}", s);
        }
        Ok(Capability {
            scope: parts[0].to_string(),
            action: parts[1].to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}", self.scope, self.action)
    }
}

/// Capability grant with time bounds
#[derive(Debug, Clone)]
pub struct CapabilityGrant {
    pub capability: Capability,
    pub granted_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub revoked: bool,
}

impl CapabilityGrant {
    pub fn new(capability: Capability, duration: Option<Duration>) -> Self {
        let granted_at = SystemTime::now();
        let expires_at = duration.map(|d| granted_at + d);

        CapabilityGrant {
            capability,
            granted_at,
            expires_at,
            revoked: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.revoked {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            if SystemTime::now() > expires_at {
                return false;
            }
        }

        true
    }

    pub fn revoke(&mut self) {
        self.revoked = true;
    }
}

/// Capability manager (default deny)
pub struct CapabilityManager {
    grants: Arc<RwLock<Vec<CapabilityGrant>>>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        CapabilityManager {
            grants: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Grant a capability with optional duration
    pub async fn grant(&self, capability: Capability, duration: Option<Duration>) -> Result<()> {
        let grant = CapabilityGrant::new(capability.clone(), duration);
        let mut grants = self.grants.write().await;
        grants.push(grant);

        tracing::info!("Granted capability: {}", capability.to_string());
        Ok(())
    }

    /// Check if a capability is granted (default deny)
    pub async fn check(&self, capability: &Capability) -> bool {
        let grants = self.grants.read().await;

        grants
            .iter()
            .any(|grant| grant.capability == *capability && grant.is_valid())
    }

    /// Revoke a capability
    pub async fn revoke(&self, capability: &Capability) -> Result<()> {
        let mut grants = self.grants.write().await;

        let mut revoked = false;
        for grant in grants.iter_mut() {
            if grant.capability == *capability && !grant.revoked {
                grant.revoke();
                revoked = true;
            }
        }

        if revoked {
            tracing::info!("Revoked capability: {}", capability.to_string());
            Ok(())
        } else {
            anyhow::bail!(
                "Capability not found or already revoked: {}",
                capability.to_string()
            )
        }
    }

    /// Get all active grants
    pub async fn active_grants(&self) -> Vec<CapabilityGrant> {
        let grants = self.grants.read().await;
        grants.iter().filter(|g| g.is_valid()).cloned().collect()
    }

    /// Cleanup expired grants
    pub async fn cleanup_expired(&self) {
        let mut grants = self.grants.write().await;
        grants.retain(|g| g.is_valid() || !g.revoked);
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_parsing() {
        let cap = Capability::parse("files.read").unwrap();
        assert_eq!(cap.scope, "files");
        assert_eq!(cap.action, "read");
    }

    #[tokio::test]
    async fn test_default_deny() {
        let manager = CapabilityManager::new();
        let cap = Capability::new("files", "read");

        // Should be denied by default
        assert!(!manager.check(&cap).await);
    }

    #[tokio::test]
    async fn test_grant_and_check() {
        let manager = CapabilityManager::new();
        let cap = Capability::new("files", "read");

        manager.grant(cap.clone(), None).await.unwrap();
        assert!(manager.check(&cap).await);
    }

    #[tokio::test]
    async fn test_revoke() {
        let manager = CapabilityManager::new();
        let cap = Capability::new("files", "read");

        manager.grant(cap.clone(), None).await.unwrap();
        assert!(manager.check(&cap).await);

        manager.revoke(&cap).await.unwrap();
        assert!(!manager.check(&cap).await);
    }

    #[tokio::test]
    async fn test_time_bounded() {
        let manager = CapabilityManager::new();
        let cap = Capability::new("network", "connect");

        // Grant for 1 millisecond
        manager
            .grant(cap.clone(), Some(Duration::from_millis(1)))
            .await
            .unwrap();

        // Should be valid immediately
        assert!(manager.check(&cap).await);

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Should be expired
        assert!(!manager.check(&cap).await);
    }
}
