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
