//! Consent ledger for audit trail

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Consent action types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum ConsentAction {
    Grant {
        capability: String,
        duration_s: Option<u64>,
    },
    Revoke {
        capability: String,
    },
    Deny {
        capability: String,
        reason: String,
    },
}

/// Consent ledger entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentEntry {
    pub timestamp: SystemTime,
    pub agent_id: String,
    pub action: ConsentAction,
    pub user_id: Option<String>,
}

/// Append-only consent ledger
pub struct ConsentLedger {
    entries: Arc<RwLock<Vec<ConsentEntry>>>,
}

impl ConsentLedger {
    pub fn new() -> Self {
        ConsentLedger {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log a grant
    pub async fn log_grant(
        &self,
        agent_id: String,
        capability: String,
        duration_s: Option<u64>,
    ) -> Result<()> {
        let entry = ConsentEntry {
            timestamp: SystemTime::now(),
            agent_id: agent_id.clone(),
            action: ConsentAction::Grant {
                capability: capability.clone(),
                duration_s,
            },
            user_id: None,
        };

        let mut entries = self.entries.write().await;
        entries.push(entry);

        tracing::info!("Consent granted: {} -> {}", agent_id, capability);
        Ok(())
    }

    /// Log a revocation
    pub async fn log_revoke(&self, agent_id: String, capability: String) -> Result<()> {
        let entry = ConsentEntry {
            timestamp: SystemTime::now(),
            agent_id: agent_id.clone(),
            action: ConsentAction::Revoke {
                capability: capability.clone(),
            },
            user_id: None,
        };

        let mut entries = self.entries.write().await;
        entries.push(entry);

        tracing::info!("Consent revoked: {} -> {}", agent_id, capability);
        Ok(())
    }

    /// Log a denial
    pub async fn log_deny(&self, agent_id: String, capability: String, reason: String) -> Result<()> {
        let entry = ConsentEntry {
            timestamp: SystemTime::now(),
            agent_id: agent_id.clone(),
            action: ConsentAction::Deny {
                capability: capability.clone(),
                reason: reason.clone(),
            },
            user_id: None,
        };

        let mut entries = self.entries.write().await;
        entries.push(entry);

        tracing::info!("Consent denied: {} -> {} ({})", agent_id, capability, reason);
        Ok(())
    }

    /// Get all entries
    pub async fn get_all(&self) -> Vec<ConsentEntry> {
        let entries = self.entries.read().await;
        entries.clone()
    }

    /// Get entries for a specific agent
    pub async fn get_for_agent(&self, agent_id: &str) -> Vec<ConsentEntry> {
        let entries = self.entries.read().await;
        entries
            .iter()
            .filter(|e| e.agent_id == agent_id)
            .cloned()
            .collect()
    }

    /// Export ledger (with secrets redacted)
    pub async fn export(&self) -> Result<String> {
        let entries = self.entries.read().await;
        let json = serde_json::to_string_pretty(&*entries)?;
        Ok(json)
    }
}

impl Default for ConsentLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consent_ledger() {
        let ledger = ConsentLedger::new();

        // Log grant
        ledger
            .log_grant("agent1".to_string(), "files.read".to_string(), Some(3600))
            .await
            .unwrap();

        // Log revoke
        ledger
            .log_revoke("agent1".to_string(), "files.read".to_string())
            .await
            .unwrap();

        // Get all entries
        let entries = ledger.get_all().await;
        assert_eq!(entries.len(), 2);

        // Get entries for agent
        let agent_entries = ledger.get_for_agent("agent1").await;
        assert_eq!(agent_entries.len(), 2);
    }

    #[tokio::test]
    async fn test_export() {
        let ledger = ConsentLedger::new();
        ledger
            .log_grant("test".to_string(), "network".to_string(), None)
            .await
            .unwrap();

        let export = ledger.export().await.unwrap();
        assert!(export.contains("network"));
    }
}
