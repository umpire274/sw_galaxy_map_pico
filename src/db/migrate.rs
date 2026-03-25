//! Database schema migrations.

use crate::db::meta::meta_set;
use crate::utils::time::now_utc_iso;
use anyhow::{Context, Result};
use rusqlite::Connection;

/// Returns the current SQLite user_version.
pub(crate) fn get_user_version(conn: &Connection) -> Result<i64> {
    let version = conn.query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))?;
    Ok(version)
}

/// Sets the current SQLite user_version.
fn set_user_version(conn: &Connection, version: i64) -> Result<()> {
    conn.execute(&format!("PRAGMA user_version = {version}"), [])
        .context("Failed to update SQLite user_version")?;
    Ok(())
}

/// Migration 1:
/// Adds the route_fingerprint column and unique index to routes.
fn migrate_to_v1(conn: &Connection) -> Result<()> {
    let mut has_column = false;

    {
        let mut stmt = conn.prepare("PRAGMA table_info(routes)")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;

        for row in rows {
            let name = row?;
            if name == "route_fingerprint" {
                has_column = true;
                break;
            }
        }
    }

    if !has_column {
        conn.execute("ALTER TABLE routes ADD COLUMN route_fingerprint TEXT", [])
            .context("Failed to add route_fingerprint column to routes")?;
    }

    conn.execute_batch(
        r#"
        CREATE UNIQUE INDEX IF NOT EXISTS idx_routes_fingerprint
            ON routes(route_fingerprint);
        "#,
    )
    .context("Failed to create unique index on route_fingerprint")?;

    Ok(())
}

fn migrate_to_v2(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS meta (
            key         TEXT PRIMARY KEY,
            value       TEXT NOT NULL,
            updated_utc TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}

fn migrate_to_v3(conn: &Connection) -> Result<()> {
    let mut has_column = false;

    {
        let mut stmt = conn.prepare("PRAGMA table_info(routes)")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;

        for row in rows {
            let name = row?;
            if name == "route_explain_json" {
                has_column = true;
                break;
            }
        }
    }

    if !has_column {
        conn.execute("ALTER TABLE routes ADD COLUMN route_explain_json TEXT", [])?;
    }

    Ok(())
}

/// Applies all pending migrations to the history database.
pub fn migrate_history_db(conn: &Connection) -> Result<()> {
    let version = get_user_version(conn)?;

    print!("[db:migrate] History schema version: {version} .... ");

    if version < 1 {
        migrate_to_v1(conn)?;
        set_user_version(conn, 1)?;
    }

    if version < 2 {
        migrate_to_v2(conn)?;
        set_user_version(conn, 2)?;
    }

    if version < 3 {
        migrate_to_v3(conn)?;
        set_user_version(conn, 3)?;
    }

    let final_version = get_user_version(conn)?;

    // 🔥 aggiorna meta
    let now = now_utc_iso();
    meta_set(
        conn,
        "history_schema_version",
        &final_version.to_string(),
        &now,
    )?;

    println!("Done.");

    Ok(())
}
