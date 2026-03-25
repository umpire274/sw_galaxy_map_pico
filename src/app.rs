//! Application state and orchestration.

use anyhow::Result;

use crate::config::Cli;
use crate::db::Database;
use crate::db::mapper::convert_to_nav_planet;
use crate::db::meta::{meta_set, meta_set_if_absent};
use crate::db::migrate::migrate_history_db;
use crate::db::planets::{PlanetDetails, get_planet_details, search_planets};
use crate::db::queries::{ObstacleQueryBounds, list_routing_obstacles_in_bbox};
use crate::db::routes::{
    SaveRouteEndpoints, SaveRouteOutcome, get_route_details, list_recent_routes,
};
use crate::nav::models::{RouteRequest, SpeedProfile};
use crate::nav::route::{build_saved_route_explain, calculate_iterative_route};
use crate::ui;
use crate::utils::time::now_utc_iso;

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

        migrate_history_db(db.history_conn())?;
        println!();

        let now = now_utc_iso();

        // Install time: written only once.
        meta_set_if_absent(db.history_conn(), "installed_utc", &now, &now)?;

        // Current application version: always updated.
        meta_set(
            db.history_conn(),
            "app_version",
            env!("CARGO_PKG_VERSION"),
            &now,
        )?;

        // Last start time: always updated.
        meta_set(db.history_conn(), "last_start_utc", &now, &now)?;

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
                "3" => self.handle_recent_routes()?,
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
    fn handle_search_planet(&self) -> Result<()> {
        let _ = self.search_and_select_planet("Search planet", SelectionMode::ViewOnly)?;
        Ok(())
    }

    /// Starts the reusable planet selection flow and returns the chosen planet.
    fn select_planet(&self, title: &str) -> Result<Option<PlanetDetails>> {
        self.search_and_select_planet(title, SelectionMode::Select)
    }

    /// Handles the route calculation flow.
    fn handle_calculate_route(&mut self) -> Result<()> {
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

        let conn = self.db.galaxy_conn();
        let from_id = from.remote_id;
        let to_id = to.remote_id;

        let mut loader = |min_x: f64, max_x: f64, min_y: f64, max_y: f64| {
            let bounds = ObstacleQueryBounds {
                min_x,
                max_x,
                min_y,
                max_y,
            };

            list_routing_obstacles_in_bbox(conn, bounds, from_id, to_id, 2.0).unwrap_or_default()
        };

        let route = calculate_iterative_route(&request, &mut loader);

        let created_at_utc = now_utc_iso();
        let explain = build_saved_route_explain(&route);
        let route_explain_json = serde_json::to_string(&explain)?;

        let endpoints = SaveRouteEndpoints {
            from_planet_id: from.remote_id,
            from_planet_name: &from.name,
            to_planet_id: to.remote_id,
            to_planet_name: &to.name,
        };

        let save_outcome = crate::db::routes::save_route(
            self.db.history_conn_mut(),
            &endpoints,
            &route,
            Some(route_explain_json.as_str()),
            &created_at_utc,
        )?;

        ui::show_route_result(&from.name, &to.name, &route, speed_profile);

        println!();
        match save_outcome {
            SaveRouteOutcome::Inserted(route_id) => {
                println!("Route saved successfully. ID: {}", route_id);
            }
            SaveRouteOutcome::AlreadyExists(route_id) => {
                println!("Route already present in history. ID: {}", route_id);
            }
        }

        ui::prompt_go_back()?;

        Ok(())
    }

    /// Displays recently saved routes and allows opening one route detail.
    fn handle_recent_routes(&self) -> Result<()> {
        loop {
            ui::show_section_title("Recent routes");

            let routes = list_recent_routes(self.db.history_conn(), 20)?;
            ui::show_recent_routes(&routes);

            let input = ui::prompt_line("\nEnter route ID to open (ENTER or 0 to go back): ")?;

            if ui::is_back_input(&input) {
                return Ok(());
            }

            let route_id: i64 = match input.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid route ID.");
                    ui::prompt_go_back()?;
                    continue;
                }
            };

            let details = get_route_details(self.db.history_conn(), route_id)?;

            match details {
                Some(route) => {
                    ui::show_saved_route_details(&route);

                    if let Some(json) = &route.route_explain_json {
                        let trimmed = json.trim();

                        if !trimmed.is_empty() {
                            match serde_json::from_str::<crate::db::route_explain::SavedRouteExplain>(
                                trimmed,
                            ) {
                                Ok(explain) => {
                                    ui::show_saved_route_explain(&explain);
                                }
                                Err(err) => {
                                    println!();
                                    println!("Saved explain JSON could not be parsed: {}", err);
                                }
                            }
                        }
                    }

                    ui::prompt_go_back()?;
                }
                None => {
                    println!("Route not found.");
                    ui::prompt_go_back()?;
                }
            }
        }
    }

    /// Displays current database counters.
    fn show_database_info(&self) -> Result<()> {
        let status = crate::db::status::collect_database_status(
            self.db.history_conn(),
            self.db.galaxy_conn(),
            self.db.get_galaxy_path(),
        )?;

        ui::show_database_status(&status);

        ui::prompt_go_back()?;
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
                ui::prompt_go_back()?;
                Ok(None)
            }
        }
    }

    /// Shared search flow used both for:
    /// - view-only search (`Search planet`)
    /// - actual selection (`Calculate route`)
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
                ui::prompt_go_back()?;
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
                        ui::prompt_go_back()?;
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
                        ui::prompt_go_back()?;
                    }
                    SelectionMode::Select => {
                        return Ok(Some(details));
                    }
                }
            }
        }
    }
}
