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

        CREATE TABLE IF NOT EXISTS planet_aliases (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            planet_fid      INTEGER NOT NULL,
            alias           TEXT NOT NULL,
            alias_norm      TEXT NOT NULL,
            source          TEXT NOT NULL,
            UNIQUE(planet_fid, alias)
        );

        CREATE INDEX IF NOT EXISTS idx_planet_aliases_alias_norm
            ON planet_aliases(alias_norm);

        CREATE TABLE IF NOT EXISTS waypoints (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            code            TEXT NOT NULL UNIQUE,
            label           TEXT,
            x               REAL NOT NULL,
            y               REAL NOT NULL,
            kind            TEXT NOT NULL DEFAULT 'user',
            is_enabled      INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS waypoint_planets (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            waypoint_id     INTEGER NOT NULL,
            planet_fid      INTEGER NOT NULL,
            role            TEXT NOT NULL,
            distance        REAL,
            UNIQUE(waypoint_id, planet_fid, role),
            FOREIGN KEY (waypoint_id) REFERENCES waypoints(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_waypoint_planets_planet_fid
            ON waypoint_planets(planet_fid);

        CREATE INDEX IF NOT EXISTS idx_waypoint_planets_role
            ON waypoint_planets(role);

        CREATE INDEX IF NOT EXISTS idx_waypoints_enabled
            ON waypoints(is_enabled);
        "#,
    )?;

    Ok(())
}
