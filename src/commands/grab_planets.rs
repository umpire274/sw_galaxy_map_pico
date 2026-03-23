//! Planet catalog grab/update command.

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use rusqlite::Connection;
use std::time::Duration;

use crate::db::planets::{replace_unknown_planets, upsert_planet};
use crate::db::schema;
use crate::net::connectivity::ensure_arcgis_reachable;
use crate::provision::arcgis::{
    self, collect_skipped_planets, is_valid_planet, map_feature_to_planet, summarize_skipped_rows,
};

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

    let skipped_rows = collect_skipped_planets(&all_features);
    let skipped_summary = summarize_skipped_rows(&skipped_rows);

    println!("Skipped planets detected: {}", skipped_summary.total);
    println!("  missing Planet: {}", skipped_summary.missing_planet);
    println!("  missing X     : {}", skipped_summary.missing_x);
    println!("  missing Y     : {}", skipped_summary.missing_y);

    if !skipped_rows.is_empty() {
        println!("\nFirst skipped rows:");
        for row in skipped_rows.iter().take(10) {
            println!(
                "  fid={:?} planet={:?} x={:?} y={:?} reason={}",
                row.fid, row.planet, row.x, row.y, row.reason
            );
        }
    }

    let valid_planets = all_features
        .iter()
        .filter(|f| is_valid_planet(&f.attributes))
        .map(map_feature_to_planet)
        .collect::<Result<Vec<_>>>()?;

    println!("\nValid planets ready for import: {}", valid_planets.len());

    if dry_run {
        println!("Dry-run enabled: no database changes were committed.");
        return Ok(());
    }

    let mut conn = Connection::open(galaxy_db_path)
        .with_context(|| format!("Failed to open galaxy database: {galaxy_db_path}"))?;

    schema::initialize_galaxy_schema(&conn)?;

    let tx = conn.transaction().context("Failed to start transaction")?;

    for record in &valid_planets {
        upsert_planet(&tx, record)?;
    }

    replace_unknown_planets(&tx, &skipped_rows)?;

    tx.commit().context("Failed to commit transaction")?;

    let imported_planets = crate::db::planets::count_planets(&conn)?;
    let imported_unknown = crate::db::planets::count_unknown_planets(&conn)?;

    println!("Planet grab completed successfully.");
    println!("Imported planets       : {}", imported_planets);
    println!("Synced unknown planets : {}", imported_unknown);

    Ok(())
}
