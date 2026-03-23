//! Input abstractions placeholder.

/// Logical input event placeholder for future keyboard integration.
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
