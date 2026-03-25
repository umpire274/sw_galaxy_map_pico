# Changelog

All notable changes to this project will be documented in this file.

---

## [0.9.1] - 2026-03-25

### ✨ Improvements

- Introduced **Database status screen** (menu 5):
    - database path and size
    - meta information (app version, install time, last start)
    - dataset counts (schema-aware, safe on missing tables/columns)
    - schema inspection (views/tables presence)
    - FTS diagnostics

### 🧠 Robustness

- Database diagnostics now adapt to schema differences:
    - no crashes on missing tables (e.g. `planet_search`)
    - no crashes on missing columns (e.g. `deleted`)
    - graceful fallback with `n/a` values

### 🔧 Refactor

- Removed legacy database counting logic (`DatabaseCounts`)
- Centralized DB introspection in `db::status`
- Cleaned unused functions and fields across DB layer
- Improved separation between:
    - data collection (DB layer)
    - rendering (UI layer)

### 🧹 Cleanup

- Resolved all Clippy warnings (`-D warnings`)
- Removed dead code and unused helpers

### 📚 Documentation

- Updated README to reflect v0.9.x capabilities
- Updated ROADMAP with v0.9.0 and v0.9.1 milestones

---

## [0.9.0] - 2026-03-24

### 🚀 Features

- Introduced advanced route scoring:
    - Turn penalty
    - Proximity penalty
    - Offset penalty
- Added full route quality metrics:
    - Waypoint count
    - Detour overhead
    - Max / total penalties (turn, proximity, offset)
- Implemented iterative routing explain model:
    - Per-iteration breakdown
    - Candidate evaluation details
    - Selected detour tracking

### 💾 Persistence

- Added `route_explain_json` to `routes` table
- Persist full explain snapshot for each computed route
- Automatic backfill:
    - If a route already exists but has no explain JSON, it is updated
- Introduced route fingerprint-based deduplication

### 🔍 UX Improvements

- Added full explain visualization in **Recent routes**
- Output parity between:
    - Live route calculation
    - Saved route inspection
- Removed duplicated "Final path" section when explain is available
- Improved readability and structure of route output

### 🧱 Internal

- Refactored `save_route(...)` to use `SaveRouteEndpoints`
- Introduced helper logic for missing explain detection
- Cleaned Clippy warnings (`-D warnings` compliant)
- Improved separation between:
    - navigation logic
    - persistence layer
    - UI rendering

### 🐛 Fixes

- Fixed missing explain update for already existing routes
- Fixed handling of empty / NULL JSON fields

---

## [0.8.0] - 2026-03-24

### 🚀 Added

* Route persistence in history database
* Storage of computed routes as first-class entities
* Storage of final path (multi-waypoint sequence)
* Recent routes listing (interactive)
* Route detail view with:

    * origin/destination
    * distances (direct and final)
    * ETA (direct and final)
    * safety flags
    * iteration count
    * full final path reconstruction

### 🔧 Improved

* Integration of routing engine with persistence layer
* Clean separation between routing, database, and UI
* User flow:

    * calculate → save → browse → inspect
* Improved usability of navigation menu (Recent routes now functional)

### 🧠 Internal

* Added `routes` table
* Added `route_points` table
* Introduced route persistence transaction logic
* Added models:

    * `RecentRouteRow`
    * `SavedRouteDetails`
    * `SavedRoutePoint`
* Introduced database helpers:

    * `save_route`
    * `list_recent_routes`
    * `get_route_details`

### 📚 Documentation

* Updated README to include route persistence capabilities
* Updated ROADMAP with v0.8.0 milestone

### 💡 Notes

This release completes the transition from a pure routing engine to a **persistent navigation system**.

Routes are now:

* computed
* stored
* retrieved
* inspected

This enables future features such as favorites, replay, and advanced analysis.

---

## [0.7.0] - 2026-03-24

### 🚀 Added

* True multi-waypoint routing
* Iterative routing loop with multiple collision handling
* Dynamic insertion of multiple waypoints along the path
* Final path generation as explicit sequence of points
* Total iteration tracking in route summary
* Final collision detection after routing

### 🔧 Improved

* Routing engine upgraded from single-detour to iterative multi-step routing
* Collision detection extended to full path (not only direct route)
* Candidate evaluation grouped per iteration
* Routing explain restructured and improved readability
* Output clarity:

    * direct vs final route
    * total iterations
    * final collision (explicit)
    * last selected detour
    * final path visualization

### 🧠 Internal

* Refactored routing flow to operate on dynamic path segments
* Improved separation between routing engine, database and UI layers
* Enhanced RouteSummary model with:

    * final_path
    * total_iterations
    * final_collision
* Prepared groundwork for future parity with desktop core

### 📚 Documentation

* Updated README to reflect multi-waypoint routing engine
* Updated ROADMAP with completed v0.7.0 milestone

### 💡 Notes

This release marks a major milestone:

The routing engine transitions from a **single-detour system** to a **true iterative multi-waypoint router**, capable of
handling multiple obstacles along the route and dynamically refining the path until a safe solution is found.

---

## [0.6.2] - 2026-03-23

### ✨ New — Dynamic obstacle loading

- Introduced dynamic obstacle loading during iterative routing
- Obstacles are now reloaded at each iteration based on the current path bounding box
- Ensures that newly introduced path segments are validated against all relevant obstacles

### 🧭 Routing engine improvements

- Routing engine now operates on a dynamically updated obstacle set
- Added bounding box computation for full route path
- Improved correctness of collision detection on expanded paths

### 🧩 Architecture

- Replaced static obstacle list with loader-based approach:
    - `calculate_iterative_route` now accepts a closure for obstacle retrieval
- Introduced obstacle merging to avoid duplicates across iterations
- Improved separation of responsibilities:
    - app layer → provides loader
    - routing layer → controls when to load obstacles

### 🔍 Collision handling

- Collision detection now reflects the actual current path, not just the initial direct segment
- Final route validation uses the full dynamically loaded obstacle set

### 📐 Internal utilities

- Added `path_bbox` helper for computing bounding boxes of multi-point paths
- Added `merge_obstacles` helper to deduplicate obstacle lists

### ✅ Validation milestone

- Confirmed that existing routes (e.g. Coruscant → Naboo) remain stable
- Verified that routing results are unchanged when no new obstacles are introduced
- Validated that the routing engine is now ready for multi-waypoint scenarios

### ⚠️ Current limitations

- Dynamic loading does not yet adjust search radius beyond bounding box expansion
- Candidate grouping is still global (not per iteration)
- No persistence of loaded obstacle sets or route snapshots

### 🚧 Next steps

- Group detour candidates by iteration in output
- Improve iteration explain with per-step candidate breakdown
- Support multiple waypoint insertions in complex routes
- Expand obstacle loading strategy beyond simple bounding box

---

## [0.6.1] - 2026-03-23

### ✨ New — Routing iteration explain

- Added iteration-level explain output to the Pico route engine
- Each routing step now records:
    - iteration index
    - colliding segment index
    - obstacle hit on that segment
    - evaluated candidates
    - selected candidate

### 🧭 Route explain improvements

- Route output now distinguishes clearly between:
    - direct route status
    - final route status
- Added explicit `Direct collision` section
- Added `Routing iterations` section for step-by-step routing introspection

### 🔍 Iteration tracking

- Introduced `RouteIterationExplain`
- Route summary now stores explain history for each routing pass
- Iterative router now records:
    - colliding obstacle
    - candidate set
    - chosen waypoint for the iteration

### 🧩 Output clarity

- Improved route result presentation for iterative routing
- Final route metrics are now easier to compare against the original direct route
- Explain output is now significantly closer to the desktop core `route explain` workflow

### ✅ Validation milestone

- Confirmed routing iteration output on `Coruscant -> Naboo`
- Iteration explain correctly reports:
    - segment `0`
    - obstacle `Darepp [2633]`
    - selected detour side and offset
    - candidate count

### ⚠️ Current limitations

- Only one practical reroute iteration is exercised in the current scenario
- Candidate output is aggregated globally, not grouped visually by iteration
- No recursive multi-waypoint explain tree yet
- No persisted iteration history in database yet

### 🚧 Next steps

- Group detour candidates by iteration in output
- Support multiple reroute iterations across expanded paths
- Add per-iteration path snapshot
- Improve parity with desktop core explain formatting
- Persist route explain data for later inspection

---

## [0.6.0] - 2026-03-23

### ✨ New — First iterative router

- Introduced the first iterative routing workflow for `sw_galaxy_map_pico`
- Routes are now treated as multi-segment paths instead of a single direct line
- Added support for inserting detour waypoints directly into the route path

### 🧭 Route path model

- Introduced path-based routing using ordered 2D points
- Added segment abstraction for route analysis
- Added waypoint insertion after a colliding segment
- Added full path segment rebuilding for rerouted paths

### ⚠️ Collision analysis

- Added support for detecting the first collision across an entire path
- Preserved direct-route collision analysis separately from final-route validation
- Route output now clearly distinguishes:
    - direct route status
    - final route status

### 🔄 Applied reroute

- Selected detour waypoint is now applied to the final path
- Final route metrics are now computed from the rerouted path:
    - final distance
    - final ETA
- Added full safety verification for the rerouted path

### 🔍 Explain improvements

- Added explicit direct collision section
- Added final-route status reporting
- Added structured explain output for:
    - collision details
    - selected detour
    - full detour candidate list

### 📐 Routing behavior

- Preserved direct-route metrics for comparison
- Recomputed final route from actual inserted waypoint
- Candidate evaluation now supports iterative path insertion workflow

### ✅ Validation milestone

- Direct route and first-hit obstacle detection remain aligned with desktop core
- Pico now produces:
    - unsafe direct route
    - safe final rerouted path
    - effective final distance and ETA

### ⚠️ Current limitations

- Only one inserted waypoint is effectively used in current practical flow
- No recursive reroute chain for additional collisions beyond the first applied path adjustment
- No persistence of computed paths or detours yet
- No iteration-by-iteration explain history yet
- Obstacle loading still depends on the initial route bounding box

### 🚧 Next steps

- Add iterative explain by routing step
- Support multiple detour waypoints in real route construction
- Recompute obstacle search bounds dynamically after path expansion
- Persist computed detours and route paths
- Continue parity work with desktop core route explain model

---

## [0.5.5] - 2026-03-23

### ✨ New — First applied reroute

- Implemented the first applied reroute workflow in the Pico route engine
- Direct routes are now converted into an effective detoured route when a valid waypoint is found
- The final route now reports:
    - final distance
    - final ETA
    - detour safety status

### 🧭 Routing engine

- Added support for applying a selected detour waypoint to the final route
- Route engine now distinguishes between:
    - direct route metrics
    - final effective route metrics
- Added detour safety verification across both route legs:
    - `from -> waypoint`
    - `waypoint -> to`

### ⚠️ Collision & detour explain

- Added structured collision explain output:
    - obstacle name and ID
    - obstacle center
    - obstacle radius
    - closest distance
    - required clearance
    - violation amount
    - closest point
    - segment factor `t`
    - collision penalty

- Added structured detour explain output:
    - chosen side
    - offset used
    - total score
    - base distance
    - turn penalty
    - back penalty
    - proximity penalty

### 🔍 Candidate analysis

- Added full candidate evaluation output for each generated detour
- Each candidate now reports:
    - side
    - offset
    - validity
    - score
    - rejection reason (if invalid)
    - score breakdown (if valid)

### 📐 Scoring improvements

- Introduced stronger detour scoring based on:
    - base route distance
    - turn penalty
    - backtracking penalty
    - residual proximity penalty

- Added soft proximity comfort zone to discourage detours that remain too close to obstacles
- Candidate selection is now based on score quality, not just shortest valid path

### ✅ Validation milestone

- Achieved parity with desktop core on:
    - direct route distance
    - ETA model
    - first obstacle identification
    - first collision metrics
    - obstacle-centered detour generation

### ⚠️ Current limitations

- Only one detour waypoint is supported
- No recursive reroute if the detoured path hits another obstacle
- No multi-waypoint route building yet
- No persistence of computed waypoints or detours yet
- Scoring is closer to desktop logic but not yet fully identical

### 🚧 Next steps

- Add second collision check over the rerouted path
- Support multiple detour waypoints
- Introduce iterative route recomputation
- Improve parity with desktop scoring and explanation model
- Persist computed detours into routing tables

---

## [0.5.1] - 2026-03-23

### ✨ New — Obstacle detection (Phase 3)

- Introduced obstacle detection along direct routes using point-to-segment distance
- Implemented closest approach calculation between route segment and planetary obstacles
- Added safety clearance model:
    - `required_clearance = obstacle_radius + route_clearance`
- Introduced violation detection when route intersects obstacle safety zone

### 🧩 Navigation model extensions

- Added new navigation data structures:
    - `Obstacle`
    - `ObstacleCheck`
    - `Point2`
- Extended `RouteSummary` to include closest obstacle violation information

### 📐 Geometry layer

- Implemented reusable geometry utilities:
    - Euclidean distance (2D)
    - Closest point on segment
    - Segment interpolation factor `t ∈ [0,1]`
- Ensured numerical stability for degenerate segments

### ⚠️ Route safety feedback

- Route result now includes:
    - closest obstacle name and ID
    - closest distance to route
    - required minimum clearance
    - closest point on segment
    - segment factor `t`
    - violation flag

### 🔍 Debug & validation (internal)

- Added diagnostic tooling to inspect obstacle ranking and collision behavior
- Identified discrepancy with desktop core obstacle selection
- Discovered presence of large number of `(x=0, y=0)` placeholder records in dataset

### 🧠 Key findings

- Not all planets should be treated as routing obstacles
- Desktop core uses a filtered obstacle set (via `waypoint_planets`)
- Current Pico implementation uses full `planets` table → temporary divergence expected

### ⚠️ Known limitations

- All planets are currently treated as potential obstacles (no filtering yet)
- Obstacle radius is fixed (`2.0 pc`) and not data-driven
- No detour generation implemented yet
- No obstacle prioritization based on route traversal order (first-hit logic incomplete)
- Dataset contains placeholder coordinates `(0,0)` affecting collision results

### 🚧 Next steps

- Introduce `waypoints` and `waypoint_planets` schema
- Implement obstacle query parity with desktop core
- Port first-hit collision logic from desktop
- Implement detour generation and waypoint insertion
- Align ETA model with desktop routing engine

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