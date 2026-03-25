//! Simple key-value metadata storage for the history database.

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, params};

#[allow(dead_code)]
/// Retrieves a metadata value by key.
pub fn meta_get(conn: &Connection, key: &str) -> Result<Option<String>> {
    let value = conn
        .query_row("SELECT value FROM meta WHERE key = ?1", [key], |row| {
            row.get::<_, String>(0)
        })
        .optional()?;

    Ok(value)
}

/// Inserts or updates a metadata value.
pub fn meta_set(conn: &Connection, key: &str, value: &str, updated_utc: &str) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO meta (key, value, updated_utc)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(key) DO UPDATE SET
            value = excluded.value,
            updated_utc = excluded.updated_utc
        "#,
        params![key, value, updated_utc],
    )?;

    Ok(())
}

/// Inserts a metadata value only if the key is not already present.
pub fn meta_set_if_absent(
    conn: &Connection,
    key: &str,
    value: &str,
    updated_utc: &str,
) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO meta (key, value, updated_utc)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(key) DO NOTHING
        "#,
        params![key, value, updated_utc],
    )?;

    Ok(())
}
