//! Time utilities.

use chrono::Utc;

/// Returns the current UTC timestamp in ISO 8601 format.
///
/// Example: "2026-03-24T10:15:30Z"
pub fn now_utc_iso() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
