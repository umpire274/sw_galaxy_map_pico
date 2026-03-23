//! Distance calculation helpers.

use super::models::Planet;

/// Computes the Euclidean distance between two planets.
pub fn euclidean_distance(from: &Planet, to: &Planet) -> f64 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;

    (dx * dx + dy * dy + dz * dz).sqrt()
}
