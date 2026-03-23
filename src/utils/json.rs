//! JSON helper utilities for ArcGIS and external data parsing.

use serde_json::Value;

/// Extracts a string field from a JSON object.
///
/// Trims whitespace and returns `None` if the value is empty.
pub fn get_string(attributes: &Value, key: &str) -> Option<String> {
    attributes
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
}

/// Extracts a floating-point value from a JSON object.
///
/// Accepts both numeric and string representations.
pub fn get_f64(attributes: &Value, key: &str) -> Option<f64> {
    match attributes.get(key) {
        Some(Value::Number(n)) => n.as_f64(),
        Some(Value::String(s)) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

/// Extracts an integer value from a JSON object.
///
/// Accepts both numeric and string representations.
pub fn get_i64(attributes: &Value, key: &str) -> Option<i64> {
    match attributes.get(key) {
        Some(Value::Number(n)) => n.as_i64(),
        Some(Value::String(s)) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}
