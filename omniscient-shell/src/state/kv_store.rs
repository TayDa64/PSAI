//! Key-value store for agent state

use anyhow::Result;
use rusqlite::params;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::sqlite::SqliteStore;

/// Key-value store
pub struct KVStore {
    store: Arc<SqliteStore>,
}

impl KVStore {
    pub fn new(store: Arc<SqliteStore>) -> Self {
        KVStore { store }
    }

    /// Set a value
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        conn.execute(
            "INSERT OR REPLACE INTO kv_store (key, value, created_at, updated_at) 
             VALUES (?1, ?2, COALESCE((SELECT created_at FROM kv_store WHERE key = ?1), ?3), ?3)",
            params![key, value, now as i64],
        )?;

        Ok(())
    }

    /// Get a value
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        let mut stmt = conn.prepare("SELECT value FROM kv_store WHERE key = ?1")?;
        
        let result = stmt.query_row([key], |row| row.get(0));

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Delete a value
    pub async fn delete(&self, key: &str) -> Result<()> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        conn.execute("DELETE FROM kv_store WHERE key = ?1", params![key])?;

        Ok(())
    }

    /// List all keys
    pub async fn keys(&self) -> Result<Vec<String>> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        let mut stmt = conn.prepare("SELECT key FROM kv_store")?;
        
        let keys: Result<Vec<String>> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into);

        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kv_store() {
        let store = Arc::new(SqliteStore::in_memory().unwrap());
        let kv = KVStore::new(store);

        kv.set("test_key", "test_value").await.unwrap();
        
        let value = kv.get("test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        kv.delete("test_key").await.unwrap();
        
        let value = kv.get("test_key").await.unwrap();
        assert_eq!(value, None);
    }
}
