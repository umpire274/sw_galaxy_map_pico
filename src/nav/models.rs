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

/// Navigation speed configuration.
#[derive(Debug, Clone, Copy)]
pub struct SpeedProfile {
    /// Base travel speed expressed in parsecs per hour.
    pub base_speed_parsec_per_hour: f64,
    /// Hyperdrive class multiplier.
    pub hyperdrive_class: f64,
    /// Route-specific multiplier applied after class adjustment.
    pub route_multiplier: f64,
}

/// Route request between two planets.
#[derive(Debug, Clone)]
pub struct RouteRequest {
    /// Origin planet.
    pub from: Planet,
    /// Destination planet.
    pub to: Planet,
    /// Travel speed profile.
    pub speed_profile: SpeedProfile,
}

/// Summary returned by the route engine.
#[derive(Debug, Clone)]
pub struct RouteSummary {
    /// Raw distance in map coordinate units.
    pub raw_distance: f64,
    /// Total route distance in parsecs.
    pub distance_parsec: f64,
    /// Estimated travel time in seconds.
    pub eta_seconds: u64,
}
