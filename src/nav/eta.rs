//! ETA calculation helpers.

use super::models::SpeedProfile;

/// Computes the effective speed in parsecs per hour.
///
/// Formula:
/// `(base_speed_parsec_per_hour / hyperdrive_class) * route_multiplier`
pub fn effective_speed_parsec_per_hour(speed: SpeedProfile) -> f64 {
    if speed.base_speed_parsec_per_hour <= 0.0
        || speed.hyperdrive_class <= 0.0
        || speed.route_multiplier <= 0.0
    {
        return 0.0;
    }

    (speed.base_speed_parsec_per_hour / speed.hyperdrive_class) * speed.route_multiplier
}

/// Computes the travel time in seconds for a given distance in parsecs.
pub fn estimate_eta_seconds(distance_parsec: f64, speed: SpeedProfile) -> u64 {
    let effective_speed = effective_speed_parsec_per_hour(speed);

    if effective_speed <= 0.0 {
        return 0;
    }

    let hours = distance_parsec / effective_speed;
    (hours * 3600.0).round() as u64
}

/// Formats a duration in seconds as `dd hh mm ss`.
pub fn format_eta_dd_hh_mm_ss(total_seconds: u64) -> String {
    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    format!("{days:02}d {hours:02}h {minutes:02}m {seconds:02}s")
}
