//! Remote connectivity checks for online data sources.

use anyhow::{Context, Result};
use reqwest::blocking::Client;

/// Verifies that the remote ArcGIS service is reachable.
///
/// This function is intentionally source-specific rather than checking
/// generic internet access.
pub fn ensure_arcgis_reachable(client: &Client, url: &str) -> Result<()> {
    let response = client
        .get(url)
        .send()
        .with_context(|| format!("Failed to contact remote ArcGIS endpoint: {url}"))?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Remote ArcGIS endpoint is not reachable (HTTP {})",
            response.status()
        );
    }

    Ok(())
}
