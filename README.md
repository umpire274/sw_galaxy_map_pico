# SW Galaxy Map for Pico

[![GitHub release](https://img.shields.io/github/v/release/umpire274/sw_galaxy_map_pico?include_prereleases)](https://github.com/umpire274/sw_galaxy_map_pico/releases)
[![Rust CI](https://img.shields.io/github/actions/workflow/status/umpire274/sw_galaxy_map_pico/rust.yml?branch=main\&label=build)](https://github.com/umpire274/sw_galaxy_map_pico/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/sw_galaxy_map_pico)](https://crates.io/crates/sw_galaxy_map_pico)

`sw_galaxy_map_pico` is a Pico-oriented offline navicomputer for Star Wars galaxy routing.

This project is **not a direct port** of `sw_galaxy_map_core`, but a dedicated implementation designed for constrained environments and small displays (e.g. PicoCalc).

---

## ✨ Features

* Offline galaxy database (SQLite)
* Planet search (canonical + aliases)
* Interactive origin/destination selection
* Route calculation with:

    * distance (parsec)
    * ETA (based on speed profile)
* Obstacle detection along route
* Automatic detour generation
* **Multi-waypoint iterative routing engine**

---

## 🧠 Advanced routing (v0.9.x)

The routing engine now includes:

### 🔹 Iterative multi-waypoint routing

* multi-step collision resolution
* dynamic waypoint insertion
* path recomputation and validation

### 🔹 Advanced scoring system

* turn penalty
* proximity penalty
* offset penalty
* balanced detour selection

### 🔹 Route quality metrics

* waypoint count
* detour overhead
* max / total penalties

---

## 🔍 Full explain system

Each route provides a full explain including:

* direct vs final route
* collision analysis
* detour selection
* candidate evaluation per iteration
* routing iterations breakdown
* final path

### 💾 Persistent explain

* explain is saved in the database (`route_explain_json`)
* existing routes are automatically updated if missing explain
* explain can be replayed from **Recent routes**

---

## 🗄️ Database & diagnostics

Menu **Database status** provides:

* database path and size
* meta information (app version, install time, etc.)
* dataset counts (robust to schema differences)
* schema inspection
* FTS diagnostics

Designed to work even with partial or evolving schemas.

---

## 🧱 Architecture

* `nav` → routing engine (distance, ETA, detours, scoring)
* `db` → SQLite access and persistence
* `ui` → textual interface
* `app` → orchestration layer

---

## 📊 Project status

Active development.

Current milestone: **v0.9.x — explainable routing engine + persistence**

The project has evolved into:

> 🔹 an explainable routing engine
> 🔹 with persistent history
> 🔹 and deterministic behavior

---

## 🚧 Next steps

* explain optimization for small displays
* export (JSON / CLI)
* favorites and route management
* further scoring refinement

---

## ❌ Out of scope (short term)

* graphical galaxy map
* advanced filtering
* full PicoCalc hardware integration (early stage)

---

## 📌 Notes

This project prioritizes:

* clarity
* debuggability
* deterministic routing

The goal is to progressively align with the desktop `sw_galaxy_map` core while remaining optimized for small devices.
