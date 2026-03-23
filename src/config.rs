//! CLI configuration and startup options.

use clap::{Parser, Subcommand};

/// Startup configuration for the bootstrap application.
#[derive(Debug, Parser)]
#[command(
    name = "sw_galaxy_map_pico",
    version,
    about = "Bootstrap navicomputer for Star Wars galaxy routing"
)]
pub struct Cli {
    /// Path to the readonly galaxy database.
    #[arg(long, default_value = "assets/db/galaxy.db")]
    pub galaxy_db: String,

    /// Path to the writable history database.
    #[arg(long, default_value = "assets/db/history.db")]
    pub history_db: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Supported CLI commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Downloads or refreshes planet data from the ArcGIS source.
    GrabPlanets {
        /// Runs the download without committing changes.
        #[arg(long)]
        dry_run: bool,
    },
}
