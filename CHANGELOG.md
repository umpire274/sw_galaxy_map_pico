# Changelog

All notable changes to this project will be documented in this file.

---

## [0.2.0] - 2026-03-23

### ✨ New — ArcGIS integration (bootstrap)

- Introduced initial ArcGIS integration for remote planet data retrieval
- Added `grab-planets` command to download planet dataset from ArcGIS source
- Implemented paginated download using ArcGIS `resultOffset` / `resultRecordCount`

### 🌐 Networking

- Added `reqwest` (blocking client) for HTTP communication
- Implemented source-specific connectivity check:
    - validates reachability of ArcGIS endpoint before execution
    - aborts command if remote service is not reachable

### 🧩 Provisioning layer

- Introduced `provision::arcgis` module:
    - layer metadata retrieval (`maxRecordCount`)
    - feature page download
    - full dataset pagination
- Added dynamic endpoint construction using a single `LAYER_URL`

### 📥 Data handling (foundation)

- Introduced `ArcGisPlanetFeature` structure for raw feature handling
- Implemented JSON parsing helpers (`get_string`, `get_f64`, `get_i64`)
- Added initial validation of feature payloads

### 🔍 Dry-run diagnostics

- Added `--dry-run` mode to inspect remote dataset without modifying database
- Displays:
    - total downloaded features
    - validated feature payloads
    - skipped planet count
    - breakdown of skipped reasons:
        - missing planet name
        - missing X coordinate
        - missing Y coordinate

- Skipped planets follow the same logic as the main project:
    - records missing required fields are excluded from `planets`
    - these records are intended for `planets_unknown` in future versions

### 🧠 CLI

- Extended CLI with subcommands using `clap`:
    - `grab-planets`
    - optional `--dry-run` mode

### 🏗️ Architecture

- Added new modules:
    - `commands` → CLI command handlers
    - `net` → connectivity checks
    - `provision` → remote data acquisition
    - `utils` → shared JSON helpers
- Maintained strict separation between:
    - remote data acquisition
    - parsing utilities
    - CLI orchestration

### ⚠️ Current limitations

- Planet data is downloaded but not yet persisted into database
- No normalization or mapping to internal structures yet
- No alias or search table integration
- No differential update logic (full fetch only)

### 🚧 Next steps

- Introduce `RemotePlanetRecord`
- Implement mapping from ArcGIS features
- Add upsert logic for:
    - `planets`
    - `planets_unknown`
- Introduce alias extraction and persistence
- Build search index (`planet_search`)

---

## [0.1.0] - 2026-03-23

### ✨ Initial Bootstrap Release

First working version of `sw_galaxy_map_pico`.

This release establishes the foundational architecture of the Pico-oriented
Star Wars galaxy navicomputer project.

### 🧱 Project Structure

- Introduced modular architecture:
    - `app` → application orchestration
    - `nav` → route, distance and ETA engine (initial version)
    - `db` → SQLite database access layer
    - `ui` → textual interface shell
    - `config` → CLI configuration
    - `error` → project error definitions

### 🗄️ Database Layer

- Added dual-database design:
    - `galaxy.db` → readonly galaxy catalog
    - `history.db` → writable route history
- Implemented:
    - database connection management
    - initial history schema (`route_history`)
    - basic database validation (counts)

### 🧭 Navigation Engine (Initial)

- Introduced core navigation models:
    - `Planet`
    - `RouteRequest`
    - `RouteSummary`
- Implemented:
    - Euclidean distance calculation
    - basic ETA estimation
    - simple direct route calculation

### 🖥️ UI (Bootstrap)

- Added initial textual interface:
    - banner
    - main menu
- Prepared input abstraction layer for future Pico integration

### ⚙️ CLI

- Added CLI configuration via `clap`
- Configurable database paths:
    - `--galaxy-db`
    - `--history-db`

### 🧩 Architecture Decisions

- This project is a **Pico-oriented rewrite**, not a port of `sw_galaxy_map_core`
- Separation between:
    - navigation logic
    - database access
    - UI layer
- SQLite retained as storage backend, but isolated

### 🚧 Known Limitations

- No real planet search yet
- No alias resolution yet
- No real route engine (only direct distance)
- No route history persistence logic yet
- No PicoCalc hardware integration yet

---

## Upcoming

### v0.2.0 (planned)

- Planet search (name + alias)
- Real route engine integration
- Route history save/load
- Improved UI flow
- Desktop validation vs core project