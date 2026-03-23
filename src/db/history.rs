//! Route history queries.

use anyhow::Result;
use rusqlite::Connection;

/// Returns the total number of history entries in the writable database.
pub fn count_history_entries(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM route_history", [], |row| row.get(0))?;
    Ok(count)
}
