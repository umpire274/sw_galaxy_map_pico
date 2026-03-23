//! Planet alias queries.

use anyhow::Result;
use rusqlite::Connection;

/// Returns the total number of aliases in the galaxy database.
pub fn count_aliases(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM planet_aliases", [], |row| row.get(0))?;
    Ok(count)
}
