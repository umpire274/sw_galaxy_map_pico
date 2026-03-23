//! Textual screen helpers.

use crate::db::planets::PlanetDetails;

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
