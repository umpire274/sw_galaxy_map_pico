# SW Galaxy Map for Pico

[![GitHub release](https://img.shields.io/github/v/release/umpire274/sw_galaxy_map_pico?include_prereleases)](https://github.com/umpire274/sw_galaxy_map_pico/releases)
[![Rust CI](https://img.shields.io/github/actions/workflow/status/umpire274/sw_galaxy_map_pico/rust.yml?branch=main)](https://github.com/umpire274/sw_galaxy_map_pico/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/sw_galaxy_map_pico)](https://crates.io/crates/sw_galaxy_map_pico)

`sw_galaxy_map_pico` is a Pico-oriented offline navicomputer project for Star Wars galaxy routing.

This project is **not** a direct port of `sw_galaxy_map_core`.
It is a dedicated implementation designed around the constraints and UX of small devices such as PicoCalc.

---

## ✨ Features (current)

* offline galaxy database (SQLite)
* planet search by canonical name and aliases
* interactive selection of origin and destination
* route calculation with:

    * distance (parsec)
    * ETA (based on speed profile)
* obstacle detection along route
* automatic detour generation
* **multi-waypoint iterative routing engine**
* detailed route explain:

    * direct vs final route
    * collision analysis
    * detour selection
    * candidate evaluation (per iteration)
    * routing iterations (multi-step)
    * final path output

---

## 🧭 Routing engine

The current engine (v0.7.0) supports:

* direct route analysis
* obstacle collision detection (segment-based)
* detour candidate generation
* scoring system (distance + penalties)
* waypoint insertion into route path
* **multi-step iterative routing**
* collision detection on expanded path
* final route recomputation
* safety validation of rerouted path
* structured explain per iteration

Example output includes:

* direct route status (safe / unsafe)
* final route status (safe / unsafe)
* total iterations
* final collision (if any)
* collision explain
* last selected detour
* routing iterations (grouped)
* final path (multi-waypoint)

---

## 🧱 Architecture

* `nav`: routing engine (distance, ETA, detours, iterations)
* `db`: SQLite access (planets, aliases, obstacles)
* `ui`: textual interface and screens
* `app`: application orchestration

---

## 📊 Project status

Active development.

Current milestone: **v0.7.0 — multi-waypoint iterative routing**

The project has evolved into a **true iterative routing engine**, capable of:

* handling multiple collisions along a route
* inserting multiple waypoints dynamically
* recomputing and validating the full path
* providing a structured explain of each routing step

---

## 🚧 Next steps

* improve explain clarity and compactness for small displays
* refine scoring model (closer to desktop core)
* improve performance for large obstacle sets
* route persistence (save/load)
* enhanced debugging tools for routing

---

## ❌ Out of scope (short term)

* graphical galaxy map
* advanced filtering
* import/export features
* full hardware integration with PicoCalc (early stage)

---

## 📌 Notes

This project is designed with **clarity, debuggability, and deterministic routing behavior** as primary goals.

The routing engine is intentionally built step-by-step to mirror and eventually approach the behavior of the desktop `sw_galaxy_map` core.

The v0.7.0 milestone marks the transition from a **single-detour system** to a **true iterative multi-waypoint router**.
