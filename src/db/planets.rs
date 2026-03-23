//! Planet catalog queries.

use anyhow::Result;
use rusqlite::Connection;

/// Returns the total number of planets in the galaxy database.
pub fn count_planets(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM planets", [], |row| row.get(0))?;
    Ok(count)
}
