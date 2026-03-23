//! Distance calculation helpers.

use super::models::Planet;

/// Conversion factor from raw map units to parsecs.
///
/// Set this to the correct value used by the project reference dataset.
/// If the ArcGIS coordinates are already expressed in parsecs, keep `1.0`.
pub const PARSEC_SCALE: f64 = 1.0;

/// Computes the Euclidean distance between two planets in raw map units.
pub fn euclidean_distance_raw(from: &Planet, to: &Planet) -> f64 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Converts a raw map distance into parsecs.
pub fn raw_distance_to_parsec(raw_distance: f64) -> f64 {
    raw_distance * PARSEC_SCALE
}

/// Computes the Euclidean distance between two planets in parsecs.
pub fn euclidean_distance_parsec(from: &Planet, to: &Planet) -> f64 {
    raw_distance_to_parsec(euclidean_distance_raw(from, to))
}
