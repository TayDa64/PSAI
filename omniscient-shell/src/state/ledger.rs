//! Event-sourced ledger

use anyhow::Result;
use rusqlite::params;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::sqlite::SqliteStore;
use crate::agents::event_protocol::Event;

/// Event ledger
pub struct EventLedger {
    store: Arc<SqliteStore>,
}

impl EventLedger {
    pub fn new(store: Arc<SqliteStore>) -> Self {
        EventLedger { store }
    }

    /// Append an event to the ledger
    pub async fn append(&self, event: &Event) -> Result<()> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        let timestamp = event.timestamp
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let event_type = format!("{:?}", event.event_type);
        let data = serde_json::to_string(&event)?;

        conn.execute(
            "INSERT INTO event_log (timestamp, event_type, agent_id, data) VALUES (?1, ?2, ?3, ?4)",
            params![timestamp as i64, event_type, event.agent_id, data],
        )?;

        tracing::debug!("Event appended to ledger: {} from {}", event_type, event.agent_id);
        Ok(())
    }

    /// Get all events for an agent
    pub async fn get_for_agent(&self, agent_id: &str) -> Result<Vec<Event>> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        let mut stmt = conn.prepare(
            "SELECT data FROM event_log WHERE agent_id = ?1 ORDER BY timestamp ASC"
        )?;

        let events: Result<Vec<Event>> = stmt
            .query_map([agent_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?
            .map(|result| {
                let data = result?;
                let event: Event = serde_json::from_str(&data)?;
                Ok(event)
            })
            .collect();

        events
    }

    /// Get recent events (last n)
    pub async fn get_recent(&self, limit: usize) -> Result<Vec<Event>> {
        let conn = self.store.connection().await;
        let conn = conn.lock().await;

        let mut stmt = conn.prepare(
            "SELECT data FROM event_log ORDER BY timestamp DESC LIMIT ?1"
        )?;

        let events: Result<Vec<Event>> = stmt
            .query_map([limit], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?
            .map(|result| {
                let data = result?;
                let event: Event = serde_json::from_str(&data)?;
                Ok(event)
            })
            .collect();

        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_ledger() {
        let store = Arc::new(SqliteStore::in_memory().unwrap());
        let ledger = EventLedger::new(store);

        let event = Event::input("test-agent", "test input".to_string(), 1);
        ledger.append(&event).await.unwrap();

        let events = ledger.get_for_agent("test-agent").await.unwrap();
        assert_eq!(events.len(), 1);
    }
}
