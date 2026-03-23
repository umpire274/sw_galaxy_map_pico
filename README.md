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
