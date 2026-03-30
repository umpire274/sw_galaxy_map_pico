//! Textual screen helpers.

use crate::db::planets::PlanetDetails;
use crate::db::route_explain::SavedRouteExplain;
use crate::db::routes::{RecentRouteRow, SavedRouteDetails};
use crate::db::status::DatabaseStatus;
use crate::nav::eta::{effective_speed_parsec_per_hour, format_eta_dd_hh_mm_ss};
use crate::nav::models::{RouteSummary, SpeedProfile};

/// Outp
/// ut detail level for route explain rendering.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplainMode {
    /// Full diagnostic output.
    Full,
    /// Compact output optimized for small displays.
    Compact,
}

/// Renders a route result using the selected explain mode.
pub fn show_route_result_with_mode(
    from: &str,
    to: &str,
    route: &RouteSummary,
    speed_profile: SpeedProfile,
    mode: ExplainMode,
) {
    match mode {
        ExplainMode::Full => show_route_result(from, to, route, speed_profile),
        ExplainMode::Compact => show_route_result_compact(from, to, route),
    }
}

/// Renders the application banner.
pub fn show_banner() {
    println!("====================================");
    println!("      SW Galaxy Map Pico");
    println!("      Offline Navicomputer");
    println!("====================================");
}

/// Renders a simple section title.
pub fn show_section_title(title: &str) {
    println!();
    println!("== {title} ==");
}

/// Renders a list of planet search results.
pub fn show_search_results(results: &[(i64, String)]) {
    if results.is_empty() {
        println!("No planets found.");
        return;
    }

    println!("\nResults:");
    for (index, (_, name)) in results.iter().enumerate() {
        println!("{}. {}", index + 1, name);
    }
}

/// Renders detailed information for one planet.
pub fn show_planet_details(details: &PlanetDetails) {
    println!();
    println!("== Planet details ==");
    println!("ID      : {}", details.remote_id);
    println!("Name    : {}", details.name);
    println!("Region  : {}", details.region.as_deref().unwrap_or("-"));
    println!("Sector  : {}", details.sector.as_deref().unwrap_or("-"));
    println!(
        "System  : {}",
        details.system_name.as_deref().unwrap_or("-")
    );
    println!("Grid    : {}", details.grid.as_deref().unwrap_or("-"));
    println!("X       : {}", details.x);
    println!("Y       : {}", details.y);
    println!(
        "Canon   : {}",
        details
            .canon
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".to_string())
    );
    println!(
        "Legends : {}",
        details
            .legends
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".to_string())
    );
    println!("Status  : {}", details.status.as_deref().unwrap_or("-"));
}

/// Renders the search results screen.
pub fn show_search_results_screen(results: &[(i64, String)]) {
    show_section_title("Search results");
    show_search_results(results);
}

/// Renders the route calculation result.
pub fn show_route_result(from: &str, to: &str, route: &RouteSummary, speed: SpeedProfile) {
    let effective_speed = effective_speed_parsec_per_hour(speed);

    println!();
    println!("== Route result ==");
    println!("From             : {from}");
    println!("To               : {to}");
    println!("Direct distance  : {:.2} pc", route.distance_parsec);
    println!(
        "Direct ETA       : {}",
        format_eta_dd_hh_mm_ss(route.eta_seconds)
    );
    println!(
        "Base speed       : {:.2} pc/h",
        speed.base_speed_parsec_per_hour
    );
    println!("Hyperdrive class : {:.2}", speed.hyperdrive_class);
    println!("Route multiplier : {:.3}", speed.route_multiplier);
    println!("Effective speed  : {:.2} pc/h", effective_speed);

    println!();
    println!(
        "Direct route     : {}",
        if route.direct_route_has_collision {
            "unsafe"
        } else {
            "safe"
        }
    );

    println!(
        "Final route      : {}",
        if route.used_detour {
            if route.detour_is_safe {
                "safe"
            } else {
                "unsafe"
            }
        } else if route.direct_route_has_collision {
            "unsafe"
        } else {
            "safe"
        }
    );

    println!("Total iterations : {}", route.total_iterations);

    print!("Final collision  :");
    if let Some(final_collision) = &route.final_collision {
        println!(
            "  obstacle       : {} [{}]",
            final_collision.obstacle_name, final_collision.obstacle_id
        );
        println!(
            "  center         : ({:.3}, {:.3})",
            final_collision.obstacle_x, final_collision.obstacle_y
        );
        println!("  closest dist   : {:.3}", final_collision.closest_distance);
        println!(
            "  required       : {:.3}",
            final_collision.required_clearance
        );
        println!("  t              : {:.3}", final_collision.t);
    } else {
        println!(" none");
    }

    if route.used_detour {
        println!("Final distance   : {:.6} pc", route.final_distance_parsec);
        println!(
            "Final ETA        : {}",
            format_eta_dd_hh_mm_ss(route.final_eta_seconds)
        );
    }

    println!();
    println!("Route quality:");
    println!(
        "  waypoint count : {}",
        route.quality_metrics.waypoint_count
    );
    println!(
        "  detour overhead: {:.6} pc",
        route.quality_metrics.detour_overhead_pc
    );
    println!(
        "  max turn pen.  : {:.6}",
        route.quality_metrics.max_turn_penalty
    );
    println!(
        "  total turn pen.: {:.6}",
        route.quality_metrics.total_turn_penalty
    );
    println!(
        "  total prox pen.: {:.6}",
        route.quality_metrics.total_proximity_penalty
    );
    println!(
        "  max offset pen.: {:.6}",
        route.quality_metrics.max_offset_penalty
    );
    println!(
        "  total off. pen.: {:.6}",
        route.quality_metrics.total_offset_penalty
    );

    if let Some(v) = &route.closest_violation {
        println!();
        println!("Direct collision:");
        println!("  obstacle       : {} [{}]", v.obstacle_name, v.obstacle_id);
        println!(
            "  center         : ({:.3}, {:.3})",
            v.obstacle_x, v.obstacle_y
        );
        println!("  closest dist   : {:.3} pc", v.closest_distance);
        println!("  required       : {:.3} pc", v.required_clearance);
        println!(
            "  closest point  : ({:.3}, {:.3})",
            v.closest_point.x, v.closest_point.y
        );
        println!("  segment t      : {:.3}", v.t);
    }

    if let Some(explain) = &route.collision_explain {
        println!();
        println!("Collision explain:");
        println!(
            "  obstacle       : {} [{}]",
            explain.obstacle_name, explain.obstacle_id
        );
        println!(
            "  center         : ({:.3}, {:.3})",
            explain.obstacle_x, explain.obstacle_y
        );
        println!("  radius         : {:.3}", explain.obstacle_radius);
        println!("  closest dist   : {:.3}", explain.closest_distance);
        println!("  required       : {:.3}", explain.required_clearance);
        println!("  violated by    : {:.3}", explain.violated_by);
        println!(
            "  closest point  : ({:.3}, {:.3})",
            explain.closest_point.x, explain.closest_point.y
        );
        println!("  t              : {:.3}", explain.t);
        println!("  penalty        : {:.3}", explain.proximity_penalty);
    }

    match &route.detour_candidate {
        Some(candidate) => {
            println!();
            println!("Last selected detour:");
            println!(
                "  waypoint       : ({:.3}, {:.3})",
                candidate.waypoint.x, candidate.waypoint.y
            );
            println!("  side           : {}", candidate.side);
            println!("  offset used    : {:.3}", candidate.offset_used);
            println!("  score          : {:.6}", candidate.total_score);
            println!("  base distance  : {:.6}", candidate.base_distance);
            println!("  turn penalty   : {:.6}", candidate.turn_penalty);
            println!("  back penalty   : {:.6}", candidate.back_penalty);
            println!("  proximity pen. : {:.6}", candidate.proximity_penalty);
            println!("  offset penalty : {:.6}", candidate.offset_penalty);
        }
        None => {
            println!();
            println!("Selected detour  : none");
        }
    }

    if route.iterations.is_empty() {
        println!();
        println!("Routing iterations: none");
    } else {
        println!();
        println!("Routing iterations:");

        for step in &route.iterations {
            println!("  Iteration {}:", step.iteration);
            println!("    segment       : {}", step.segment_index);
            println!(
                "    obstacle      : {} [{}]",
                step.collision.obstacle_name, step.collision.obstacle_id
            );
            println!(
                "    center        : ({:.3}, {:.3})",
                step.collision.obstacle_x, step.collision.obstacle_y
            );
            println!("    closest dist  : {:.3}", step.collision.closest_distance);
            println!(
                "    required      : {:.3}",
                step.collision.required_clearance
            );
            println!("    t             : {:.3}", step.collision.t);

            match &step.selected_candidate {
                Some(selected) => {
                    println!(
                        "    selected      : side={} offset={:.3} score={:.6}",
                        selected.side, selected.offset_used, selected.total_score
                    );
                    println!(
                        "                    waypoint=({:.3}, {:.3})",
                        selected.waypoint.x, selected.waypoint.y
                    );
                }
                None => {
                    println!("    selected      : none");
                }
            }

            if step.candidates.is_empty() {
                println!("    candidates    : none");
            } else {
                println!("    candidates:");

                for (index, candidate) in step.candidates.iter().enumerate() {
                    println!(
                        "      {:02}) side={} offset={:.3} valid={} score={:.6}",
                        index + 1,
                        candidate.side,
                        candidate.offset_used,
                        candidate.is_valid,
                        candidate.total_score
                    );

                    if let Some(reason) = &candidate.rejection_reason {
                        println!("          reason  : {reason}");
                    } else {
                        println!(
                            "          breakdown: base={:.6} turn={:.6} back={:.6} prox={:.6} off={:.6}",
                            candidate.base_distance,
                            candidate.turn_penalty,
                            candidate.back_penalty,
                            candidate.proximity_penalty,
                            candidate.offset_penalty
                        );
                    }
                }
            }
        }
    }

    if !route.final_path.is_empty() {
        println!();
        println!("Final path:");

        for (i, p) in route.final_path.iter().enumerate() {
            println!("  {:02}) ({:.3}, {:.3})", i, p.x, p.y);
        }
    }
}

/// Renders a list of recently saved routes.
pub fn show_recent_routes(routes: &[RecentRouteRow]) {
    println!();
    println!("== Recent routes ==");

    if routes.is_empty() {
        println!("No saved routes found.");
        return;
    }

    for route in routes {
        println!(
            "[{}] {} -> {} | {:.3} pc | ETA {} | safe={} | iters={} | {}",
            route.id,
            route.from_planet_name,
            route.to_planet_name,
            route.final_distance_pc,
            format_eta_dd_hh_mm_ss(route.final_eta_seconds as u64),
            route.final_is_safe,
            route.total_iterations,
            route.created_at_utc
        );
    }
}

/// Renders one saved route in detail.
pub fn show_saved_route_details(route: &SavedRouteDetails) {
    println!();
    println!("== Saved route details ==");
    println!("ID               : {}", route.id);
    println!(
        "From             : {} [{}]",
        route.from_planet_name, route.from_planet_id
    );
    println!(
        "To               : {} [{}]",
        route.to_planet_name, route.to_planet_id
    );
    println!("Created at       : {}", route.created_at_utc);
    println!("Direct distance  : {:.6} pc", route.direct_distance_pc);
    println!("Final distance   : {:.6} pc", route.final_distance_pc);
    println!(
        "Direct ETA       : {}",
        format_eta_dd_hh_mm_ss(route.direct_eta_seconds as u64)
    );
    println!(
        "Final ETA        : {}",
        format_eta_dd_hh_mm_ss(route.final_eta_seconds as u64)
    );
    println!(
        "Direct route     : {}",
        if route.direct_is_safe {
            "safe"
        } else {
            "unsafe"
        }
    );
    println!(
        "Final route      : {}",
        if route.final_is_safe {
            "safe"
        } else {
            "unsafe"
        }
    );
    println!("Total iterations : {}", route.total_iterations);

    // Show final path only if no explain JSON is available
    let has_explain = route
        .route_explain_json
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_some();

    if !has_explain {
        println!();
        println!("Final path:");

        if route.points.is_empty() {
            println!("  none");
        } else {
            for point in &route.points {
                println!("  {:02}) ({:.3}, {:.3})", point.seq_index, point.x, point.y);
            }
        }
    }
}

/// Renders a saved route explain snapshot loaded from JSON.
#[allow(dead_code)]
pub fn show_saved_route_explain(explain: &SavedRouteExplain) {
    println!();
    println!("== Saved route explain ==");

    println!("Direct route     : {}", explain.direct_route_status);
    println!("Final route      : {}", explain.final_route_status);
    println!("Total iterations : {}", explain.total_iterations);

    match &explain.final_collision {
        Some(collision) => {
            println!(
                "Final collision  : {} [{}]",
                collision.obstacle_name, collision.obstacle_id
            );
        }
        None => {
            println!("Final collision  : none");
        }
    }

    println!();
    println!("Route quality:");
    println!("  waypoint count : {}", explain.quality.waypoint_count);
    println!(
        "  detour overhead: {:.6} pc",
        explain.quality.detour_overhead_pc
    );
    println!("  max turn pen.  : {:.6}", explain.quality.max_turn_penalty);
    println!(
        "  total turn pen.: {:.6}",
        explain.quality.total_turn_penalty
    );
    println!(
        "  total prox pen.: {:.6}",
        explain.quality.total_proximity_penalty
    );
    println!(
        "  max offset pen.: {:.6}",
        explain.quality.max_offset_penalty
    );
    println!(
        "  total off. pen.: {:.6}",
        explain.quality.total_offset_penalty
    );

    if let Some(collision) = &explain.direct_collision {
        println!();
        println!("Direct collision:");
        println!(
            "  obstacle       : {} [{}]",
            collision.obstacle_name, collision.obstacle_id
        );
        println!(
            "  center         : ({:.3}, {:.3})",
            collision.obstacle_x, collision.obstacle_y
        );
        println!("  closest dist   : {:.3}", collision.closest_distance);
        println!("  required       : {:.3}", collision.required_clearance);
        println!(
            "  closest point  : ({:.3}, {:.3})",
            collision.closest_point_x, collision.closest_point_y
        );
        println!("  t              : {:.3}", collision.t);
    }

    if let Some(collision) = &explain.collision_explain {
        println!();
        println!("Collision explain:");
        println!(
            "  obstacle       : {} [{}]",
            collision.obstacle_name, collision.obstacle_id
        );
        println!(
            "  center         : ({:.3}, {:.3})",
            collision.obstacle_x, collision.obstacle_y
        );
        println!("  radius         : {:.3}", collision.obstacle_radius);
        println!("  closest dist   : {:.3}", collision.closest_distance);
        println!("  required       : {:.3}", collision.required_clearance);
        println!("  violated by    : {:.3}", collision.violated_by);
        println!(
            "  closest point  : ({:.3}, {:.3})",
            collision.closest_point_x, collision.closest_point_y
        );
        println!("  t              : {:.3}", collision.t);
        println!("  penalty        : {:.3}", collision.proximity_penalty);
    }

    if let Some(detour) = &explain.last_selected_detour {
        println!();
        println!("Last selected detour:");
        println!(
            "  waypoint       : ({:.3}, {:.3})",
            detour.waypoint_x, detour.waypoint_y
        );
        println!("  side           : {}", detour.side);
        println!("  offset used    : {:.3}", detour.offset_used);
        println!("  score          : {:.6}", detour.score);
        println!("  base distance  : {:.6}", detour.base_distance);
        println!("  turn penalty   : {:.6}", detour.turn_penalty);
        println!("  back penalty   : {:.6}", detour.back_penalty);
        println!("  proximity pen. : {:.6}", detour.proximity_penalty);
        println!("  offset penalty : {:.6}", detour.offset_penalty);
    }

    if explain.iterations.is_empty() {
        println!();
        println!("Routing iterations: none");
    } else {
        println!();
        println!("Routing iterations:");

        for step in &explain.iterations {
            println!("  Iteration {}:", step.iteration);
            println!("    segment       : {}", step.segment_index);
            println!(
                "    obstacle      : {} [{}]",
                step.collision.obstacle_name, step.collision.obstacle_id
            );
            println!(
                "    center        : ({:.3}, {:.3})",
                step.collision.obstacle_x, step.collision.obstacle_y
            );
            println!("    closest dist  : {:.3}", step.collision.closest_distance);
            println!(
                "    required      : {:.3}",
                step.collision.required_clearance
            );
            println!("    t             : {:.3}", step.collision.t);

            match &step.selected_candidate {
                Some(selected) => {
                    println!(
                        "    selected      : side={} offset={:.3} score={:.6}",
                        selected.side, selected.offset_used, selected.score
                    );
                    println!(
                        "                    waypoint=({:.3}, {:.3})",
                        selected.waypoint_x, selected.waypoint_y
                    );
                }
                None => {
                    println!("    selected      : none");
                }
            }

            if step.candidates.is_empty() {
                println!("    candidates    : none");
            } else {
                println!("    candidates:");

                for (index, candidate) in step.candidates.iter().enumerate() {
                    println!(
                        "      {:02}) side={} offset={:.3} valid={} score={:.6}",
                        index + 1,
                        candidate.side,
                        candidate.offset_used,
                        candidate.is_valid,
                        candidate.score
                    );

                    if let Some(reason) = &candidate.rejection_reason {
                        println!("          reason  : {reason}");
                    } else {
                        println!(
                            "          breakdown: base={:.6} turn={:.6} back={:.6} prox={:.6} off={:.6}",
                            candidate.base_distance,
                            candidate.turn_penalty,
                            candidate.back_penalty,
                            candidate.proximity_penalty,
                            candidate.offset_penalty
                        );
                    }
                }
            }
        }
    }

    println!();
    println!("Final path:");
    if explain.final_path.is_empty() {
        println!("  none");
    } else {
        for (index, point) in explain.final_path.iter().enumerate() {
            println!("  {:02}) ({:.3}, {:.3})", index, point.x, point.y);
        }
    }
}

pub fn show_database_status(status: &DatabaseStatus) {
    println!("== Database status ==");

    println!("✅ Status: OK");
    println!("ℹ️ Database path: {}", status.db_path);
    println!("ℹ️ Database size: {} bytes", status.db_size_bytes);

    println!();
    println!("Meta:");
    for (k, v) in &status.meta {
        println!("  {k}: {v}");
    }

    println!();
    println!("Counts:");
    println!("  planets: {}", status.counts.planets);

    match status.counts.active_planets {
        Some(v) => println!("  active_planets: {v}"),
        None => println!("  active_planets: n/a"),
    }

    match status.counts.deleted_planets {
        Some(v) => println!("  deleted_planets: {v}"),
        None => println!("  deleted_planets: n/a"),
    }

    match status.counts.planets_unknown {
        Some(v) => println!("  planets_unknown: {v}"),
        None => println!("  planets_unknown: n/a"),
    }

    match status.counts.planet_aliases {
        Some(v) => println!("  planet_aliases: {v}"),
        None => println!("  planet_aliases: n/a"),
    }

    match status.counts.planet_search {
        Some(v) => println!("  planet_search: {v}"),
        None => println!("  planet_search: n/a"),
    }

    println!();
    println!("Schema:");
    for (name, present) in &status.schema_objects {
        println!("  {name}: {}", if *present { "present" } else { "missing" });
    }

    println!();
    println!("FTS:");
    for (k, v) in &status.fts_info {
        println!("  {k}: {v}");
    }
}

/// Renders a compact route explain optimized for small displays.
///
/// This view intentionally focuses on:
/// - final route outcome
/// - iteration count
/// - waypoint count
/// - detour overhead
/// - selected detours
/// - final path
pub fn show_route_result_compact(from: &str, to: &str, route: &RouteSummary) {
    println!();
    println!("== Route compact ==");
    println!("{from} -> {to}");

    let final_status = if route.detour_is_safe {
        "safe"
    } else {
        "unsafe"
    };

    println!(
        "{}  iter:{}  wp:{}",
        final_status, route.total_iterations, route.quality_metrics.waypoint_count
    );

    println!("{:.3} pc", route.final_distance_parsec);
    println!("{}", format_eta_dd_hh_mm_ss(route.final_eta_seconds));
    println!("+{:.3} pc", route.quality_metrics.detour_overhead_pc);

    println!();
    println!("== Detours ==");

    if route.iterations.is_empty() {
        println!("none");
    } else {
        for step in &route.iterations {
            match &step.selected_candidate {
                Some(selected) => {
                    let side = match selected.side.as_str() {
                        "left" => "L",
                        "right" => "R",
                        other => other,
                    };

                    println!(
                        "#{} {} {} {:.2}",
                        step.iteration, step.collision.obstacle_name, side, selected.offset_used
                    );
                }
                None => {
                    println!("#{} {} none", step.iteration, step.collision.obstacle_name);
                }
            }
        }
    }

    println!();
    println!("== Path ==");

    if route.final_path.is_empty() {
        println!("none");
    } else {
        for (index, point) in route.final_path.iter().enumerate() {
            println!("{} ({:.0},{:.0})", index, point.x, point.y);
        }
    }
}

/// Renders a compact saved route explain optimized for small displays.
pub fn show_saved_route_explain_compact(
    from: &str,
    to: &str,
    explain: &SavedRouteExplain,
    final_distance_parsec: f64,
    final_eta_seconds: i64,
) {
    println!("== Route compact ==");
    println!("{from} -> {to}");

    println!(
        "{}  iter:{}  wp:{}",
        explain.final_route_status, explain.total_iterations, explain.quality.waypoint_count
    );

    println!("{:.3} pc", final_distance_parsec);
    println!("{}", format_eta_dd_hh_mm_ss(final_eta_seconds as u64));
    println!("+{:.3} pc", explain.quality.detour_overhead_pc);

    println!();
    println!("== Detours ==");

    if explain.iterations.is_empty() {
        println!("none");
    } else {
        for step in &explain.iterations {
            match &step.selected_candidate {
                Some(selected) => {
                    let side = match selected.side.as_str() {
                        "left" => "L",
                        "right" => "R",
                        other => other,
                    };

                    println!(
                        "#{} {} {} {:.2}",
                        step.iteration, step.collision.obstacle_name, side, selected.offset_used
                    );
                }
                None => {
                    println!("#{} {} none", step.iteration, step.collision.obstacle_name);
                }
            }
        }
    }

    println!();
    println!("== Path ==");

    if explain.final_path.is_empty() {
        println!("none");
    } else {
        for (index, point) in explain.final_path.iter().enumerate() {
            println!("{} ({:.0},{:.0})", index, point.x, point.y);
        }
    }
}
