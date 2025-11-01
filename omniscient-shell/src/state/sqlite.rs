//! SQLite-backed state storage

use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// SQLite state store
pub struct SqliteStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStore {
    /// Create a new store at the given path
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS kv_store (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS event_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS artifact_index (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                size_bytes INTEGER NOT NULL,
                bookmarked INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        Ok(SqliteStore {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Create an in-memory store
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        
        conn.execute(
            "CREATE TABLE kv_store (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE event_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE artifact_index (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                size_bytes INTEGER NOT NULL,
                bookmarked INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        Ok(SqliteStore {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get connection (for migrations)
    pub async fn connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_store() {
        let store = SqliteStore::in_memory().unwrap();
        // Basic creation test - just verify we can lock the connection
        let _guard = store.conn.lock().await;
        // If we got here, the store was created successfully
    }
}
