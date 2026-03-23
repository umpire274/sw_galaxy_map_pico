//! Input helpers for the textual user interface.
use anyhow::Result;
use std::io;
use std::io::Write;

/// Logical input event placeholder for future keyboard integration.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    /// Move focus up.
    Up,
    /// Move focus down.
    Down,
    /// Confirm current selection.
    Enter,
    /// Cancel or go back.
    Back,
}

/// Standard prompt shown when the user can go back by pressing ENTER.
pub const ENTER_TO_GO_BACK_PROMPT: &str = "(ENTER to go back): ";

/// Reads one trimmed line from standard input after showing a prompt.
pub fn prompt_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

/// Reads one trimmed line using the standard "go back" prompt.
pub fn prompt_go_back() -> Result<String> {
    println!();
    prompt_line(ENTER_TO_GO_BACK_PROMPT)
}

/// Returns true when the input should be interpreted as "back" or "exit".
///
/// The following inputs are treated as equivalent:
/// - empty input (ENTER)
/// - `0`
pub fn is_back_input(input: &str) -> bool {
    input.trim().is_empty() || input.trim() == "0"
}
