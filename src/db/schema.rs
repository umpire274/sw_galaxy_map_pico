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

/// Initializes the galaxy catalog schema if it does not already exist.
pub fn initialize_galaxy_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS planets (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            remote_id       INTEGER NOT NULL UNIQUE,
            name            TEXT NOT NULL,
            region          TEXT,
            sector          TEXT,
            system_name     TEXT,
            grid            TEXT,
            x               REAL NOT NULL,
            y               REAL NOT NULL,
            canon           INTEGER,
            legends         INTEGER,
            zm              INTEGER,
            name0           TEXT,
            name1           TEXT,
            name2           TEXT,
            lat             REAL,
            long            REAL,
            ref             TEXT,
            status          TEXT,
            c_region        TEXT,
            c_region_li     TEXT
        );

        CREATE TABLE IF NOT EXISTS planets_unknown (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            fid             INTEGER,
            planet          TEXT,
            x               REAL,
            y               REAL,
            reason          TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_planets_remote_id
            ON planets(remote_id);

        CREATE INDEX IF NOT EXISTS idx_planets_name
            ON planets(name);

        CREATE INDEX IF NOT EXISTS idx_planets_unknown_fid
            ON planets_unknown(fid);

        CREATE INDEX IF NOT EXISTS idx_planets_unknown_planet
            ON planets_unknown(planet);
        "#,
    )?;

    Ok(())
}
