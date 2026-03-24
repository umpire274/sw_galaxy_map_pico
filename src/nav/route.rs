//! Basic route calculation orchestration.

use super::detour::{compute_detour_distance_parsec, evaluate_detour_with_offset_growth};
use super::distance::{euclidean_distance_raw, raw_distance_to_parsec};
use super::eta::estimate_eta_seconds;
use super::models::{
    Obstacle, Planet, Point2, RouteIterationExplain, RouteOptions, RouteRequest, RouteSummary,
};
use super::obstacle::{
    build_collision_explain, detour_route_is_safe, first_collision_on_path,
    first_collision_on_segment,
};
use crate::nav::segment::{build_segments, insert_waypoint};

/// Calculates a basic route summary with optional single-waypoint detour.
pub fn calculate_basic_route(request: &RouteRequest, obstacles: &[Obstacle]) -> RouteSummary {
    let options = RouteOptions::default();

    let raw_distance = euclidean_distance_raw(&request.from, &request.to);
    let distance_parsec = raw_distance_to_parsec(raw_distance);
    let eta_seconds = estimate_eta_seconds(distance_parsec, request.speed_profile);

    let closest_violation =
        first_collision_on_segment(&request.from, &request.to, obstacles, options.clearance);

    let direct_route_has_collision = closest_violation.is_some();
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
            (
                false,
                distance_parsec,
                eta_seconds,
                !direct_route_has_collision,
            )
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
        direct_route_has_collision,
        used_detour,
        final_distance_parsec,
        final_eta_seconds,
        detour_is_safe,
        iterations: Vec::new(),
    }
}

/// Calculates an iterative route summary with repeated waypoint insertion.
///
/// This is the first multi-step router version:
/// - starts from the direct route
/// - detects the first collision on the current path
/// - inserts one detour waypoint on the colliding segment
/// - repeats up to a fixed iteration limit
pub fn calculate_iterative_route<F>(request: &RouteRequest, mut load_obstacles: F) -> RouteSummary
where
    F: FnMut(f64, f64, f64, f64) -> Vec<Obstacle>,
{
    let options = RouteOptions::default();

    // Direct-route metrics (always preserved in the summary).
    let raw_distance = euclidean_distance_raw(&request.from, &request.to);
    let distance_parsec = raw_distance_to_parsec(raw_distance);
    let eta_seconds = estimate_eta_seconds(distance_parsec, request.speed_profile);

    // Initial direct-route obstacle set.
    let mut obstacles = load_obstacles(
        request.from.x.min(request.to.x),
        request.from.x.max(request.to.x),
        request.from.y.min(request.to.y),
        request.from.y.max(request.to.y),
    );

    let closest_violation =
        first_collision_on_segment(&request.from, &request.to, &obstacles, options.clearance);

    let direct_route_has_collision = closest_violation.is_some();
    let collision_explain = closest_violation.as_ref().and_then(build_collision_explain);

    // Initial path: direct route only.
    let mut path = vec![
        Point2 {
            x: request.from.x,
            y: request.from.y,
        },
        Point2 {
            x: request.to.x,
            y: request.to.y,
        },
    ];

    let mut iterations = 0usize;
    let max_iters = 8usize;

    let mut all_candidates = Vec::new();
    let mut last_candidate = None;
    let mut iterations_explain = Vec::new();

    loop {
        if iterations >= max_iters {
            break;
        }

        // Reload obstacles dynamically using the current path bounding box.
        let (min_x, max_x, min_y, max_y) = crate::nav::segment::path_bbox(&path);
        let new_obstacles = load_obstacles(min_x, max_x, min_y, max_y);
        crate::nav::obstacle::merge_obstacles(&mut obstacles, new_obstacles);

        let collision = first_collision_on_path(&path, &obstacles, options.clearance);

        let Some((segment_index, violation)) = collision else {
            break;
        };

        let obstacle = obstacles.iter().find(|o| o.id == violation.obstacle_id);

        let Some(obstacle) = obstacle else {
            break;
        };

        let from = Planet {
            id: -1,
            name: "seg_start".into(),
            x: path[segment_index].x,
            y: path[segment_index].y,
            z: 0.0,
        };

        let to = Planet {
            id: -1,
            name: "seg_end".into(),
            x: path[segment_index + 1].x,
            y: path[segment_index + 1].y,
            z: 0.0,
        };

        let (candidates, best) =
            evaluate_detour_with_offset_growth(&from, &to, obstacle, &violation, options);

        iterations_explain.push(RouteIterationExplain {
            iteration: iterations + 1,
            segment_index,
            collision: violation.clone(),
            candidates: candidates.clone(),
            selected_candidate: best.clone(),
        });

        all_candidates.extend(candidates);

        let Some(best) = best else {
            break;
        };

        insert_waypoint(
            &mut path,
            segment_index,
            Point2 {
                x: best.waypoint.x,
                y: best.waypoint.y,
            },
        );

        last_candidate = Some(best);
        iterations += 1;
    }

    // Compute final path distance.
    let mut total_distance_parsec = 0.0;
    let segments = build_segments(&path);

    for seg in &segments {
        let from = Planet {
            id: -1,
            name: "path_from".into(),
            x: seg.start.x,
            y: seg.start.y,
            z: 0.0,
        };

        let to = Planet {
            id: -1,
            name: "path_to".into(),
            x: seg.end.x,
            y: seg.end.y,
            z: 0.0,
        };

        total_distance_parsec += raw_distance_to_parsec(euclidean_distance_raw(&from, &to));
    }

    let final_eta_seconds = estimate_eta_seconds(total_distance_parsec, request.speed_profile);

    // Final safety check on the fully expanded path.
    let (final_min_x, final_max_x, final_min_y, final_max_y) =
        crate::nav::segment::path_bbox(&path);
    let final_new_obstacles = load_obstacles(final_min_x, final_max_x, final_min_y, final_max_y);
    crate::nav::obstacle::merge_obstacles(&mut obstacles, final_new_obstacles);

    let final_collision = first_collision_on_path(&path, &obstacles, options.clearance);
    let detour_is_safe = final_collision.is_none();

    let detour_waypoint = last_candidate.as_ref().map(|c| c.waypoint.clone());

    RouteSummary {
        raw_distance,
        distance_parsec,
        eta_seconds,
        closest_violation,
        collision_explain,
        detour_waypoint,
        detour_candidate: last_candidate,
        detour_candidates: all_candidates,
        direct_route_has_collision,
        used_detour: iterations > 0,
        final_distance_parsec: total_distance_parsec,
        final_eta_seconds,
        detour_is_safe,
        iterations: iterations_explain,
    }
}
