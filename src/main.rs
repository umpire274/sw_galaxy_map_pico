//! Application entry point for `sw_galaxy_map_pico`.

mod app;
mod commands;
mod config;
mod db;
#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod nav;
mod net;
mod provision;
mod ui;
mod utils;

use anyhow::Result;
use clap::Parser;
use config::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::GrabPlanets { dry_run }) => {
            commands::grab_planets::run(&cli.galaxy_db, *dry_run)?;
        }
        None => {
            let mut app = app::App::bootstrap(cli)?;
            app.run()?;
        }
    }

    Ok(())
}
