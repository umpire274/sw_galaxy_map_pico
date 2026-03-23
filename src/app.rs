//! Application state and orchestration.

use crate::config::Cli;
use crate::db::Database;
use crate::ui;
use anyhow::Result;

/// Central application object.
pub struct App {
    /// Database facade used by the application.
    db: Database,
}

impl App {
    /// Bootstraps the application and validates the configured databases.
    pub fn bootstrap(cli: Cli) -> Result<Self> {
        let db = Database::new(&cli.galaxy_db, &cli.history_db)?;
        Ok(Self { db })
    }

    /// Runs the initial textual shell.
    pub fn run(&mut self) -> Result<()> {
        ui::show_banner();
        ui::show_main_menu();

        let counts = self.db.get_database_counts()?;
        println!("\nDatabase status:");
        println!("  planets        : {}", counts.planets);
        println!("  aliases        : {}", counts.aliases);
        println!("  history entries: {}", counts.history_entries);

        Ok(())
    }
}
