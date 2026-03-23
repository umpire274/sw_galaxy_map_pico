//! Application state and orchestration.

use crate::config::Cli;
use crate::db::Database;
use crate::db::mapper::convert_to_nav_planet;
use crate::db::planets::{PlanetDetails, get_planet_details, search_planets};
use crate::nav::models::{RouteRequest, SpeedProfile};
use crate::nav::route::calculate_basic_route;
use crate::ui;
use anyhow::Result;

/// Central application object.
pub struct App {
    /// Database facade used by the application.
    db: Database,
}

#[derive(Clone, Copy)]
enum SelectionMode {
    ViewOnly,
    Select,
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
                "2" => self.handle_calculate_route()?,
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
        let _ = self.search_and_select_planet("Search planet", SelectionMode::ViewOnly)?;

        Ok(())
    }

    fn select_planet(&self, title: &str) -> Result<Option<PlanetDetails>> {
        self.search_and_select_planet(title, SelectionMode::Select)
    }

    fn handle_calculate_route(&self) -> Result<()> {
        ui::show_section_title("Calculate route");

        let from = match self.select_planet("Select origin")? {
            Some(value) => value,
            None => return Ok(()),
        };

        let to = match self.select_planet("Select destination")? {
            Some(value) => value,
            None => return Ok(()),
        };

        let from_nav = convert_to_nav_planet(&from);
        let to_nav = convert_to_nav_planet(&to);

        let speed_profile = SpeedProfile {
            base_speed_parsec_per_hour: 35.0,
            hyperdrive_class: 1.0,
            route_multiplier: 0.895,
        };

        let request = RouteRequest {
            from: from_nav,
            to: to_nav,
            speed_profile,
        };

        let route = calculate_basic_route(&request);

        ui::show_route_result(&from.name, &to.name, &route, speed_profile);
        ui::prompt_go_back()?;

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

    /// Fetches planet details or handles the "not found" case.
    ///
    /// Returns:
    /// - `Some(PlanetDetails)` if found
    /// - `None` if not found (after showing message and waiting for user)
    fn fetch_planet_details_or_notify(&self, planet_id: i64) -> Result<Option<PlanetDetails>> {
        match get_planet_details(self.db.galaxy_conn(), planet_id)? {
            Some(details) => Ok(Some(details)),
            None => {
                println!("Planet details not found.");
                let _ = ui::prompt_go_back()?;
                Ok(None)
            }
        }
    }

    fn search_and_select_planet(
        &self,
        title: &str,
        mode: SelectionMode,
    ) -> Result<Option<PlanetDetails>> {
        loop {
            ui::show_section_title(title);

            let query = ui::prompt_line("\nSearch query (ENTER or 0 to go back): ")?;

            if ui::is_back_input(&query) {
                return Ok(None);
            }

            let results = search_planets(self.db.galaxy_conn(), &query)?;

            if results.is_empty() {
                ui::show_search_results(&results);
                let _ = ui::prompt_go_back()?;
                continue;
            }

            if results.len() == 1 {
                let planet_id = results[0].0;

                let details = match self.fetch_planet_details_or_notify(planet_id)? {
                    Some(d) => d,
                    None => continue,
                };

                match mode {
                    SelectionMode::ViewOnly => {
                        ui::show_planet_details(&details);
                        let _ = ui::prompt_go_back()?;
                        continue;
                    }
                    SelectionMode::Select => {
                        println!("Selected planet: {}", details.name);
                        return Ok(Some(details));
                    }
                }
            }

            loop {
                ui::show_search_results_screen(&results);

                let selection =
                    ui::prompt_line("\nSelect result number (ENTER or 0 to go back): ")?;

                if ui::is_back_input(&selection) {
                    break;
                }

                let selected_index: usize = match selection.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Invalid selection.");
                        continue;
                    }
                };

                if selected_index == 0 || selected_index > results.len() {
                    println!("Invalid selection.");
                    continue;
                }

                let planet_id = results[selected_index - 1].0;

                let details = match self.fetch_planet_details_or_notify(planet_id)? {
                    Some(d) => d,
                    None => continue,
                };

                match mode {
                    SelectionMode::ViewOnly => {
                        ui::show_planet_details(&details);
                        let _ = ui::prompt_go_back()?;
                    }
                    SelectionMode::Select => {
                        return Ok(Some(details));
                    }
                }
            }
        }
    }
}
