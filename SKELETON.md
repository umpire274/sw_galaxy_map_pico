# sw_galaxy_map_pico — Scheletro iniziale

## Struttura del progetto

```text
sw_galaxy_map_pico/
├── Cargo.toml
├── README.md
├── ROADMAP.md
├── .gitignore
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── error.rs
│   ├── config.rs
│   ├── nav/
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── distance.rs
│   │   ├── eta.rs
│   │   └── route.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── schema.rs
│   │   ├── planets.rs
│   │   ├── aliases.rs
│   │   └── history.rs
│   └── ui/
│       ├── mod.rs
│       ├── menu.rs
│       ├── screens.rs
│       └── input.rs
└── assets/
    └── db/
        ├── .gitkeep
        └── README.md
```

---

## 1. `Cargo.toml`

```toml
[package]
name = "sw_galaxy_map_pico"
version = "0.1.0"
edition = "2024"
authors = ["Alessandro Maestri"]
description = "Pico-oriented offline Star Wars galaxy route planner and navicomputer"
license = "MIT OR Apache-2.0"
readme = "README.md"
repository = "https://github.com/umpire274/sw_galaxy_map_pico"
keywords = ["star-wars", "sqlite", "navigation", "route-planner", "picocalc"]
categories = ["command-line-utilities", "database"]

[dependencies]
anyhow = "1.0"
chrono = { version = "0.4", features = ["clock"] }
clap = { version = "4.5", features = ["derive"] }
rusqlite = { version = "0.32", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "2.0"

[profile.release]
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

> Nota: per questa fase iniziale ho impostato il progetto come **CLI desktop-oriented di bootstrap**, così puoi sviluppare e validare logica, DB e UX testuale prima del passaggio al PicoCalc reale.

---

## 2. `.gitignore`

```gitignore
/target
Cargo.lock
*.db
*.db-shm
*.db-wal
.DS_Store
Thumbs.db
.idea/
.vscode/
```

Se vuoi pubblicarlo come binario applicativo e non come libreria, puoi anche decidere di **tenere** `Cargo.lock`. In quel caso rimuovi la riga `Cargo.lock`.

---

## 3. `README.md`

```md
# sw_galaxy_map_pico

`sw_galaxy_map_pico` is a Pico-oriented offline navicomputer project for Star Wars galaxy routing.

This project is **not** a direct port of `sw_galaxy_map_core`.
It is a dedicated implementation designed around the constraints and UX of a small device such as PicoCalc.

## Goals

- offline galaxy database
- local route calculation
- planet search by canonical name or alias
- local route history
- small-device-friendly interface

## Project status

Early bootstrap phase.

## Planned modules

- `nav`: route, distance and ETA calculation
- `db`: SQLite access for planets, aliases and route history
- `ui`: textual navigation shell and screens
- `app`: application state and orchestration

## Initial scope

- open local SQLite databases
- search planets
- select origin and destination
- calculate basic route data
- save route history
- show recent routes

## Out of scope for v0.1.0

- graphical galaxy map
- advanced filters
- import/export features
- hardware-specific PicoCalc integration
```

---

## 4. `ROADMAP.md`

```md
# ROADMAP

## v0.1.0

- [ ] project bootstrap
- [ ] database bootstrap
- [ ] schema creation for history database
- [ ] planet search by canonical name
- [ ] alias resolution
- [ ] select origin/destination
- [ ] distance calculation
- [ ] ETA calculation
- [ ] route history save/load
- [ ] textual menu shell

## v0.2.0

- [ ] improved route engine
- [ ] favorites support
- [ ] better search UX
- [ ] route detail screen
- [ ] desktop validation against reference project

## v0.3.0

- [ ] PicoCalc integration prototype
- [ ] storage path abstraction for target device
- [ ] small-screen rendering strategy
- [ ] keyboard/input adapter
```

---

## 5. `src/main.rs`

```rust
//! Application entry point for `sw_galaxy_map_pico`.
//!
//! This binary currently provides the initial bootstrap shell used to validate
//! the project architecture, database access, and navigation workflow before
//! PicoCalc-specific integration.

mod app;
mod config;
mod db;
#[allow(dead_code)]
mod error;
#[allow(dead_code)]
mod nav;
mod ui;

use :Result;
use clap::Parser;
use config::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut app = app::App::bootstrap(cli)?;
    app.run()?;
    Ok(())
}
```

---

## 6. `src/config.rs`

```rust
//! CLI configuration and startup options.

use clap::Parser;

/// Startup configuration for the bootstrap application.
#[derive(Debug, Parser)]
#[command(
    name = "sw_galaxy_map_pico",
    version,
    about = "Bootstrap navicomputer for Star Wars galaxy routing"
)]
pub struct Cli {
    /// Path to the readonly galaxy database.
    #[arg(long, default_value = "assets/db/galaxy.db")]
    pub galaxy_db: String,

    /// Path to the writable history database.
    #[arg(long, default_value = "assets/db/history.db")]
    pub history_db: String,
}
```

---

## 7. `src/error.rs`

```rust
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
```

---

## 8. `src/app.rs`

```rust
//! Application state and orchestration.

use crate::config::Cli;
use crate::db::Database;
use crate::ui;
use anyhow::Result;

/// Central application object.
pub struct App {
    /// Database facade used by the application.
    db: Database,
}

impl App {
    /// Bootstraps the application and validates the configured databases.
    pub fn bootstrap(cli: Cli) -> Result<Self> {
        let db = Database::new(&cli.galaxy_db, &cli.history_db)?;
        Ok(Self { db })
    }

    /// Runs the initial textual shell.
    pub fn run(&mut self) -> Result<()> {
        ui::show_banner();
        ui::show_main_menu();

        let counts = self.db.get_database_counts()?;
        println!("\nDatabase status:");
        println!("  planets        : {}", counts.planets);
        println!("  aliases        : {}", counts.aliases);
        println!("  history entries: {}", counts.history_entries);

        Ok(())
    }
}
```

---

## 9. `src/nav/mod.rs`

```rust
//! Navigation engine modules.

pub mod distance;
pub mod eta;
pub mod models;
pub mod route;
```

### `src/nav/models.rs`

```rust
//! Core navigation models.

/// Unique identifier of a planet.
pub type PlanetId = i64;

/// Basic planet information required by the route engine.
#[derive(Debug, Clone)]
pub struct Planet {
    /// Planet identifier.
    pub id: PlanetId,
    /// Canonical display name.
    pub name: String,
    /// X coordinate in the project reference system.
    pub x: f64,
    /// Y coordinate in the project reference system.
    pub y: f64,
    /// Z coordinate in the project reference system.
    pub z: f64,
}

/// Route request between two planets.
#[derive(Debug, Clone)]
pub struct RouteRequest {
    /// Origin planet.
    pub from: Planet,
    /// Destination planet.
    pub to: Planet,
    /// Travel speed in arbitrary project units.
    pub speed: f64,
}

/// Summary returned by the route engine.
#[derive(Debug, Clone)]
pub struct RouteSummary {
    /// Distance between origin and destination.
    pub distance: f64,
    /// Estimated travel time in minutes.
    pub eta_minutes: u64,
}
```

### `src/nav/distance.rs`

```rust
//! Distance calculation helpers.

use super::models::Planet;

/// Computes the Euclidean distance between two planets.
pub fn euclidean_distance(from: &Planet, to: &Planet) -> f64 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;

    (dx * dx + dy * dy + dz * dz).sqrt()
}
```

### `src/nav/eta.rs`

```rust
//! ETA calculation helpers.

/// Computes the travel time in minutes for a given distance and speed.
pub fn estimate_eta_minutes(distance: f64, speed: f64) -> u64 {
    if speed <= 0.0 {
        return 0;
    }

    ((distance / speed) * 60.0).round() as u64
}
```

### `src/nav/route.rs`

```rust
//! Basic route calculation orchestration.

use super::distance::euclidean_distance;
use super::eta::estimate_eta_minutes;
use super::models::{RouteRequest, RouteSummary};

/// Calculates a basic direct route summary.
pub fn calculate_basic_route(request: &RouteRequest) -> RouteSummary {
    let distance = euclidean_distance(&request.from, &request.to);
    let eta_minutes = estimate_eta_minutes(distance, request.speed);

    RouteSummary {
        distance,
        eta_minutes,
    }
}
```

---

## 10. `src/db/mod.rs`

```rust
//! Database facade and modules.

pub mod aliases;
pub mod connection;
pub mod history;
pub mod planets;
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
    connections: DatabaseConnections,
}

impl Database {
    /// Creates a new database facade and initializes writable schema.
    pub fn new(galaxy_db_path: &str, history_db_path: &str) -> Result<Self> {
        let connections = DatabaseConnections::open(galaxy_db_path, history_db_path)?;
        schema::initialize_history_schema(&connections.history)?;
        Ok(Self { connections })
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
```

### `src/db/connection.rs`

```rust
//! SQLite connection helpers.

use anyhow::Result;
use rusqlite::Connection;

/// Holds the two SQLite connections used by the application.
pub struct DatabaseConnections {
    /// Readonly or read-mostly galaxy catalog database.
    pub galaxy: Connection,
    /// Writable history database.
    pub history: Connection,
}

impl DatabaseConnections {
    /// Opens the configured SQLite databases.
    pub fn open(galaxy_db_path: &str, history_db_path: &str) -> Result<Self> {
        let galaxy = Connection::open(galaxy_db_path)?;
        let history = Connection::open(history_db_path)?;

        Ok(Self { galaxy, history })
    }
}
```

### `src/db/schema.rs`

```rust
//! SQLite schema helpers.

use anyhow::Result;
use rusqlite::Connection;

/// Initializes the writable history schema if it does not already exist.
pub fn initialize_history_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS route_history (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at      TEXT NOT NULL,
            from_planet_id  INTEGER NOT NULL,
            to_planet_id    INTEGER NOT NULL,
            distance        REAL NOT NULL,
            eta_minutes     INTEGER NOT NULL
        );
        "#,
    )?;

    Ok(())
}
```

### `src/db/planets.rs`

```rust
//! Planet catalog queries.

use anyhow::Result;
use rusqlite::Connection;

/// Returns the total number of planets in the galaxy database.
pub fn count_planets(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM planets", [], |row| row.get(0))?;
    Ok(count)
}
```

### `src/db/aliases.rs`

```rust
//! Planet alias queries.

use anyhow::Result;
use rusqlite::Connection;

/// Returns the total number of aliases in the galaxy database.
pub fn count_aliases(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM planet_aliases", [], |row| row.get(0))?;
    Ok(count)
}
```

### `src/db/history.rs`

```rust
//! Route history queries.

use anyhow::Result;
use rusqlite::Connection;

/// Returns the total number of history entries in the writable database.
pub fn count_history_entries(conn: &Connection) -> Result<i64> {
    let count = conn.query_row("SELECT COUNT(*) FROM route_history", [], |row| row.get(0))?;
    Ok(count)
}
```

---

## 11. `src/ui/mod.rs`

```rust
//! Textual user interface modules.

#[allow(dead_code)]

pub mod menu;
pub mod screens;

pub use menu::show_main_menu;
pub use screens::show_banner;
```

### `src/ui/menu.rs`

```rust
//! Main textual menu rendering.

/// Renders the main menu.
pub fn ;
println!("1. Search planet");
println!("2. Calculate route");
println!("3. Recent routes");
println!("4. Favorites");
println!("5. Database info");
println!("6. Settings");
println!("0. Exit");
}
```

### `src/ui/screens.rs`

```rust
//! Textual screen helpers.

/// Renders the application banner.
pub fn show_banner() {
    println!("====================================");
    println!("      SW Galaxy Map Pico");
    println!("      Offline Navicomputer");
    println!("====================================");
}
```

### `src/ui/input.rs`

```rust
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
```

---

## 12. `assets/db/README.md`

```md
# Database assets

Place the local SQLite databases used by `sw_galaxy_map_pico` in this directory.

Expected files:

- `galaxy.db` → readonly or read-mostly catalog database
- `history.db` → writable route history database
```

---

## 13. Note architetturali iniziali

### Perché partire così

Questa base ti consente di lavorare subito su tre cose importanti:

1. **bootstrap del progetto**
2. **separazione dei moduli**
3. **validazione del doppio database**

### Perché non ho ancora inserito:

* import/export
* ricerca completa per alias/nome
* route engine avanzato
* collegamento al progetto desktop
* hardware PicoCalc vero

Perché in questa fase l’obiettivo giusto è avere una base che:

* compili
* apra i DB
* abbia i moduli già separati bene
* sia pronta per crescere senza rifattorizzazioni pesanti immediate

---

## 14. Ordine operativo che ti consiglio

### Step 1

Creare repo e file base.

### Step 2

Far compilare lo scheletro.

### Step 3

Aggiungere le query reali per:

* ricerca pianeta
* ricerca alias
* recupero pianeta by id

### Step 4

Sostituire il route engine basilare con le tue formule reali.

### Step 5

Aggiungere `save_route()` e `recent_routes()` in `db/history.rs`.

### Step 6

Solo dopo, progettare la UI Pico-oriented vera.

---

## 15. Prima evoluzione che ti suggerisco subito dopo lo scheletro

La prossima iterazione ideale sarebbe:

* aggiungere `PlanetMatch`
* implementare `search_planets_by_name()`
* implementare `search_planets_by_alias()`
* creare una schermata `Search results`
* definire `RouteHistoryEntry`
* aggiungere il primo comando reale `--db-info`

---

## 16. Direzione successiva

Quando vuoi, il passo dopo questo scheletro può essere uno di questi due:

1. **portare dentro il database layer reale**, con query vere su `planets`, `planet_aliases` e `planet_search`
2. **strutturare meglio il motore `nav`**, definendo già i tipi compatibili con la tua logica attuale

Per come hai impostato il progetto, io andrei prima con il **database layer reale**.
