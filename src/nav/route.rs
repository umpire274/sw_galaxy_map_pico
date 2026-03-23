//! Basic route calculation orchestration.

use super::distance::{euclidean_distance_raw, raw_distance_to_parsec};
use super::eta::estimate_eta_seconds;
use super::models::{Obstacle, RouteRequest, RouteSummary};
use super::obstacle::{DEFAULT_ROUTE_CLEARANCE, rank_obstacles_by_distance};

/// Calculates a basic direct route summary.
pub fn calculate_basic_route(request: &RouteRequest, obstacles: &[Obstacle]) -> RouteSummary {
    let raw_distance = euclidean_distance_raw(&request.from, &request.to);
    let distance_parsec = raw_distance_to_parsec(raw_distance);
    let eta_seconds = estimate_eta_seconds(distance_parsec, request.speed_profile);

    let obstacle_debug = rank_obstacles_by_distance(
        &request.from,
        &request.to,
        obstacles,
        DEFAULT_ROUTE_CLEARANCE,
    );

    let closest_violation = obstacle_debug
        .iter()
        .find(|check| check.is_violating)
        .cloned();

    RouteSummary {
        raw_distance,
        distance_parsec,
        eta_seconds,
        closest_violation,
    }
}
