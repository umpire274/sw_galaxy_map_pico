# sw_galaxy_map_pico

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
* iterative routing engine (multi-step ready)
* detailed route explain:

  * direct vs final route
  * collision analysis
  * detour selection
  * candidate evaluation
  * routing iterations

---

## 🧭 Routing engine

The current engine (v0.6.x) supports:

* direct route analysis
* obstacle collision detection
* detour candidate generation
* waypoint insertion into route path
* final route recomputation
* safety validation of rerouted path
* iteration-based routing explain

Example output includes:

* direct route status (safe / unsafe)
* final route status (safe / unsafe)
* collision explain
* selected detour
* full candidate list
* routing iterations summary

---

## 🧱 Architecture

* `nav`: routing engine (distance, ETA, detours, iterations)
* `db`: SQLite access (planets, aliases, obstacles)
* `ui`: textual interface and screens
* `app`: application orchestration

---

## 📊 Project status

Active development.

Current milestone: **v0.6.1 — iterative routing explain**

The project has transitioned from a basic route calculator to a **first iterative routing engine**.

---

## 🚧 Next steps

* multi-waypoint routing (true iterative expansion)
* dynamic obstacle loading (beyond initial bbox)
* improved explain (per-iteration detail expansion)
* route persistence
* improved UX for small displays

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
