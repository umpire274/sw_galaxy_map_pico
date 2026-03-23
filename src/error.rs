//! Project-specific error definitions.

use thiserror::Error;

/// Errors returned by the bootstrap application.
#[derive(Debug, Error)]
pub enum SwgmPicoError {
    /// Returned when a requested entity cannot be found.
    #[error("entity not found: {0}")]
    NotFound(String),

    /// Returned when invalid user input is detected.
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
