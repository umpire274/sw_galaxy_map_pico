# Changelog

All notable changes to this project will be documented in this file.

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