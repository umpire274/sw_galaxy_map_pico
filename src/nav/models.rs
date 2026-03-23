//! Core navigation models.

/// Unique identifier of a planet.
pub type PlanetId = i64;

/// Basic planet information required by the route engine.
#[derive(Debug, Clone)]
pub struct Planet {
    /// Planet identifier.
    pub id: PlanetId,
    /// Canonical display name.
    pub name: String,
    /// X coordinate in the project reference system.
    pub x: f64,
    /// Y coordinate in the project reference system.
    pub y: f64,
    /// Z coordinate in the project reference system.
    pub z: f64,
}

/// Route request between two planets.
#[derive(Debug, Clone)]
pub struct RouteRequest {
    /// Origin planet.
    pub from: Planet,
    /// Destination planet.
    pub to: Planet,
    /// Travel speed in arbitrary project units.
    pub speed: f64,
}

/// Summary returned by the route engine.
#[derive(Debug, Clone)]
pub struct RouteSummary {
    /// Distance between origin and destination.
    pub distance: f64,
    /// Estimated travel time in minutes.
    pub eta_minutes: u64,
}
