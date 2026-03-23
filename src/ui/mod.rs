//! Textual user interface modules.

pub mod input;
pub mod menu;
pub mod screens;

pub use input::{is_back_input, prompt_go_back, prompt_line};
pub use menu::show_main_menu;
pub use screens::{
    show_banner, show_planet_details, show_search_results,
    show_section_title, show_search_results_screen
};
