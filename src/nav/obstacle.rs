//! Obstacle detection helpers for direct routes.

use crate::nav::geometry::closest_point_on_segment;
use crate::nav::models::{
    CollisionExplain, Obstacle, ObstacleCheck, Planet, Point2, RouteWaypoint,
};

/// Default route clearance added to each obstacle radius.
pub const DEFAULT_ROUTE_CLEARANCE: f64 = 0.2;

/// Checks one obstacle against the direct route segment between two planets.
pub fn check_obstacle_against_route(
    from: &Planet,
    to: &Planet,
    obstacle: &Obstacle,
    route_clearance: f64,
) -> ObstacleCheck {
    let a = Point2 {
        x: from.x,
        y: from.y,
    };
    let b = Point2 { x: to.x, y: to.y };
    let p = Point2 {
        x: obstacle.x,
        y: obstacle.y,
    };

    let (t, closest_point, closest_distance) = closest_point_on_segment(a, b, p);
    let required_clearance = obstacle.radius + route_clearance;
    let is_violating = closest_distance < required_clearance;

    ObstacleCheck {
        obstacle_id: obstacle.id,
        obstacle_name: obstacle.name.clone(),
        obstacle_x: obstacle.x,
        obstacle_y: obstacle.y,
        obstacle_radius: obstacle.radius,
        closest_distance,
        required_clearance,
        t,
        closest_point,
        is_violating,
    }
}

/// Builds a direct collision explanation from an obstacle check.
pub fn build_collision_explain(check: &ObstacleCheck) -> Option<CollisionExplain> {
    if !check.is_violating {
        return None;
    }

    let violated_by = (check.required_clearance - check.closest_distance).max(0.0);
    let proximity_penalty = violated_by * 10_000.0;

    Some(CollisionExplain {
        obstacle_id: check.obstacle_id,
        obstacle_name: check.obstacle_name.clone(),
        obstacle_x: check.obstacle_x,
        obstacle_y: check.obstacle_y,
        obstacle_radius: check.obstacle_radius,
        closest_distance: check.closest_distance,
        required_clearance: check.required_clearance,
        violated_by,
        closest_point: check.closest_point,
        t: check.t,
        proximity_penalty,
    })
}

/// Returns the first collision encountered along the segment.
///
/// Ordering:
/// - smallest `t` first
/// - then smallest closest distance
pub fn first_collision_on_segment(
    from: &Planet,
    to: &Planet,
    obstacles: &[Obstacle],
    route_clearance: f64,
) -> Option<ObstacleCheck> {
    const EPS: f64 = 1e-6;

    obstacles
        .iter()
        .map(|obstacle| check_obstacle_against_route(from, to, obstacle, route_clearance))
        .filter(|check| check.is_violating && check.t > EPS && check.t < 1.0 - EPS)
        .min_by(|a, b| {
            a.t.total_cmp(&b.t)
                .then(a.closest_distance.total_cmp(&b.closest_distance))
        })
}

/// Returns true if the segment from planet A to waypoint W is safe.
pub fn segment_planet_to_waypoint_is_safe(
    from: &Planet,
    waypoint: &RouteWaypoint,
    obstacle: &Obstacle,
    route_clearance: f64,
) -> bool {
    let wp_planet = Planet {
        id: -1,
        name: "detour".to_string(),
        x: waypoint.x,
        y: waypoint.y,
        z: 0.0,
    };

    let check = check_obstacle_against_route(from, &wp_planet, obstacle, route_clearance);
    !check.is_violating
}

/// Returns true if the segment from waypoint W to planet B is safe.
pub fn segment_waypoint_to_planet_is_safe(
    waypoint: &RouteWaypoint,
    to: &Planet,
    obstacle: &Obstacle,
    route_clearance: f64,
) -> bool {
    let wp_planet = Planet {
        id: -1,
        name: "detour".to_string(),
        x: waypoint.x,
        y: waypoint.y,
        z: 0.0,
    };

    let check = check_obstacle_against_route(&wp_planet, to, obstacle, route_clearance);
    !check.is_violating
}

/// Returns true if the segment from one planet to another is free of collisions.
pub fn segment_is_safe(
    from: &Planet,
    to: &Planet,
    obstacles: &[Obstacle],
    route_clearance: f64,
) -> bool {
    first_collision_on_segment(from, to, obstacles, route_clearance).is_none()
}

/// Returns true if a two-leg detour route is fully safe against all obstacles.
///
/// Route:
/// `from -> waypoint -> to`
pub fn detour_route_is_safe(
    from: &Planet,
    waypoint: &RouteWaypoint,
    to: &Planet,
    obstacles: &[Obstacle],
    route_clearance: f64,
) -> bool {
    let wp_planet = Planet {
        id: -1,
        name: "detour".to_string(),
        x: waypoint.x,
        y: waypoint.y,
        z: 0.0,
    };

    segment_is_safe(from, &wp_planet, obstacles, route_clearance)
        && segment_is_safe(&wp_planet, to, obstacles, route_clearance)
}
