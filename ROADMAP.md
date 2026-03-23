# ROADMAP

## v0.1.0

* [x] project bootstrap
* [x] database bootstrap
* [x] schema creation for history database
* [x] planet search by canonical name
* [x] alias resolution
* [x] select origin/destination
* [x] distance calculation
* [x] ETA calculation
* [x] textual menu shell

---

## v0.2.0

* [x] improved route engine
* [x] better search UX
* [x] route detail screen
* [x] desktop validation against reference project

---

## v0.3.0

* [x] small-screen oriented UI structure
* [x] interactive navigation flow
* [x] improved input handling

---

## v0.4.x

* [x] search improvements
* [x] direct selection optimization (single result auto-select)
* [x] better navigation flow (ENTER = back)
* [x] improved menu UX

---

## v0.5.x

* [x] obstacle detection along route
* [x] detour candidate generation
* [x] scoring system (distance + penalties)
* [x] detour selection
* [x] collision explain output
* [x] candidate evaluation output

---

## v0.6.0

* [x] path-based routing (multi-segment)
* [x] waypoint insertion into route
* [x] iterative routing engine (first version)
* [x] final route recomputation
* [x] final route safety validation
* [x] direct vs final route distinction in output

---

## v0.6.1

* [x] routing iteration explain
* [x] iteration tracking (segment + obstacle)
* [x] selected candidate per iteration
* [x] integration of iteration history into RouteSummary
* [x] improved route explain readability

---

## v0.6.x (next)

* [ ] group candidates per iteration in UI
* [ ] per-iteration detailed explain (breakdown + decision reasoning)
* [ ] dynamic obstacle loading after path expansion
* [ ] path debug visualization (textual)

---

## v0.7.0 (planned)

* [ ] true multi-waypoint routing
* [ ] multiple collision handling across expanded path
* [ ] iterative refinement loop (core-like behavior)
* [ ] improved scoring parity with desktop core

---

## v0.8.0 (future)

* [ ] route persistence (save/load)
* [ ] route history navigation
* [ ] favorites system

---

## v1.0.0 (vision)

* [ ] stable routing engine
* [ ] consistent behavior with desktop core
* [ ] optimized performance for small devices
* [ ] production-ready Pico-oriented UX

---

## Long-term ideas

* PicoCalc hardware integration
* graphical route preview (lightweight)
* advanced filtering and search
* import/export routes
* API compatibility with desktop core
