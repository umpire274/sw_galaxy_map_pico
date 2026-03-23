//! Basic route calculation orchestration.

use super::distance::{euclidean_distance_raw, raw_distance_to_parsec};
use super::eta::estimate_eta_seconds;
use super::models::{RouteRequest, RouteSummary};

/// Calculates a basic direct route summary.
pub fn calculate_basic_route(request: &RouteRequest) -> RouteSummary {
    let raw_distance = euclidean_distance_raw(&request.from, &request.to);
    let distance_parsec = raw_distance_to_parsec(raw_distance);
    let eta_seconds = estimate_eta_seconds(distance_parsec, request.speed_profile);

    RouteSummary {
        raw_distance,
        distance_parsec,
        eta_seconds,
    }
}
