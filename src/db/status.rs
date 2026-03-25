use anyhow::Result;
use rusqlite::{Connection, OptionalExtension};
use std::fs;

/// Snapshot of database status and metadata.
#[derive(Debug)]
pub struct DatabaseStatus {
    pub db_path: String,
    pub db_size_bytes: u64,

    pub meta: Vec<(String, String)>,

    pub counts: DatabaseCounts,

    pub schema_objects: Vec<(String, bool)>,

    pub fts_info: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct DatabaseCounts {
    pub planets: i64,
    pub active_planets: Option<i64>,
    pub deleted_planets: Option<i64>,
    pub planets_unknown: Option<i64>,
    pub planet_aliases: Option<i64>,
    pub planet_search: Option<i64>,
}

pub fn collect_database_status(
    history_conn: &Connection,
    galaxy_conn: &Connection,
    db_path: &str,
) -> Result<DatabaseStatus> {
    println!();
    let db_size_bytes = fs::metadata(db_path)?.len();

    // --- META ---
    let mut stmt = history_conn.prepare("SELECT key, value FROM meta ORDER BY key")?;
    let meta_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut meta = Vec::new();
    for row in meta_iter {
        meta.push(row?);
    }

    // --- COUNTS ---
    let has_deleted = column_exists(galaxy_conn, "planets", "deleted")?;

    let planets = galaxy_conn.query_row("SELECT COUNT(*) FROM planets", [], |r| r.get(0))?;

    let active_planets = if has_deleted {
        Some(
            galaxy_conn.query_row("SELECT COUNT(*) FROM planets WHERE deleted = 0", [], |r| {
                r.get(0)
            })?,
        )
    } else {
        None
    };

    let deleted_planets = if has_deleted {
        Some(
            galaxy_conn.query_row("SELECT COUNT(*) FROM planets WHERE deleted = 1", [], |r| {
                r.get(0)
            })?,
        )
    } else {
        None
    };

    let planets_unknown = galaxy_conn
        .query_row("SELECT COUNT(*) FROM planets_unknown", [], |r| r.get(0))
        .optional()?;

    let planet_aliases = galaxy_conn
        .query_row("SELECT COUNT(*) FROM planet_aliases", [], |r| r.get(0))
        .optional()?;

    let planet_search = if table_exists(galaxy_conn, "planet_search")? {
        Some(galaxy_conn.query_row("SELECT COUNT(*) FROM planet_search", [], |r| r.get(0))?)
    } else {
        None
    };

    let counts = DatabaseCounts {
        planets,
        active_planets,
        deleted_planets,
        planets_unknown,
        planet_aliases,
        planet_search,
    };

    // --- SCHEMA ---
    let schema_objects = vec![(
        "v_planets_clean".to_string(),
        table_exists(galaxy_conn, "v_planets_clean")?,
    )];

    // --- FTS ---
    let fts_enabled = meta
        .iter()
        .find(|(k, _)| k == "fts_enabled")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| "0".into());

    let fts_table_exists = table_exists(galaxy_conn, "planets_fts")?;

    let fts_rows = if fts_table_exists {
        galaxy_conn
            .query_row("SELECT COUNT(*) FROM planets_fts", [], |r| r.get(0))
            .unwrap_or(0)
    } else {
        0
    };

    let fts_info = vec![
        ("meta.fts_enabled".into(), fts_enabled),
        (
            "planets_fts table".into(),
            if fts_table_exists {
                "present"
            } else {
                "missing"
            }
            .into(),
        ),
        ("planets_fts rows".into(), fts_rows.to_string()),
    ];

    Ok(DatabaseStatus {
        db_path: db_path.to_string(),
        db_size_bytes,
        meta,
        counts,
        schema_objects,
        fts_info,
    })
}

fn table_exists(conn: &Connection, name: &str) -> Result<bool> {
    let exists: Option<i64> = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE name = ?1 LIMIT 1",
            [name],
            |r| r.get(0),
        )
        .optional()?;

    Ok(exists.is_some())
}

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    let pragma = format!("PRAGMA table_info({table})");
    let mut stmt = conn.prepare(&pragma)?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;

    for row in rows {
        if row? == column {
            return Ok(true);
        }
    }

    Ok(false)
}
