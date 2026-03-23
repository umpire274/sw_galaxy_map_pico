//! ArcGIS download and normalization helpers.

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;

use crate::utils::json::{get_f64, get_i64, get_string};

const LAYER_URL: &str =
    "https://services3.arcgis.com/nM57tYg6wB9iTP3P/arcgis/rest/services/planets/FeatureServer/0";

/// Minimal ArcGIS layer info used by the downloader.
#[derive(Debug, Clone)]
pub struct ArcGisLayerInfo {
    /// Maximum number of records returned by the service in one page.
    pub max_record_count: usize,
}

/// Raw ArcGIS feature attributes for a planet record.
#[derive(Debug, Clone)]
pub struct ArcGisPlanetFeature {
    /// Raw feature attributes as returned by ArcGIS.
    pub attributes: Value,
}

/// Normalized planet record imported from ArcGIS.
#[derive(Debug, Clone)]
pub struct RemotePlanetRecord {
    /// Remote ArcGIS feature identifier.
    pub remote_id: i64,
    /// Canonical planet name.
    pub name: String,
    /// Optional region name.
    pub region: Option<String>,
    /// Optional sector name.
    pub sector: Option<String>,
    /// Optional system name.
    pub system_name: Option<String>,
    /// Optional grid name.
    pub grid: Option<String>,
    /// Required X coordinate.
    pub x: f64,
    /// Required Y coordinate.
    pub y: f64,
    /// Optional canon flag.
    pub canon: Option<i64>,
    /// Optional legends flag.
    pub legends: Option<i64>,
    /// Optional zm value.
    pub zm: Option<i64>,
    /// Optional alias field name0.
    pub name0: Option<String>,
    /// Optional alias field name1.
    pub name1: Option<String>,
    /// Optional alias field name2.
    pub name2: Option<String>,
    /// Optional latitude.
    pub lat: Option<f64>,
    /// Optional longitude.
    pub long: Option<f64>,
    /// Optional reference.
    pub ref_code: Option<String>,
    /// Optional status.
    pub status: Option<String>,
    /// Optional canonical region field.
    pub c_region: Option<String>,
    /// Optional canonical region label field.
    pub c_region_li: Option<String>,
}

/// Describes one skipped planet row from ArcGIS import.
#[derive(Debug, Clone)]
pub struct SkippedPlanetRow {
    /// Optional remote identifier.
    pub fid: Option<i64>,
    /// Optional planet name.
    pub planet: Option<String>,
    /// Optional X coordinate.
    pub x: Option<f64>,
    /// Optional Y coordinate.
    pub y: Option<f64>,
    /// Comma-separated skip reason list.
    pub reason: String,
}

/// Aggregate summary for skipped rows.
#[derive(Debug, Clone, Default)]
pub struct SkippedSummary {
    /// Total skipped rows.
    pub total: usize,
    /// Rows missing planet name.
    pub missing_planet: usize,
    /// Rows missing X coordinate.
    pub missing_x: usize,
    /// Rows missing Y coordinate.
    pub missing_y: usize,
}

/// Returns the ArcGIS layer metadata endpoint.
pub fn layer_info_url() -> String {
    format!("{LAYER_URL}?f=json")
}

/// Returns the ArcGIS query endpoint.
pub fn query_url() -> String {
    format!("{LAYER_URL}/query")
}

/// Downloads ArcGIS layer metadata.
pub fn fetch_layer_info(client: &Client) -> Result<ArcGisLayerInfo> {
    let json = client
        .get(layer_info_url())
        .send()
        .context("Failed to fetch ArcGIS layer info")?
        .error_for_status()
        .context("ArcGIS layer info request returned an error status")?
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
    let json = client
        .get(query_url())
        .query(&[
            ("f", "json"),
            ("where", "1=1"),
            ("outFields", "*"),
            ("returnGeometry", "false"),
            ("orderByFields", "FID"),
            ("resultOffset", &offset.to_string()),
            ("resultRecordCount", &limit.to_string()),
        ])
        .send()
        .context("Failed to fetch ArcGIS feature page")?
        .error_for_status()
        .context("ArcGIS feature page request returned an error status")?
        .json::<Value>()
        .context("Failed to decode ArcGIS feature page JSON")?;

    let features = json
        .get("features")
        .and_then(Value::as_array)
        .context("ArcGIS response does not contain a valid 'features' array")?;

    let parsed = features
        .iter()
        .map(|feature| ArcGisPlanetFeature {
            attributes: feature
                .get("attributes")
                .cloned()
                .unwrap_or(Value::Object(Default::default())),
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

/// Returns true if a planet record is valid for insertion.
///
/// A valid planet must have:
/// - non-empty name
/// - X coordinate
/// - Y coordinate
pub fn is_valid_planet(attributes: &Value) -> bool {
    let planet_ok = get_string(attributes, "Planet").is_some();
    let x_ok = get_f64(attributes, "X").is_some();
    let y_ok = get_f64(attributes, "Y").is_some();

    planet_ok && x_ok && y_ok
}

/// Returns true if a planet should be classified as unknown.
///
/// In this project, unknown planets are rows skipped due to missing
/// required fields for the main `planets` table.
pub fn is_unknown(attributes: &Value) -> bool {
    !is_valid_planet(attributes)
}

/// Builds a skipped-row representation for a feature that cannot be imported.
pub fn build_skipped_planet_row(feature: &ArcGisPlanetFeature) -> Option<SkippedPlanetRow> {
    let attrs = &feature.attributes;

    if !is_unknown(attrs) {
        return None;
    }

    let fid = get_i64(attrs, "FID");
    let planet = get_string(attrs, "Planet");
    let x = get_f64(attrs, "X");
    let y = get_f64(attrs, "Y");

    let mut reasons = Vec::new();

    if planet.is_none() {
        reasons.push("missing_planet");
    }
    if x.is_none() {
        reasons.push("missing_x");
    }
    if y.is_none() {
        reasons.push("missing_y");
    }

    Some(SkippedPlanetRow {
        fid,
        planet,
        x,
        y,
        reason: reasons.join(","),
    })
}

/// Collects all skipped rows from a feature list.
pub fn collect_skipped_planets(features: &[ArcGisPlanetFeature]) -> Vec<SkippedPlanetRow> {
    features
        .iter()
        .filter_map(build_skipped_planet_row)
        .collect()
}

/// Summarizes skipped rows for reporting.
pub fn summarize_skipped_rows(rows: &[SkippedPlanetRow]) -> SkippedSummary {
    let mut summary = SkippedSummary::default();

    for row in rows {
        summary.total += 1;

        if row.reason.contains("missing_planet") {
            summary.missing_planet += 1;
        }
        if row.reason.contains("missing_x") {
            summary.missing_x += 1;
        }
        if row.reason.contains("missing_y") {
            summary.missing_y += 1;
        }
    }

    summary
}

/// Maps one valid ArcGIS feature into a normalized planet record.
pub fn map_feature_to_planet(feature: &ArcGisPlanetFeature) -> Result<RemotePlanetRecord> {
    let attrs = &feature.attributes;

    let remote_id = get_i64(attrs, "FID").context("ArcGIS feature does not contain a valid FID")?;

    let name = get_string(attrs, "Planet")
        .context("ArcGIS feature does not contain a valid planet name")?;

    let x = get_f64(attrs, "X").context("ArcGIS feature does not contain a valid X coordinate")?;

    let y = get_f64(attrs, "Y").context("ArcGIS feature does not contain a valid Y coordinate")?;

    Ok(RemotePlanetRecord {
        remote_id,
        name,
        region: get_string(attrs, "Region"),
        sector: get_string(attrs, "Sector"),
        system_name: get_string(attrs, "System"),
        grid: get_string(attrs, "Grid"),
        x,
        y,
        canon: get_i64(attrs, "Canon"),
        legends: get_i64(attrs, "Legends"),
        zm: get_i64(attrs, "zm"),
        name0: get_string(attrs, "name0"),
        name1: get_string(attrs, "name1"),
        name2: get_string(attrs, "name2"),
        lat: get_f64(attrs, "lat"),
        long: get_f64(attrs, "long"),
        ref_code: get_string(attrs, "ref"),
        status: get_string(attrs, "status"),
        c_region: get_string(attrs, "CRegion"),
        c_region_li: get_string(attrs, "CRegion_li"),
    })
}
