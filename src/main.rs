//! Application entry point for `sw_galaxy_map_pico`.
//!
//! This binary currently provides the initial bootstrap shell used to validate
//! the project architecture, database access, and navigation workflow before
//! PicoCalc-specific integration.

mod app;
mod config;
mod db;
#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod nav;
mod ui;

use anyhow::Result;
use clap::Parser;
use config::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut app = app::App::bootstrap(cli)?;
    app.run()?;
    Ok(())
}
