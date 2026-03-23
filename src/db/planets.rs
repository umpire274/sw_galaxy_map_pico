//! Planet catalog queries.

use anyhow::Result;
use rusqlite::{Connection, Transaction, params};

use crate::provision::arcgis::{RemotePlanetRecord, SkippedPlanetRow};

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
