//! Mapping helpers between database models and navigation domain models.

use crate::db::planets::PlanetDetails;
use crate::nav::models::Planet;

/// Converts a database planet record into a navigation model.
///
/// This adapter isolates the navigation layer from database-specific
/// structures and allows independent evolution of both layers.
pub fn convert_to_nav_planet(details: &PlanetDetails) -> Planet {
    Planet {
        id: details.remote_id,
        name: details.name.clone(),
        x: details.x,
        y: details.y,
        z: 0.0, // future: support real Z axis if available
    }
}
