//! Application state and orchestration.

use crate::config::Cli;
use crate::db::Database;
use crate::db::planets::{get_planet_details, search_planets};
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

    /// Runs the textual shell.
    pub fn run(&mut self) -> Result<()> {
        ui::show_banner();

        loop {
            ui::show_main_menu();

            let choice = ui::prompt_line("\nSelect an option (0 to exit): ")?;

            if ui::is_back_input(&choice) {
                println!("Exiting SW Galaxy Map Pico.");
                break;
            }

            match choice.as_str() {
                "1" => self.handle_search_planet()?,
                "2" => self.show_not_implemented("Calculate route"),
                "3" => self.show_not_implemented("Recent routes"),
                "4" => self.show_not_implemented("Favorites"),
                "5" => self.show_database_info()?,
                "6" => self.show_not_implemented("Settings"),
                _ => {
                    println!("Invalid option. Please try again.");
                }
            }
        }

        Ok(())
    }

    /// Handles the interactive planet search flow.
    /// Handles the interactive planet search flow.
    fn handle_search_planet(&self) -> Result<()> {
        loop {
            ui::show_section_title("Search planet");

            let query = ui::prompt_line("Search query (ENTER or 0 to go back): ")?;

            if ui::is_back_input(&query) {
                break;
            }

            let results = search_planets(self.db.galaxy_conn(), &query)?;

            if results.is_empty() {
                ui::show_search_results(&results);
                ui::prompt_go_back()?;
                continue;
            }

            loop {
                ui::show_search_results_screen(&results);

                let selection =
                    ui::prompt_line("\nSelect result number (ENTER or 0 to go back): ")?;

                if ui::is_back_input(&selection) {
                    break;
                }

                let selected_index: usize = match selection.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid selection. Please enter a valid number.");
                        continue;
                    }
                };

                if selected_index == 0 || selected_index > results.len() {
                    println!("Invalid selection. Please choose a listed result.");
                    continue;
                }

                let planet_id = results[selected_index - 1].0;

                match get_planet_details(self.db.galaxy_conn(), planet_id)? {
                    Some(details) => {
                        ui::show_planet_details(&details);
                        ui::prompt_go_back()?;
                    }
                    None => {
                        println!("Planet details not found.");
                        ui::prompt_go_back()?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Displays current database counters.
    fn show_database_info(&self) -> Result<()> {
        ui::show_section_title("Database info");

        let counts = self.db.get_database_counts()?;
        println!("Planets        : {}", counts.planets);
        println!("Aliases        : {}", counts.aliases);
        println!("History entries: {}", counts.history_entries);

        let _ = ui::prompt_go_back()?;

        Ok(())
    }

    /// Displays a placeholder for unfinished sections.
    fn show_not_implemented(&self, feature_name: &str) {
        ui::show_section_title(feature_name);
        println!("This feature is not implemented yet.");
        let _ = ui::prompt_go_back();
    }
}
