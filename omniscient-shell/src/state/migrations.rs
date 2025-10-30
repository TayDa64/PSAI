//! Database migrations

use anyhow::Result;
use rusqlite::Connection;

/// Migration version
const CURRENT_VERSION: i32 = 1;

/// Run migrations
pub fn migrate(conn: &mut Connection) -> Result<()> {
    // Get current version
    let version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if version < CURRENT_VERSION {
        tracing::info!("Running migrations from version {} to {}", version, CURRENT_VERSION);
        
        // Run migrations based on current version
        if version < 1 {
            migrate_to_v1(conn)?;
        }
    }

    Ok(())
}

fn migrate_to_v1(conn: &mut Connection) -> Result<()> {
    tracing::info!("Migrating to schema version 1");
    
    // Create schema_version table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;

    // Record migration
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    conn.execute(
        "INSERT INTO schema_version (version, applied_at) VALUES (?1, ?2)",
        [1, now as i32],
    )?;

    Ok(())
}
