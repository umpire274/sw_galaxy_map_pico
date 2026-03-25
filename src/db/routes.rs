//! Route persistence helpers.

use anyhow::{Context, Result};
use rusqlite::{Connection, OptionalExtension, params};

use crate::nav::models::RouteSummary;
use crate::nav::route::build_route_fingerprint;

/// One route row for recent-routes listing.
#[derive(Debug, Clone)]
pub struct RecentRouteRow {
    /// Route identifier.
    pub id: i64,
    /// Origin planet name.
    pub from_planet_name: String,
    /// Destination planet name.
    pub to_planet_name: String,
    /// Final route distance in parsecs.
    pub final_distance_pc: f64,
    /// Final ETA in seconds.
    pub final_eta_seconds: i64,
    /// Whether the final route is safe.
    pub final_is_safe: bool,
    /// Total iterations performed by the router.
    pub total_iterations: i64,
    /// UTC creation timestamp.
    pub created_at_utc: String,
}

/// One point belonging to a saved route path.
#[derive(Debug, Clone)]
pub struct SavedRoutePoint {
    /// Sequential point index in the final path.
    pub seq_index: i64,
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

/// Full saved route details.
#[derive(Debug, Clone)]
pub struct SavedRouteDetails {
    /// Route identifier.
    pub id: i64,
    /// Origin planet ID.
    pub from_planet_id: i64,
    /// Origin planet name.
    pub from_planet_name: String,
    /// Destination planet ID.
    pub to_planet_id: i64,
    /// Destination planet name.
    pub to_planet_name: String,
    /// Direct route distance in parsecs.
    pub direct_distance_pc: f64,
    /// Final route distance in parsecs.
    pub final_distance_pc: f64,
    /// Direct ETA in seconds.
    pub direct_eta_seconds: i64,
    /// Final ETA in seconds.
    pub final_eta_seconds: i64,
    /// Whether the direct route was safe.
    pub direct_is_safe: bool,
    /// Whether the final route is safe.
    pub final_is_safe: bool,
    /// Total routing iterations.
    pub total_iterations: i64,
    /// UTC creation timestamp.
    pub created_at_utc: String,
    pub route_explain_json: Option<String>,
    /// Final saved path points.
    pub points: Vec<SavedRoutePoint>,
}

/// Result of a route save operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveRouteOutcome {
    /// A new route row was inserted.
    Inserted(i64),
    /// An identical route was already present.
    AlreadyExists(i64),
}

/// Endpoint metadata used when saving one computed route.
#[derive(Debug, Clone)]
pub struct SaveRouteEndpoints<'a> {
    /// Origin planet ID.
    pub from_planet_id: i64,
    /// Origin planet display name.
    pub from_planet_name: &'a str,
    /// Destination planet ID.
    pub to_planet_id: i64,
    /// Destination planet display name.
    pub to_planet_name: &'a str,
}

/// Saves one computed route and its final path into the history database.
///
/// If an identical route already exists, returns its existing ID instead of
/// inserting a duplicate row.
pub fn save_route(
    conn: &mut Connection,
    endpoints: &SaveRouteEndpoints<'_>,
    summary: &RouteSummary,
    route_explain_json: Option<&str>,
    created_at_utc: &str,
) -> Result<SaveRouteOutcome> {
    let fingerprint =
        build_route_fingerprint(endpoints.from_planet_id, endpoints.to_planet_id, summary);

    if let Some(existing_id) = conn
        .query_row(
            "SELECT id FROM routes WHERE route_fingerprint = ?1 LIMIT 1",
            [&fingerprint],
            |row| row.get::<_, i64>(0),
        )
        .optional()?
    {
        if let Some(json) = route_explain_json {
            let existing_explain: Option<String> = conn
                .query_row(
                    "SELECT route_explain_json FROM routes WHERE id = ?1",
                    [existing_id],
                    |row| row.get(0),
                )
                .optional()?;

            // 👇 QUI va questa riga
            let should_update_explain = should_fill_missing_explain(existing_explain.as_deref());

            if should_update_explain {
                conn.execute(
                    "UPDATE routes SET route_explain_json = ?1 WHERE id = ?2",
                    rusqlite::params![json, existing_id],
                )?;
            }
        }

        return Ok(SaveRouteOutcome::AlreadyExists(existing_id));
    }

    let tx = conn
        .transaction()
        .context("Failed to start route save transaction")?;

    tx.execute(
        r#"
        INSERT INTO routes (
            from_planet_id,
            from_planet_name,
            to_planet_id,
            to_planet_name,
            direct_distance_pc,
            final_distance_pc,
            direct_eta_seconds,
            final_eta_seconds,
            direct_is_safe,
            final_is_safe,
            total_iterations,
            created_at_utc,
            route_fingerprint,
            route_explain_json
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        "#,
        params![
            endpoints.from_planet_id,
            endpoints.from_planet_name,
            endpoints.to_planet_id,
            endpoints.to_planet_name,
            summary.distance_parsec,
            summary.final_distance_parsec,
            summary.eta_seconds as i64,
            summary.final_eta_seconds as i64,
            if summary.direct_route_has_collision {
                0
            } else {
                1
            },
            if summary.detour_is_safe { 1 } else { 0 },
            summary.total_iterations as i64,
            created_at_utc,
            fingerprint,
            route_explain_json,
        ],
    )?;

    let route_id = tx.last_insert_rowid();

    {
        let mut stmt = tx.prepare(
            r#"
            INSERT INTO route_points (route_id, seq_index, x, y)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )?;

        for (index, point) in summary.final_path.iter().enumerate() {
            stmt.execute(params![route_id, index as i64, point.x, point.y])?;
        }
    }

    tx.commit()
        .context("Failed to commit route save transaction")?;

    Ok(SaveRouteOutcome::Inserted(route_id))
}

/// Returns the most recent saved routes.
pub fn list_recent_routes(conn: &Connection, limit: usize) -> Result<Vec<RecentRouteRow>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT
            id,
            from_planet_name,
            to_planet_name,
            final_distance_pc,
            final_eta_seconds,
            final_is_safe,
            total_iterations,
            created_at_utc
        FROM routes
        ORDER BY id DESC
        LIMIT ?1
        "#,
    )?;

    let rows = stmt.query_map([limit as i64], |row| {
        Ok(RecentRouteRow {
            id: row.get(0)?,
            from_planet_name: row.get(1)?,
            to_planet_name: row.get(2)?,
            final_distance_pc: row.get(3)?,
            final_eta_seconds: row.get(4)?,
            final_is_safe: row.get::<_, i64>(5)? != 0,
            total_iterations: row.get(6)?,
            created_at_utc: row.get(7)?,
        })
    })?;

    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }

    Ok(out)
}

/// Reads one saved route with its final path points.
pub fn get_route_details(conn: &Connection, route_id: i64) -> Result<Option<SavedRouteDetails>> {
    let route_row = conn
        .query_row(
            r#"
            SELECT
                id,
                from_planet_id,
                from_planet_name,
                to_planet_id,
                to_planet_name,
                direct_distance_pc,
                final_distance_pc,
                direct_eta_seconds,
                final_eta_seconds,
                direct_is_safe,
                final_is_safe,
                total_iterations,
                created_at_utc,
                route_explain_json
            FROM routes
            WHERE id = ?1
            "#,
            [route_id],
            |row| {
                Ok(SavedRouteDetails {
                    id: row.get(0)?,
                    from_planet_id: row.get(1)?,
                    from_planet_name: row.get(2)?,
                    to_planet_id: row.get(3)?,
                    to_planet_name: row.get(4)?,
                    direct_distance_pc: row.get(5)?,
                    final_distance_pc: row.get(6)?,
                    direct_eta_seconds: row.get(7)?,
                    final_eta_seconds: row.get(8)?,
                    direct_is_safe: row.get::<_, i64>(9)? != 0,
                    final_is_safe: row.get::<_, i64>(10)? != 0,
                    total_iterations: row.get(11)?,
                    created_at_utc: row.get(12)?,
                    route_explain_json: row.get(13)?,
                    points: Vec::new(),
                })
            },
        )
        .optional()?;

    let Some(mut details) = route_row else {
        return Ok(None);
    };

    let mut stmt = conn.prepare(
        r#"
        SELECT seq_index, x, y
        FROM route_points
        WHERE route_id = ?1
        ORDER BY seq_index ASC
        "#,
    )?;

    let rows = stmt.query_map([route_id], |row| {
        Ok(SavedRoutePoint {
            seq_index: row.get(0)?,
            x: row.get(1)?,
            y: row.get(2)?,
        })
    })?;

    for row in rows {
        details.points.push(row?);
    }

    Ok(Some(details))
}

fn should_fill_missing_explain(existing: Option<&str>) -> bool {
    existing.map(str::trim).filter(|s| !s.is_empty()).is_none()
}
