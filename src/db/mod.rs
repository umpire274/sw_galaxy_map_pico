//! Database facade and modules.

pub mod connection;
pub mod mapper;
pub mod meta;
pub mod migrate;
pub mod planets;
pub mod queries;
pub mod route_explain;
pub mod routes;
pub mod schema;
pub mod status;

use anyhow::Result;
use connection::DatabaseConnections;

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

    /// Returns the configured galaxy database path.
    pub fn get_galaxy_path(&self) -> &String {
        &self.connections.galaxy_path
    }

    /// Returns the readonly galaxy database connection.
    pub fn galaxy_conn(&self) -> &rusqlite::Connection {
        &self.connections.galaxy
    }

    /// Returns an immutable reference to the history database connection.
    pub fn history_conn(&self) -> &rusqlite::Connection {
        &self.connections.history
    }

    /// Returns a mutable reference to the history database connection.
    pub fn history_conn_mut(&mut self) -> &mut rusqlite::Connection {
        &mut self.connections.history
    }
}
