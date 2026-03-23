//! Planet catalog queries.

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, Transaction, params};
use std::collections::HashSet;

use crate::provision::arcgis::{RemotePlanetRecord, SkippedPlanetRow};
use crate::utils::normalize::normalize_text;

/// Detailed planet information used by the textual UI.
#[derive(Debug, Clone)]
pub struct PlanetDetails {
    /// Remote ArcGIS identifier.
    pub remote_id: i64,
    /// Canonical planet name.
    pub name: String,
    /// Optional region.
    pub region: Option<String>,
    /// Optional sector.
    pub sector: Option<String>,
    /// Optional system name.
    pub system_name: Option<String>,
    /// Optional grid.
    pub grid: Option<String>,
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Optional canon flag.
    pub canon: Option<i64>,
    /// Optional legends flag.
    pub legends: Option<i64>,
    /// Optional status.
    pub status: Option<String>,
}

/// Returns the total number of planets in the galaxy database.
pub fn count_planets(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM planets", [], |row| row.get(0))?;
    Ok(count)
}

/// Returns the total number of unknown planets in the galaxy database.
pub fn count_unknown_planets(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM planets_unknown", [], |row| row.get(0))?;
    Ok(count)
}

/// Upserts a normalized planet record into the `planets` table.
pub fn upsert_planet(tx: &Transaction<'_>, record: &RemotePlanetRecord) -> Result<()> {
    tx.execute(
        r#"
        INSERT INTO planets (
            remote_id,
            name,
            region,
            sector,
            system_name,
            grid,
            x,
            y,
            canon,
            legends,
            zm,
            name0,
            name1,
            name2,
            lat,
            long,
            ref,
            status,
            c_region,
            c_region_li
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20
        )
        ON CONFLICT(remote_id) DO UPDATE SET
            name = excluded.name,
            region = excluded.region,
            sector = excluded.sector,
            system_name = excluded.system_name,
            grid = excluded.grid,
            x = excluded.x,
            y = excluded.y,
            canon = excluded.canon,
            legends = excluded.legends,
            zm = excluded.zm,
            name0 = excluded.name0,
            name1 = excluded.name1,
            name2 = excluded.name2,
            lat = excluded.lat,
            long = excluded.long,
            ref = excluded.ref,
            status = excluded.status,
            c_region = excluded.c_region,
            c_region_li = excluded.c_region_li
        "#,
        params![
            record.remote_id,
            record.name,
            record.region,
            record.sector,
            record.system_name,
            record.grid,
            record.x,
            record.y,
            record.canon,
            record.legends,
            record.zm,
            record.name0,
            record.name1,
            record.name2,
            record.lat,
            record.long,
            record.ref_code,
            record.status,
            record.c_region,
            record.c_region_li,
        ],
    )?;

    Ok(())
}

/// Replaces the entire `planets_unknown` table with the current skipped rows.
///
/// This simple synchronization strategy is suitable for the first persistence
/// milestone and can be optimized later.
pub fn replace_unknown_planets(
    tx: &Transaction<'_>,
    skipped_rows: &[SkippedPlanetRow],
) -> Result<()> {
    tx.execute("DELETE FROM planets_unknown", [])?;

    let mut stmt = tx.prepare(
        r#"
        INSERT INTO planets_unknown (
            fid,
            planet,
            x,
            y,
            reason
        ) VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )?;

    for row in skipped_rows {
        stmt.execute(params![row.fid, row.planet, row.x, row.y, row.reason])?;
    }

    Ok(())
}

/// Inserts aliases for a given planet.
pub fn insert_aliases(tx: &Transaction<'_>, record: &RemotePlanetRecord) -> Result<()> {
    let mut stmt = tx.prepare(
        r#"
        INSERT OR IGNORE INTO planet_aliases (
            planet_fid,
            alias,
            alias_norm,
            source
        ) VALUES (?1, ?2, ?3, ?4)
        "#,
    )?;

    for (source, value) in [
        ("name0", &record.name0),
        ("name1", &record.name1),
        ("name2", &record.name2),
    ] {
        if let Some(alias) = value {
            let alias = alias.trim();

            if !alias.is_empty() {
                stmt.execute(rusqlite::params![
                    record.remote_id,
                    alias,
                    normalize_text(alias),
                    source,
                ])?;
            }
        }
    }

    Ok(())
}

/// Searches planets by canonical name and alias, returning unique results.
pub fn search_planets(conn: &Connection, query: &str) -> Result<Vec<(i64, String)>> {
    let mut seen = HashSet::new();
    let mut combined = Vec::new();

    for (id, name) in search_planets_by_name(conn, query)? {
        if seen.insert(id) {
            combined.push((id, name));
        }
    }

    for (id, name) in search_planets_by_alias(conn, query)? {
        if seen.insert(id) {
            combined.push((id, name));
        }
    }

    combined.sort_by(|a, b| a.1.cmp(&b.1));
    combined.truncate(20);

    Ok(combined)
}

pub fn search_planets_by_name(conn: &Connection, query: &str) -> Result<Vec<(i64, String)>> {
    let q = normalize_text(query);

    let mut stmt = conn.prepare(
        r#"
        SELECT remote_id, name
        FROM planets
        WHERE LOWER(name) LIKE ?1
        ORDER BY name
        LIMIT 20
        "#,
    )?;

    let rows = stmt.query_map([format!("%{}%", q)], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}

pub fn search_planets_by_alias(conn: &Connection, query: &str) -> Result<Vec<(i64, String)>> {
    let q = normalize_text(query);

    let mut stmt = conn.prepare(
        r#"
        SELECT p.remote_id, p.name
        FROM planet_aliases a
        JOIN planets p ON p.remote_id = a.planet_fid
        WHERE a.alias_norm LIKE ?1
        ORDER BY p.name
        LIMIT 20
        "#,
    )?;

    let rows = stmt.query_map([format!("%{}%", q)], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}

/// Returns detailed information for one planet by remote identifier.
pub fn get_planet_details(conn: &Connection, remote_id: i64) -> Result<Option<PlanetDetails>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT
            remote_id,
            name,
            region,
            sector,
            system_name,
            grid,
            x,
            y,
            canon,
            legends,
            status
        FROM planets
        WHERE remote_id = ?1
        LIMIT 1
        "#,
    )?;

    let result = stmt
        .query_row([remote_id], |row| {
            Ok(PlanetDetails {
                remote_id: row.get(0)?,
                name: row.get(1)?,
                region: row.get(2)?,
                sector: row.get(3)?,
                system_name: row.get(4)?,
                grid: row.get(5)?,
                x: row.get(6)?,
                y: row.get(7)?,
                canon: row.get(8)?,
                legends: row.get(9)?,
                status: row.get(10)?,
            })
        })
        .optional()?;

    Ok(result)
}
