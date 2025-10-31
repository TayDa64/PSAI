//! Database migrations

use anyhow::Result;
use rusqlite::Connection;

/// Migration version
const CURRENT_VERSION: i32 = 1;

/// Run migrations
pub fn migrate(conn: &mut Connection) -> Result<()> {
    // Create schema_version table if not exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;

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
        // Add future migrations here:
        // if version < 2 {
        //     migrate_to_v2(conn)?;
        // }
    }

    Ok(())
}

fn migrate_to_v1(conn: &mut Connection) -> Result<()> {
    tracing::info!("Migrating to schema version 1");
    
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

/// Check if database needs migration
pub fn needs_migration(conn: &Connection) -> Result<bool> {
    let version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(version < CURRENT_VERSION)
}

/// Get current schema version
pub fn current_version(conn: &Connection) -> Result<i32> {
    let version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(version)
}

/// Rollback to a specific version (use with caution!)
pub fn rollback_to_version(conn: &mut Connection, target_version: i32) -> Result<()> {
    let current = current_version(conn)?;
    
    if target_version >= current {
        anyhow::bail!("Cannot rollback to version {} (current: {})", target_version, current);
    }

    tracing::warn!("Rolling back from version {} to {}", current, target_version);

    // Delete migrations after target version
    conn.execute(
        "DELETE FROM schema_version WHERE version > ?1",
        [target_version],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migration() {
        let mut conn = Connection::open_in_memory().unwrap();
        
        // Should need migration initially
        migrate(&mut conn).unwrap();
        
        // Should not need migration after running
        assert!(!needs_migration(&conn).unwrap());
        
        // Should be at current version
        assert_eq!(current_version(&conn).unwrap(), CURRENT_VERSION);
    }

    #[test]
    fn test_version_check() {
        let mut conn = Connection::open_in_memory().unwrap();
        
        migrate(&mut conn).unwrap();
        
        let version = current_version(&conn).unwrap();
        assert_eq!(version, CURRENT_VERSION);
    }
}
