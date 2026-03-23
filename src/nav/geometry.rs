//! Geometry helpers for route analysis.

use crate::nav::models::Point2;

/// Computes the squared Euclidean distance between two 2D points.
pub fn distance2(a: Point2, b: Point2) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dx * dx + dy * dy
}

/// Computes the Euclidean distance between two 2D points.
pub fn distance(a: Point2, b: Point2) -> f64 {
    distance2(a, b).sqrt()
}

/// Computes the closest point on segment AB to point P.
///
/// Returns:
/// - clamped interpolation factor `t` in [0, 1]
/// - closest point on the segment
/// - distance from P to the segment
pub fn closest_point_on_segment(a: Point2, b: Point2, p: Point2) -> (f64, Point2, f64) {
    let abx = b.x - a.x;
    let aby = b.y - a.y;
    let apx = p.x - a.x;
    let apy = p.y - a.y;

    let ab_len2 = abx * abx + aby * aby;

    if ab_len2 <= f64::EPSILON {
        let closest = a;
        let dist = distance(p, closest);
        return (0.0, closest, dist);
    }

    let t = ((apx * abx) + (apy * aby)) / ab_len2;
    let t_clamped = t.clamp(0.0, 1.0);

    let closest = Point2 {
        x: a.x + abx * t_clamped,
        y: a.y + aby * t_clamped,
    };

    let dist = distance(p, closest);

    (t_clamped, closest, dist)
}
