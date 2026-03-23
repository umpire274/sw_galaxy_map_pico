//! SQLite schema helpers.

use anyhow::Result;
use rusqlite::Connection;

/// Initializes the writable history schema if it does not already exist.
pub fn initialize_history_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS route_history (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at      TEXT NOT NULL,
            from_planet_id  INTEGER NOT NULL,
            to_planet_id    INTEGER NOT NULL,
            distance        REAL NOT NULL,
            eta_minutes     INTEGER NOT NULL
        );
        "#,
    )?;

    Ok(())
}
