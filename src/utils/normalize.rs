//! Text normalization utilities for search.

/// Normalizes text for case-insensitive comparisons.
///
/// Current strategy:
/// - lowercase
/// - trim whitespace
pub fn normalize_text(input: &str) -> String {
    input.trim().to_lowercase()
}
