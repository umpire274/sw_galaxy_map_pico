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

/// Computes the axis-aligned bounding box of a segment.
pub fn segment_bbox(a: Point2, b: Point2) -> (f64, f64, f64, f64) {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    (min_x, max_x, min_y, max_y)
}

/// Normalizes a 2D vector.
///
/// Returns `(0.0, 0.0)` if the vector length is too small.
pub fn normalize2(x: f64, y: f64) -> (f64, f64) {
    let len = (x * x + y * y).sqrt();

    if len <= f64::EPSILON {
        return (0.0, 0.0);
    }

    (x / len, y / len)
}

/// Returns the left-hand normal of a 2D vector.
pub fn left_normal(x: f64, y: f64) -> (f64, f64) {
    (-y, x)
}
