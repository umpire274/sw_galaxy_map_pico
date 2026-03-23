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
    /// Closest violating obstacle, if any.
    pub closest_violation: Option<ObstacleCheck>,
}

/// A point in 2D route space.
#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

/// Route obstacle definition.
#[derive(Debug, Clone)]
pub struct Obstacle {
    /// Planet identifier.
    pub id: i64,
    /// Planet display name.
    pub name: String,
    /// Obstacle center X coordinate.
    pub x: f64,
    /// Obstacle center Y coordinate.
    pub y: f64,
    /// Obstacle radius in parsecs.
    pub radius: f64,
}

/// Closest approach information between a route segment and an obstacle.
#[derive(Debug, Clone)]
pub struct ObstacleCheck {
    /// Obstacle identifier.
    pub obstacle_id: i64,
    /// Obstacle name.
    pub obstacle_name: String,
    /// Obstacle center X coordinate.
    pub obstacle_x: f64,
    /// Obstacle center Y coordinate.
    pub obstacle_y: f64,
    /// Closest distance from the segment to the obstacle center.
    pub closest_distance: f64,
    /// Required minimum clearance.
    pub required_clearance: f64,
    /// Segment interpolation factor clamped to [0, 1].
    pub t: f64,
    /// Closest point on the segment.
    pub closest_point: Point2,
    /// Whether the safety constraint is violated.
    pub is_violating: bool,
}
