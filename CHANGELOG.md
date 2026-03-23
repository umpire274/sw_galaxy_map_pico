# Changelog

All notable changes to this project will be documented in this file.

---

## [0.5.0] - 2026-03-23

### ✨ New — First route calculation workflow

- Implemented the first complete route calculation flow in the interactive UI
- Added support for:
    - origin planet selection
    - destination planet selection
    - direct route calculation
    - route result visualization

### 🧭 Interactive route selection

- Reused the existing search system to select route endpoints
- Added dedicated planet selection flow for route calculation:
    - single search result → automatic selection
    - multiple search results → numeric selection
- In route selection mode, planet details are no longer shown before selection
- In view-only search mode, single-result queries open planet details directly

### 🖥️ UI improvements

- Improved menu integration:
    - `Calculate route` is now active
- Standardized interaction behavior:
    - `ENTER` or `0` = back / exit depending on context
- Improved search navigation flow:
    - returning from planet detail restores search results
    - search selection is now faster and more context-aware

### 🧩 Application structure

- Refactored search logic into a reusable internal selection flow
- Added mapping helper from database models to navigation models:
    - `convert_to_nav_planet`

### 📏 Route calculation

- Added direct route computation between two selected planets
- Route result now includes:
    - distance in parsecs
    - ETA
    - base speed
    - hyperdrive class
    - route multiplier
    - effective speed

### ⏱️ ETA improvements

- Converted ETA formatting from raw minutes to:
    - `dd hh mm ss`
- Aligned ETA formula with desktop core assumptions using:
    - base speed
    - hyperdrive class
    - route multiplier

### ⚠️ Current limitations

- Route calculation is still direct (point-to-point)
- No obstacle avoidance or detour generation yet
- No contextual route multiplier computation from real route conditions yet
- No pathfinding through intermediate waypoints yet
- No route persistence into history yet

### 🚧 Next steps

- Replicate desktop core route logic in Pico version
- Introduce obstacle / proximity detection
- Implement detour generation and waypoint insertion
- Reproduce contextual speed / multiplier logic from regions and travel context
- Persist calculated routes into history

---

## [0.4.0] - 2026-03-23

### ✨ New — Search & interaction layer

- Implemented full planet search functionality in the interactive UI
- Added support for:
    - search by canonical name
    - search by alias (`name0`, `name1`, `name2`)
- Introduced result deduplication across name and alias matches

### 🔎 Search system

- Added `planet_aliases` table:
    - stores alternate planet names
    - supports normalized search (`alias_norm`)
- Implemented:
    - `search_planets_by_name`
    - `search_planets_by_alias`
    - unified `search_planets` function

### 🧭 Interactive navigation

- Implemented interactive search flow:
    - query input
    - result list
    - numeric selection
    - planet detail view

- Added planet detail screen:
    - region, sector, system, grid
    - coordinates (X/Y)
    - canon / legends flags
    - status

### 🖥️ UI improvements

- Main menu is now fully interactive
- Standardized navigation:
    - `ENTER` = go back
    - `0` = go back / exit
- Improved navigation flow:
    - returning from planet detail restores result list
    - returning from results restores search query

### 🧩 Architecture

- Introduced `utils::normalize` for text normalization
- Extended `db::planets` with:
    - alias insertion
    - search helpers
    - planet detail retrieval

- Improved separation of concerns:
    - UI (input/screens)
    - application logic (app)
    - persistence layer (db)
    - provisioning layer (arcgis)

### ⚠️ Current limitations

- No fuzzy matching yet
- No ranking or scoring of results
- No search index (FTS)
- Route calculation not yet integrated with search

### 🚧 Next steps

- Add fuzzy search (Levenshtein / trigram)
- Introduce FTS5 search index
- Implement route calculation from selected planets
- Add favorites and recent routes integration

---

## [0.3.0] - 2026-03-23

### ✨ New — Planet data persistence

- Introduced full data import pipeline from ArcGIS into local SQLite database
- Implemented normalization layer:
    - `RemotePlanetRecord` for valid planets
    - `SkippedPlanetRow` for invalid/skipped entries

### 🗄️ Database

- Added galaxy catalog schema:
    - `planets` table for valid records
    - `planets_unknown` table for skipped/invalid records
- Implemented upsert logic for `planets`
- Implemented synchronization logic for `planets_unknown` (replace strategy)

### 🔍 Data validation & classification

- Introduced ArcGIS-based validation rules:
    - valid planet = has `Planet`, `X`, `Y`
    - invalid planet = classified as "unknown"
- Added skipped row classification with reasons:
    - `missing_planet`
    - `missing_x`
    - `missing_y`

### 📊 Dry-run improvements

- Extended `--dry-run` output:
    - total downloaded features
    - validated payload count
    - skipped planet summary
    - detailed breakdown by reason
    - preview of first skipped rows
    - count of valid planets ready for import

### 🔄 Import pipeline

- Implemented full pipeline:
    - download → validate → classify → map → persist
- Transaction-based import for consistency
- Database schema auto-initialization

### 🧩 Provisioning layer

- Added ArcGIS mapping utilities:
    - `map_feature_to_planet`
    - `collect_skipped_planets`
    - `summarize_skipped_rows`
- Centralized validation logic:
    - `is_valid_planet`
    - `is_unknown`

### 🧱 Architecture

- Introduced `db::planets` module:
    - upsert logic
    - unknown sync
    - counters (`count_planets`, `count_unknown_planets`)
- Strengthened separation between:
    - provisioning (ArcGIS)
    - domain mapping
    - persistence layer

### ⚠️ Current limitations

- Unknown planet sync uses full replace strategy (not incremental)
- No alias extraction (`name0`, `name1`, `name2`) yet
- No search/index table (`planet_search`) yet
- No differential update (always full import)

### 🚧 Next steps

- Implement alias extraction (`planet_aliases`)
- Build search index (`planet_search`)
- Optimize unknown sync (incremental)
- Introduce differential update (hash-based)
- Add progress reporting for large imports

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