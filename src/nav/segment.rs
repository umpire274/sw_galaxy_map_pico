//! Route segment utilities.

use crate::nav::models::Point2;

/// One segment of a route path.
#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub start: Point2,
    pub end: Point2,
}

/// Builds all segments from a route path.
pub fn build_segments(points: &[Point2]) -> Vec<Segment> {
    let mut segments = Vec::new();

    for i in 0..points.len() - 1 {
        segments.push(Segment {
            start: points[i],
            end: points[i + 1],
        });
    }

    segments
}

/// Inserts a waypoint into the path after a segment index.
pub fn insert_waypoint(path: &mut Vec<Point2>, segment_index: usize, waypoint: Point2) {
    path.insert(segment_index + 1, waypoint);
}

/// Computes a bounding box for an entire path.
pub fn path_bbox(points: &[Point2]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for p in points {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    (min_x, max_x, min_y, max_y)
}
