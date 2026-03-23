//! Textual screen helpers.

use crate::db::planets::PlanetDetails;
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
    println!("Distance         : {:.2} pc", route.distance_parsec);
    println!(
        "ETA              : {}",
        format_eta_dd_hh_mm_ss(route.eta_seconds)
    );
    println!(
        "Base speed       : {:.2} pc/h",
        speed.base_speed_parsec_per_hour
    );
    println!("Hyperdrive class : {:.2}", speed.hyperdrive_class);
    println!("Route multiplier : {:.3}", speed.route_multiplier);
    println!("Effective speed  : {:.2} pc/h", effective_speed);

    match &route.closest_violation {
        Some(v) => {
            println!();
            println!("Obstacle warning : YES");
            println!("Closest obstacle : {} [{}]", v.obstacle_name, v.obstacle_id);
            println!("Closest distance : {:.3} pc", v.closest_distance);
            println!("Required minimum : {:.3} pc", v.required_clearance);
            println!(
                "Closest point    : ({:.3}, {:.3})",
                v.closest_point.x, v.closest_point.y
            );
            println!("Segment factor t : {:.3}", v.t);
        }
        None => {
            println!();
            println!("Obstacle warning : NO");
        }
    }
}
