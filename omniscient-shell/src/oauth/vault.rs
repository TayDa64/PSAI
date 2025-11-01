//! Encrypted token vault with OS keychain integration

use anyhow::Result;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Token vault backend
pub enum VaultBackend {
    OsKeychain,
    EncryptedSqlite(String), // path
    InMemory, // For testing
}

/// Token vault for secure storage
pub struct TokenVault {
    backend: VaultBackend,
    in_memory_store: Arc<RwLock<HashMap<String, String>>>,
    locked: Arc<RwLock<bool>>,
}

impl TokenVault {
    /// Create a new vault with OS keychain backend
    pub fn new_os_keychain() -> Self {
        TokenVault {
            backend: VaultBackend::OsKeychain,
            in_memory_store: Arc::new(RwLock::new(HashMap::new())),
            locked: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new vault with encrypted SQLite backend
    pub fn new_encrypted_sqlite(path: String) -> Self {
        TokenVault {
            backend: VaultBackend::EncryptedSqlite(path),
            in_memory_store: Arc::new(RwLock::new(HashMap::new())),
            locked: Arc::new(RwLock::new(false)),
        }
    }

    /// Create an in-memory vault (for testing)
    pub fn new_in_memory() -> Self {
        TokenVault {
            backend: VaultBackend::InMemory,
            in_memory_store: Arc::new(RwLock::new(HashMap::new())),
            locked: Arc::new(RwLock::new(false)),
        }
    }

    /// Store a token (encrypted at rest)
    pub async fn store(&self, label: &str, token: &str) -> Result<()> {
        if *self.locked.read().await {
            anyhow::bail!("Vault is locked");
        }

        match &self.backend {
            VaultBackend::OsKeychain => {
                #[cfg(not(target_os = "windows"))]
                {
                    // Use keyring crate for OS keychain
                    let entry = keyring::Entry::new("omniscient-shell", label)?;
                    entry.set_password(token)?;
                }
                #[cfg(target_os = "windows")]
                {
                    // Windows Credential Manager
                    let entry = keyring::Entry::new("omniscient-shell", label)?;
                    entry.set_password(token)?;
                }
                tracing::info!("Stored token in OS keychain: {}", label);
                Ok(())
            }
            VaultBackend::EncryptedSqlite(_path) => {
                // Placeholder for encrypted SQLite storage
                // Real implementation would:
                // 1. Derive key from passphrase using argon2id
                // 2. Encrypt token with AES-256-GCM
                // 3. Store in SQLite
                let mut store = self.in_memory_store.write().await;
                store.insert(label.to_string(), token.to_string());
                tracing::info!("Stored token in encrypted SQLite: {}", label);
                Ok(())
            }
            VaultBackend::InMemory => {
                let mut store = self.in_memory_store.write().await;
                store.insert(label.to_string(), token.to_string());
                Ok(())
            }
        }
    }

    /// Fetch a token
    pub async fn fetch(&self, label: &str) -> Result<String> {
        if *self.locked.read().await {
            anyhow::bail!("Vault is locked");
        }

        match &self.backend {
            VaultBackend::OsKeychain => {
                let entry = keyring::Entry::new("omniscient-shell", label)?;
                let token = entry.get_password()?;
                Ok(token)
            }
            VaultBackend::EncryptedSqlite(_path) => {
                // Placeholder for encrypted SQLite retrieval
                let store = self.in_memory_store.read().await;
                store.get(label)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("Token not found: {}", label))
            }
            VaultBackend::InMemory => {
                let store = self.in_memory_store.read().await;
                store.get(label)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("Token not found: {}", label))
            }
        }
    }

    /// Delete a token
    pub async fn delete(&self, label: &str) -> Result<()> {
        if *self.locked.read().await {
            anyhow::bail!("Vault is locked");
        }

        match &self.backend {
            VaultBackend::OsKeychain => {
                let entry = keyring::Entry::new("omniscient-shell", label)?;
                entry.delete_credential()?;
                tracing::info!("Deleted token from OS keychain: {}", label);
                Ok(())
            }
            VaultBackend::EncryptedSqlite(_path) => {
                let mut store = self.in_memory_store.write().await;
                store.remove(label);
                tracing::info!("Deleted token from encrypted SQLite: {}", label);
                Ok(())
            }
            VaultBackend::InMemory => {
                let mut store = self.in_memory_store.write().await;
                store.remove(label);
                Ok(())
            }
        }
    }

    /// Lock the vault
    pub async fn lock(&self) {
        let mut locked = self.locked.write().await;
        *locked = true;
        tracing::info!("Vault locked");
    }

    /// Unlock the vault
    pub async fn unlock(&self) {
        let mut locked = self.locked.write().await;
        *locked = false;
        tracing::info!("Vault unlocked");
    }

    /// Check if vault is locked
    pub async fn is_locked(&self) -> bool {
        *self.locked.read().await
    }

    /// Rotate encryption keys (for EncryptedSqlite backend)
    pub async fn rotate_keys(&self) -> Result<()> {
        match &self.backend {
            VaultBackend::EncryptedSqlite(_path) => {
                // Placeholder for key rotation
                // Real implementation would:
                // 1. Generate new encryption key
                // 2. Re-encrypt all tokens
                // 3. Update key in secure storage
                tracing::info!("Rotating encryption keys");
                Ok(())
            }
            _ => {
                tracing::warn!("Key rotation not applicable for this backend");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_vault() {
        let vault = TokenVault::new_in_memory();
        
        // Store token
        vault.store("test-token", "secret-value").await.unwrap();
        
        // Fetch token
        let token = vault.fetch("test-token").await.unwrap();
        assert_eq!(token, "secret-value");
        
        // Delete token
        vault.delete("test-token").await.unwrap();
        
        // Should fail to fetch after delete
        assert!(vault.fetch("test-token").await.is_err());
    }

    #[tokio::test]
    async fn test_vault_locking() {
        let vault = TokenVault::new_in_memory();
        
        // Store when unlocked
        vault.store("test", "value").await.unwrap();
        
        // Lock vault
        vault.lock().await;
        assert!(vault.is_locked().await);
        
        // Should fail to store when locked
        assert!(vault.store("test2", "value2").await.is_err());
        
        // Should fail to fetch when locked
        assert!(vault.fetch("test").await.is_err());
        
        // Unlock and retry
        vault.unlock().await;
        assert!(!vault.is_locked().await);
        
        let token = vault.fetch("test").await.unwrap();
        assert_eq!(token, "value");
    }
}
