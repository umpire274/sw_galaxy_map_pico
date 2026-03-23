//! ETA calculation helpers.

/// Computes the travel time in minutes for a given distance and speed.
pub fn estimate_eta_minutes(distance: f64, speed: f64) -> u64 {
    if speed <= 0.0 {
        return 0;
    }

    ((distance / speed) * 60.0).round() as u64
}
