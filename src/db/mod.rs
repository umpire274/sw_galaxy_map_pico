//! Database facade and modules.

pub mod aliases;
pub mod connection;
pub mod history;
pub mod mapper;
pub mod meta;
pub mod migrate;
pub mod planets;
pub mod queries;
pub mod route_explain;
pub mod routes;
pub mod schema;

use anyhow::Result;
use connection::DatabaseConnections;

/// Aggregate counts used to validate database availability.
#[derive(Debug, Clone)]
pub struct DatabaseCounts {
    /// Number of planets stored in the galaxy catalog.
    pub planets: i64,
    /// Number of aliases stored in the galaxy catalog.
    pub aliases: i64,
    /// Number of saved route history entries.
    pub history_entries: i64,
}

/// Main database facade.
pub struct Database {
    /// Underlying database connections.
    pub connections: DatabaseConnections,
}

impl Database {
    /// Creates a new database facade and initializes writable schema.
    pub fn new(galaxy_db_path: &str, history_db_path: &str) -> Result<Self> {
        let connections = DatabaseConnections::open(galaxy_db_path, history_db_path)?;
        println!();
        schema::initialize_galaxy_schema(&connections.galaxy)?;
        schema::initialize_history_schema(&connections.history)?;
        Ok(Self { connections })
    }

    /// Returns the readonly galaxy database connection.
    pub fn galaxy_conn(&self) -> &rusqlite::Connection {
        &self.connections.galaxy
    }

    /// Returns a mutable reference to the history database connection.
    pub fn history_conn_mut(&mut self) -> &mut rusqlite::Connection {
        &mut self.connections.history
    }

    /// Returns an immutable reference to the history database connection.
    pub fn history_conn(&self) -> &rusqlite::Connection {
        &self.connections.history
    }

    /// Returns aggregate counts from both databases.
    pub fn get_database_counts(&self) -> Result<DatabaseCounts> {
        let planets = planets::count_planets(&self.connections.galaxy)?;
        let aliases = aliases::count_aliases(&self.connections.galaxy)?;
        let history_entries = history::count_history_entries(&self.connections.history)?;

        Ok(DatabaseCounts {
            planets,
            aliases,
            history_entries,
        })
    }
}
