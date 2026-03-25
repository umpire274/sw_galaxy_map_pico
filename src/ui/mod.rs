//! Textual user interface modules.

pub mod input;
pub mod menu;
pub mod screens;

pub use input::{is_back_input, prompt_go_back, prompt_line};
pub use menu::show_main_menu;
pub use screens::{
    show_banner, show_database_status, show_planet_details, show_recent_routes, show_route_result,
    show_saved_route_details, show_saved_route_explain, show_search_results,
    show_search_results_screen, show_section_title,
};
