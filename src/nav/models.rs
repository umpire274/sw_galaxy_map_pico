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
    /// Obstacle radius.
    pub obstacle_radius: f64,
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

/// Explains why the direct route segment is unsafe.
#[derive(Debug, Clone)]
pub struct CollisionExplain {
    /// Obstacle identifier.
    pub obstacle_id: i64,
    /// Obstacle name.
    pub obstacle_name: String,
    /// Obstacle center X coordinate.
    pub obstacle_x: f64,
    /// Obstacle center Y coordinate.
    pub obstacle_y: f64,
    /// Obstacle radius.
    pub obstacle_radius: f64,
    /// Closest distance from route to obstacle.
    pub closest_distance: f64,
    /// Required minimum clearance.
    pub required_clearance: f64,
    /// Amount of clearance violation.
    pub violated_by: f64,
    /// Closest point on the segment.
    pub closest_point: Point2,
    /// Segment interpolation factor.
    pub t: f64,
    /// Penalty assigned to this unsafe segment.
    pub proximity_penalty: f64,
}

/// One route waypoint in 2D space.
#[derive(Debug, Clone)]
pub struct RouteWaypoint {
    /// Waypoint X coordinate.
    pub x: f64,
    /// Waypoint Y coordinate.
    pub y: f64,
}

/// One detour candidate built to avoid an obstacle.
#[derive(Debug, Clone)]
pub struct DetourCandidate {
    /// Candidate waypoint.
    pub waypoint: RouteWaypoint,
    /// Side label used for debugging.
    pub side: String,
    /// Offset distance used to generate the candidate.
    pub offset_used: f64,
    /// Base route distance for `from -> waypoint -> to`.
    pub base_distance: f64,
    /// Turn penalty contribution.
    pub turn_penalty: f64,
    /// Backtracking penalty contribution.
    pub back_penalty: f64,
    /// Proximity penalty contribution.
    pub proximity_penalty: f64,
    /// Total candidate score.
    pub total_score: f64,
    /// Whether the candidate passed validation.
    pub is_valid: bool,
    /// Optional rejection reason.
    pub rejection_reason: Option<String>,
}

/// Route engine tuning parameters.
#[derive(Debug, Clone, Copy)]
pub struct RouteOptions {
    /// Additional clearance added to obstacle radius.
    pub clearance: f64,
    /// Maximum number of offset attempts for detour generation.
    pub max_offset_tries: usize,
    /// Multiplicative growth applied to the detour offset at each retry.
    pub offset_growth: f64,
}

impl Default for RouteOptions {
    fn default() -> Self {
        Self {
            clearance: 0.2,
            max_offset_tries: 6,
            offset_growth: 1.4,
        }
    }
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
    /// Raw direct distance in map coordinate units.
    pub raw_distance: f64,
    /// Direct route distance in parsecs.
    pub distance_parsec: f64,
    /// Estimated travel time in seconds for the direct route.
    pub eta_seconds: u64,
    /// Closest violating obstacle on the direct route, if any.
    pub closest_violation: Option<ObstacleCheck>,
    /// Explain data for the direct collision, if any.
    pub collision_explain: Option<CollisionExplain>,
    /// Optional chosen detour waypoint.
    pub detour_waypoint: Option<RouteWaypoint>,
    /// Optional chosen detour candidate with score breakdown.
    pub detour_candidate: Option<DetourCandidate>,
    /// All evaluated detour candidates.
    pub detour_candidates: Vec<DetourCandidate>,
    /// Whether the final route uses a detour waypoint.
    pub used_detour: bool,
    /// Final effective route distance in parsecs.
    pub final_distance_parsec: f64,
    /// Final effective ETA in seconds.
    pub final_eta_seconds: u64,
    /// Whether both legs of the detoured route are safe.
    pub detour_is_safe: bool,
    /// Whether the original direct route had a collision.
    pub direct_route_has_collision: bool,
    /// Explain history for each routing iteration.
    pub iterations: Vec<RouteIterationExplain>,
    /// Final computed path (including inserted waypoints).
    pub final_path: Vec<Point2>,
    /// Total number of iterations executed by the router.
    pub total_iterations: usize,
    /// Final collision still present after routing, if any.
    pub final_collision: Option<ObstacleCheck>,
}

/// A full route path composed of multiple points.
#[derive(Debug, Clone)]
pub struct RoutePath {
    /// Ordered list of points composing the route.
    pub points: Vec<Point2>,
}

impl From<&RouteWaypoint> for Point2 {
    fn from(value: &RouteWaypoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

/// Explain data for one routing iteration.
#[derive(Debug, Clone)]
pub struct RouteIterationExplain {
    /// Iteration index starting from 1.
    pub iteration: usize,
    /// Index of the segment that collided.
    pub segment_index: usize,
    /// Collision details for the segment.
    pub collision: ObstacleCheck,
    /// All candidates evaluated during this iteration.
    pub candidates: Vec<DetourCandidate>,
    /// Selected candidate, if any.
    pub selected_candidate: Option<DetourCandidate>,
}
