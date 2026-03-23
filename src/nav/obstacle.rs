//! Obstacle detection helpers for direct routes.

use crate::nav::geometry::closest_point_on_segment;
use crate::nav::models::{Obstacle, ObstacleCheck, Planet, Point2};

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
        closest_distance,
        required_clearance,
        t,
        closest_point,
        is_violating,
    }
}

/// Finds the first violating obstacle with the smallest closest distance.
pub fn find_closest_violation(
    from: &Planet,
    to: &Planet,
    obstacles: &[Obstacle],
    route_clearance: f64,
) -> Option<ObstacleCheck> {
    obstacles
        .iter()
        .map(|obstacle| check_obstacle_against_route(from, to, obstacle, route_clearance))
        .filter(|check| check.is_violating)
        .min_by(|a, b| a.closest_distance.total_cmp(&b.closest_distance))
}

/// Returns all obstacle checks sorted by closest distance to the route segment.
pub fn rank_obstacles_by_distance(
    from: &Planet,
    to: &Planet,
    obstacles: &[Obstacle],
    route_clearance: f64,
) -> Vec<ObstacleCheck> {
    let mut checks: Vec<ObstacleCheck> = obstacles
        .iter()
        .map(|obstacle| check_obstacle_against_route(from, to, obstacle, route_clearance))
        .collect();

    checks.sort_by(|a, b| a.closest_distance.total_cmp(&b.closest_distance));
    checks
}
