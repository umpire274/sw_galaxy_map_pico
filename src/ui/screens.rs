//! Textual screen helpers.

use crate::db::planets::PlanetDetails;
use crate::db::routes::{RecentRouteRow, SavedRouteDetails};
use crate::nav::eta::{effective_speed_parsec_per_hour, format_eta_dd_hh_mm_ss};
use crate::nav::models::{RouteSummary, SpeedProfile};

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
                            "          breakdown: base={:.6} turn={:.6} back={:.6} prox={:.6}",
                            candidate.base_distance,
                            candidate.turn_penalty,
                            candidate.back_penalty,
                            candidate.proximity_penalty
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
