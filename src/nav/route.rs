//! Basic route calculation orchestration.

use super::detour::{compute_detour_distance_parsec, evaluate_detour_with_offset_growth};
use super::distance::{euclidean_distance_raw, raw_distance_to_parsec};
use super::eta::estimate_eta_seconds;
use super::models::{Obstacle, RouteOptions, RouteRequest, RouteSummary};
use super::obstacle::{build_collision_explain, detour_route_is_safe, first_collision_on_segment};

/// Calculates a basic route summary with optional single-waypoint detour.
pub fn calculate_basic_route(request: &RouteRequest, obstacles: &[Obstacle]) -> RouteSummary {
    let options = RouteOptions::default();

    let raw_distance = euclidean_distance_raw(&request.from, &request.to);
    let distance_parsec = raw_distance_to_parsec(raw_distance);
    let eta_seconds = estimate_eta_seconds(distance_parsec, request.speed_profile);

    let closest_violation =
        first_collision_on_segment(&request.from, &request.to, obstacles, options.clearance);

    let collision_explain = closest_violation.as_ref().and_then(build_collision_explain);

    let (detour_candidates, detour_candidate) = if let Some(violation) = &closest_violation {
        let obstacle = obstacles.iter().find(|o| o.id == violation.obstacle_id);

        if let Some(obstacle) = obstacle {
            evaluate_detour_with_offset_growth(
                &request.from,
                &request.to,
                obstacle,
                violation,
                options,
            )
        } else {
            (Vec::new(), None)
        }
    } else {
        (Vec::new(), None)
    };

    let detour_waypoint = detour_candidate.as_ref().map(|c| c.waypoint.clone());

    let (used_detour, final_distance_parsec, final_eta_seconds, detour_is_safe) =
        if let Some(candidate) = &detour_candidate {
            let final_distance_parsec =
                compute_detour_distance_parsec(&request.from, &request.to, &candidate.waypoint);

            let final_eta_seconds =
                estimate_eta_seconds(final_distance_parsec, request.speed_profile);

            let detour_is_safe = detour_route_is_safe(
                &request.from,
                &candidate.waypoint,
                &request.to,
                obstacles,
                options.clearance,
            );

            (
                true,
                final_distance_parsec,
                final_eta_seconds,
                detour_is_safe,
            )
        } else {
            (false, distance_parsec, eta_seconds, false)
        };

    RouteSummary {
        raw_distance,
        distance_parsec,
        eta_seconds,
        closest_violation,
        collision_explain,
        detour_waypoint,
        detour_candidate,
        detour_candidates,
        used_detour,
        final_distance_parsec,
        final_eta_seconds,
        detour_is_safe,
    }
}
