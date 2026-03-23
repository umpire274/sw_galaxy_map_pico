//! ArcGIS download helpers.

use crate::utils::json::{get_f64, get_string};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;

const LAYER_URL: &str =
    "https://services3.arcgis.com/nM57tYg6wB9iTP3P/arcgis/rest/services/planets/FeatureServer/0";

/// Minimal ArcGIS layer info used by the downloader.
#[derive(Debug, Clone)]
pub struct ArcGisLayerInfo {
    /// Maximum number of records returned by the service in one page.
    pub max_record_count: usize,
}

/// Raw ArcGIS feature attributes for a planet record.
///
/// This struct is intentionally permissive because field names and value
/// presence may vary between datasets and over time.
#[derive(Debug, Clone)]
pub struct ArcGisPlanetFeature {
    /// Raw feature attributes as returned by ArcGIS.
    pub attributes: Value,
}

/// Returns the ArcGIS layer metadata endpoint.
pub fn layer_info_url() -> String {
    format!("{LAYER_URL}?f=json")
}

/// Returns the ArcGIS query endpoint.
pub fn query_url() -> String {
    format!("{LAYER_URL}/query")
}

/// Returns true if a planet record is valid for insertion.
///
/// A valid planet must have:
/// - non-empty name
/// - X coordinate
/// - Y coordinate
pub fn is_valid_planet(attributes: &serde_json::Value) -> bool {
    let planet_ok = get_string(attributes, "Planet").is_some();
    let x_ok = get_f64(attributes, "X").is_some();
    let y_ok = get_f64(attributes, "Y").is_some();

    planet_ok && x_ok && y_ok
}

/// Returns true if a planet should be classified as "unknown".
///
/// In this project, unknown planets are those that are skipped
/// due to missing required fields.
pub fn is_unknown(attributes: &Value) -> bool {
    !is_valid_planet(attributes)
}

/// Downloads ArcGIS layer metadata.
pub fn fetch_layer_info(client: &Client) -> Result<ArcGisLayerInfo> {
    let response = client
        .get(layer_info_url())
        .send()
        .context("Failed to fetch ArcGIS layer info")?
        .error_for_status()
        .context("ArcGIS layer info request returned an error status")?;

    let json = response
        .json::<Value>()
        .context("Failed to decode ArcGIS layer info JSON")?;

    let max_record_count = json
        .get("maxRecordCount")
        .and_then(Value::as_u64)
        .unwrap_or(1000) as usize;

    Ok(ArcGisLayerInfo { max_record_count })
}

/// Downloads one page of ArcGIS features.
pub fn fetch_feature_page(
    client: &Client,
    offset: usize,
    limit: usize,
) -> Result<Vec<ArcGisPlanetFeature>> {
    let response = client
        .get(query_url())
        .query(&[
            ("f", "json"),
            ("where", "1=1"),
            ("outFields", "*"),
            ("returnGeometry", "false"),
            ("resultOffset", &offset.to_string()),
            ("resultRecordCount", &limit.to_string()),
        ])
        .send()
        .context("Failed to fetch ArcGIS feature page")?
        .error_for_status()
        .context("ArcGIS feature page request returned an error status")?;

    let json = response
        .json::<Value>()
        .context("Failed to decode ArcGIS feature page JSON")?;

    let features = json
        .get("features")
        .and_then(Value::as_array)
        .context("ArcGIS response does not contain a valid 'features' array")?;

    let parsed = features
        .iter()
        .map(|feature| {
            let attributes = feature
                .get("attributes")
                .cloned()
                .unwrap_or(Value::Object(Default::default()));

            ArcGisPlanetFeature { attributes }
        })
        .collect();

    Ok(parsed)
}

/// Downloads all ArcGIS features using paginated requests.
pub fn fetch_all_features(client: &Client, page_size: usize) -> Result<Vec<ArcGisPlanetFeature>> {
    let mut all_features = Vec::new();
    let mut offset = 0usize;

    loop {
        let page = fetch_feature_page(client, offset, page_size)?;
        let page_len = page.len();

        if page_len == 0 {
            break;
        }

        all_features.extend(page);
        offset += page_len;

        if page_len < page_size {
            break;
        }
    }

    Ok(all_features)
}
