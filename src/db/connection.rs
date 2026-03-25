//! SQLite connection helpers.

use anyhow::Result;
use rusqlite::Connection;

/// Holds the two SQLite connections used by the application.
pub struct DatabaseConnections {
    /// Readonly or read-mostly galaxy catalog database.
    pub galaxy: Connection,
    /// Writable history database.
    pub history: Connection,
    pub galaxy_path: String,
}

impl DatabaseConnections {
    /// Opens the configured SQLite databases.
    pub fn open(galaxy_db_path: &str, history_db_path: &str) -> Result<Self> {
        let galaxy_path = String::from(galaxy_db_path);

        let galaxy = Connection::open(galaxy_db_path)?;
        let history = Connection::open(history_db_path)?;

        Ok(Self {
            galaxy,
            history,
            galaxy_path,
        })
    }
}
