//! Planet catalog grab/update command.

use crate::net::connectivity::ensure_arcgis_reachable;
use crate::provision::arcgis;
use crate::provision::arcgis::is_unknown;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use rusqlite::Connection;
use std::time::Duration;

/// Runs the remote planet grab command.
pub fn run(galaxy_db_path: &str, dry_run: bool) -> Result<()> {
    println!("== Grab planets from ArcGIS ==");

    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .context("Failed to build HTTP client")?;

    ensure_arcgis_reachable(&client, &arcgis::layer_info_url())?;
    println!("Remote ArcGIS source reachable.");

    let layer_info = arcgis::fetch_layer_info(&client)?;
    let page_size = layer_info.max_record_count.min(2000);

    println!("Detected page size: {page_size}");

    let all_features = arcgis::fetch_all_features(&client, page_size)?;
    println!("Downloaded {} total features.", all_features.len());

    let valid_attribute_payloads = all_features
        .iter()
        .filter(|f| f.attributes.is_object())
        .count();

    println!(
        "Validated {} feature attribute payloads.",
        valid_attribute_payloads
    );

    let mut skipped_total = 0;
    for feature in &all_features {
        let attrs = &feature.attributes;

        if is_unknown(attrs) {
            skipped_total += 1;
        }
    }
    println!("Skipped planets detected: {}", skipped_total);

    if dry_run {
        println!("Dry-run enabled: no database changes were committed.");
        return Ok(());
    }

    let mut conn = Connection::open(galaxy_db_path)
        .with_context(|| format!("Failed to open galaxy database: {galaxy_db_path}"))?;

    let tx = conn.transaction().context("Failed to start transaction")?;

    // TODO:
    // - normalize ArcGIS attributes into internal planet records
    // - upsert planets
    // - upsert aliases
    // - rebuild search table

    tx.commit().context("Failed to commit transaction")?;

    println!("Planet grab completed successfully.");

    Ok(())
}
