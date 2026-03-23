//! Basic route calculation orchestration.

use super::distance::euclidean_distance;
use super::eta::estimate_eta_minutes;
use super::models::{RouteRequest, RouteSummary};

/// Calculates a basic direct route summary.
pub fn calculate_basic_route(request: &RouteRequest) -> RouteSummary {
    let distance = euclidean_distance(&request.from, &request.to);
    let eta_minutes = estimate_eta_minutes(distance, request.speed);

    RouteSummary {
        distance,
        eta_minutes,
    }
}
