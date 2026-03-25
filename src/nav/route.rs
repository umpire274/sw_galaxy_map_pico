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
use crate::db::route_explain::{
    SavedCandidateExplain, SavedCollisionExplain, SavedCollisionPenaltyExplain, SavedDetourExplain,
    SavedIterationExplain, SavedPointExplain, SavedQualityExplain, SavedRouteExplain,
};
use crate::nav::segment::{build_segments, insert_waypoint};
use sha2::{Digest, Sha256};

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

    let final_path = vec![
        Point2 {
            x: request.from.x,
            y: request.from.y,
        },
        Point2 {
            x: request.to.x,
            y: request.to.y,
        },
    ];

    let quality_metrics = crate::nav::models::RouteQualityMetrics {
        waypoint_count: if used_detour { 1 } else { 0 },
        detour_overhead_pc: if final_distance_parsec > distance_parsec {
            final_distance_parsec - distance_parsec
        } else {
            0.0
        },
        max_turn_penalty: detour_candidate
            .as_ref()
            .map(|candidate| candidate.turn_penalty)
            .unwrap_or(0.0),
        total_turn_penalty: detour_candidate
            .as_ref()
            .map(|candidate| candidate.turn_penalty)
            .unwrap_or(0.0),
        total_proximity_penalty: detour_candidate
            .as_ref()
            .map(|candidate| candidate.proximity_penalty)
            .unwrap_or(0.0),
        max_offset_penalty: detour_candidate
            .as_ref()
            .map(|candidate| candidate.offset_penalty)
            .unwrap_or(0.0),
        total_offset_penalty: detour_candidate
            .as_ref()
            .map(|candidate| candidate.offset_penalty)
            .unwrap_or(0.0),
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
        final_path,
        total_iterations: 0,
        final_collision: None,
        quality_metrics,
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
    let mut selected_candidates = Vec::new();

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

        selected_candidates.push(best.clone());

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

    let final_collision = first_collision_on_path(&path, &obstacles, options.clearance)
        .map(|(_, collision)| collision);

    let detour_is_safe = final_collision.is_none();

    let detour_waypoint = last_candidate.as_ref().map(|c| c.waypoint.clone());

    let waypoint_count = path.len().saturating_sub(2);

    let detour_overhead_pc = if total_distance_parsec > distance_parsec {
        total_distance_parsec - distance_parsec
    } else {
        0.0
    };

    let max_turn_penalty = selected_candidates
        .iter()
        .map(|candidate| candidate.turn_penalty)
        .fold(0.0_f64, f64::max);

    let total_turn_penalty: f64 = selected_candidates
        .iter()
        .map(|candidate| candidate.turn_penalty)
        .sum();

    let total_proximity_penalty: f64 = selected_candidates
        .iter()
        .map(|candidate| candidate.proximity_penalty)
        .sum();

    let max_offset_penalty = selected_candidates
        .iter()
        .map(|candidate| candidate.offset_penalty)
        .fold(0.0_f64, f64::max);

    let total_offset_penalty: f64 = selected_candidates
        .iter()
        .map(|candidate| candidate.offset_penalty)
        .sum();

    let quality_metrics = crate::nav::models::RouteQualityMetrics {
        waypoint_count,
        detour_overhead_pc,
        max_turn_penalty,
        total_turn_penalty,
        total_proximity_penalty,
        max_offset_penalty,
        total_offset_penalty,
    };

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
        final_path: path.clone(),
        total_iterations: iterations,
        final_collision,
        quality_metrics,
    }
}

/// Builds a stable fingerprint for one computed route.
///
/// The fingerprint is based on:
/// - endpoints
/// - direct/final metrics
/// - safety flags
/// - iteration count
/// - final path points
pub fn build_route_fingerprint(
    from_planet_id: i64,
    to_planet_id: i64,
    summary: &RouteSummary,
) -> String {
    let mut s = String::new();

    s.push_str(&format!("from={}\n", from_planet_id));
    s.push_str(&format!("to={}\n", to_planet_id));
    s.push_str(&format!("direct_distance={:.6}\n", summary.distance_parsec));
    s.push_str(&format!(
        "final_distance={:.6}\n",
        summary.final_distance_parsec
    ));
    s.push_str(&format!("direct_eta={}\n", summary.eta_seconds));
    s.push_str(&format!("final_eta={}\n", summary.final_eta_seconds));
    s.push_str(&format!(
        "direct_safe={}\n",
        if summary.direct_route_has_collision {
            0
        } else {
            1
        }
    ));
    s.push_str(&format!(
        "final_safe={}\n",
        if summary.detour_is_safe { 1 } else { 0 }
    ));
    s.push_str(&format!("iterations={}\n", summary.total_iterations));

    for (index, point) in summary.final_path.iter().enumerate() {
        s.push_str(&format!("p{}={:.6},{:.6}\n", index, point.x, point.y));
    }

    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn build_saved_route_explain(summary: &RouteSummary) -> SavedRouteExplain {
    SavedRouteExplain {
        direct_route_status: if summary.direct_route_has_collision {
            "unsafe".to_string()
        } else {
            "safe".to_string()
        },
        final_route_status: if summary.detour_is_safe {
            "safe".to_string()
        } else {
            "unsafe".to_string()
        },
        total_iterations: summary.total_iterations,
        final_collision: summary
            .final_collision
            .as_ref()
            .map(|c| SavedCollisionExplain {
                obstacle_id: c.obstacle_id,
                obstacle_name: c.obstacle_name.clone(),
                obstacle_x: c.obstacle_x,
                obstacle_y: c.obstacle_y,
                closest_distance: c.closest_distance,
                required_clearance: c.required_clearance,
                t: c.t,
                closest_point_x: c.closest_point.x,
                closest_point_y: c.closest_point.y,
            }),
        direct_collision: summary
            .closest_violation
            .as_ref()
            .map(|c| SavedCollisionExplain {
                obstacle_id: c.obstacle_id,
                obstacle_name: c.obstacle_name.clone(),
                obstacle_x: c.obstacle_x,
                obstacle_y: c.obstacle_y,
                closest_distance: c.closest_distance,
                required_clearance: c.required_clearance,
                t: c.t,
                closest_point_x: c.closest_point.x,
                closest_point_y: c.closest_point.y,
            }),
        collision_explain: summary.collision_explain.as_ref().map(|c| {
            SavedCollisionPenaltyExplain {
                obstacle_id: c.obstacle_id,
                obstacle_name: c.obstacle_name.clone(),
                obstacle_x: c.obstacle_x,
                obstacle_y: c.obstacle_y,
                obstacle_radius: c.obstacle_radius,
                closest_distance: c.closest_distance,
                required_clearance: c.required_clearance,
                violated_by: c.violated_by,
                t: c.t,
                closest_point_x: c.closest_point.x,
                closest_point_y: c.closest_point.y,
                proximity_penalty: c.proximity_penalty,
            }
        }),
        last_selected_detour: summary
            .detour_candidate
            .as_ref()
            .map(|d| SavedDetourExplain {
                waypoint_x: d.waypoint.x,
                waypoint_y: d.waypoint.y,
                side: d.side.clone(),
                offset_used: d.offset_used,
                score: d.total_score,
                base_distance: d.base_distance,
                turn_penalty: d.turn_penalty,
                back_penalty: d.back_penalty,
                proximity_penalty: d.proximity_penalty,
                offset_penalty: d.offset_penalty,
            }),
        iterations: summary
            .iterations
            .iter()
            .map(|it| SavedIterationExplain {
                iteration: it.iteration,
                segment_index: it.segment_index,
                collision: SavedCollisionExplain {
                    obstacle_id: it.collision.obstacle_id,
                    obstacle_name: it.collision.obstacle_name.clone(),
                    obstacle_x: it.collision.obstacle_x,
                    obstacle_y: it.collision.obstacle_y,
                    closest_distance: it.collision.closest_distance,
                    required_clearance: it.collision.required_clearance,
                    t: it.collision.t,
                    closest_point_x: it.collision.closest_point.x,
                    closest_point_y: it.collision.closest_point.y,
                },
                selected_candidate: it.selected_candidate.as_ref().map(|d| SavedDetourExplain {
                    waypoint_x: d.waypoint.x,
                    waypoint_y: d.waypoint.y,
                    side: d.side.clone(),
                    offset_used: d.offset_used,
                    score: d.total_score,
                    base_distance: d.base_distance,
                    turn_penalty: d.turn_penalty,
                    back_penalty: d.back_penalty,
                    proximity_penalty: d.proximity_penalty,
                    offset_penalty: d.offset_penalty,
                }),
                candidates: it
                    .candidates
                    .iter()
                    .map(|c| SavedCandidateExplain {
                        side: c.side.clone(),
                        offset_used: c.offset_used,
                        is_valid: c.is_valid,
                        score: c.total_score,
                        base_distance: c.base_distance,
                        turn_penalty: c.turn_penalty,
                        back_penalty: c.back_penalty,
                        proximity_penalty: c.proximity_penalty,
                        offset_penalty: c.offset_penalty,
                        rejection_reason: c.rejection_reason.clone(),
                    })
                    .collect(),
            })
            .collect(),
        final_path: summary
            .final_path
            .iter()
            .map(|p| SavedPointExplain { x: p.x, y: p.y })
            .collect(),
        quality: SavedQualityExplain {
            waypoint_count: summary.quality_metrics.waypoint_count,
            detour_overhead_pc: summary.quality_metrics.detour_overhead_pc,
            max_turn_penalty: summary.quality_metrics.max_turn_penalty,
            total_turn_penalty: summary.quality_metrics.total_turn_penalty,
            total_proximity_penalty: summary.quality_metrics.total_proximity_penalty,
            max_offset_penalty: summary.quality_metrics.max_offset_penalty,
            total_offset_penalty: summary.quality_metrics.total_offset_penalty,
        },
    }
}
