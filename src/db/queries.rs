//! Routing-oriented database queries.

use anyhow::Result;
use rusqlite::{Connection, params};

use crate::nav::models::Obstacle;

/// Bounding box query parameters for routing obstacle search.
#[derive(Debug, Clone, Copy)]
pub struct ObstacleQueryBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

/// Returns routing obstacles inside the bounding box of the route segment.
///
/// This mirrors the desktop-core approach:
/// - only planets linked through `waypoint_planets`
/// - only obstacle roles (`avoid`, `danger`, `interdiction`)
/// - radius from `distance` or `default_radius`
pub fn list_routing_obstacles_in_bbox(
    conn: &Connection,
    bounds: ObstacleQueryBounds,
    from_id: i64,
    to_id: i64,
    default_radius: f64,
) -> Result<Vec<Obstacle>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT
            p.remote_id,
            p.name,
            p.x,
            p.y,
            MAX(COALESCE(wpl.distance, ?7)) AS obstacle_radius
        FROM waypoint_planets wpl
        JOIN planets p
            ON p.remote_id = wpl.planet_fid
        JOIN waypoints w
            ON w.id = wpl.waypoint_id
        WHERE p.remote_id NOT IN (?5, ?6)
          AND p.x BETWEEN ?1 AND ?2
          AND p.y BETWEEN ?3 AND ?4
          AND w.is_enabled = 1
          AND wpl.role IN ('avoid', 'danger', 'interdiction')
        GROUP BY p.remote_id, p.name, p.x, p.y
        ORDER BY p.remote_id
        "#,
    )?;

    let rows = stmt.query_map(
        params![
            bounds.min_x,
            bounds.max_x,
            bounds.min_y,
            bounds.max_y,
            from_id,
            to_id,
            default_radius
        ],
        |row| {
            Ok(Obstacle {
                id: row.get(0)?,
                name: row.get(1)?,
                x: row.get(2)?,
                y: row.get(3)?,
                radius: row.get(4)?,
            })
        },
    )?;

    let mut obstacles = Vec::new();
    for row in rows {
        obstacles.push(row?);
    }

    Ok(obstacles)
}

/// Inserts a minimal routing waypoint and links multiple obstacle planets to it.
///
/// This helper is intended only for bootstrap/testing until full parity with
/// the desktop-core waypoint workflow is implemented.
#[allow(dead_code)]
pub fn seed_test_obstacle_links(conn: &Connection, planet_fids: &[i64], radius: f64) -> Result<()> {
    conn.execute(
        r#"
        INSERT OR IGNORE INTO waypoints (code, label, x, y, kind, is_enabled)
        VALUES ('seed-obstacle', 'Seed Obstacle Waypoint', 0.0, 0.0, 'seed', 1)
        "#,
        [],
    )?;

    let waypoint_id: i64 = conn.query_row(
        "SELECT id FROM waypoints WHERE code = 'seed-obstacle' LIMIT 1",
        [],
        |row| row.get(0),
    )?;

    for planet_fid in planet_fids {
        conn.execute(
            r#"
            INSERT OR IGNORE INTO waypoint_planets (waypoint_id, planet_fid, role, distance)
            VALUES (?1, ?2, 'avoid', ?3)
            "#,
            rusqlite::params![waypoint_id, planet_fid, radius],
        )?;
    }

    Ok(())
}
