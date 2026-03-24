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

## v0.6.2

* [x] dynamic obstacle loading based on route bounding box
* [x] integration of obstacle queries into routing flow
* [x] improved consistency between routing and database layer

---

## v0.7.0

* [x] true multi-waypoint routing
* [x] multiple collision handling across expanded path
* [x] iterative refinement loop (multi-step routing)
* [x] collision detection on updated path segments
* [x] insertion of multiple waypoints
* [x] grouped candidate evaluation per iteration
* [x] final path generation (explicit waypoint sequence)
* [x] final collision detection (post-routing validation)
* [x] total iteration tracking
* [x] improved routing explain structure

---

## v0.8.0 (next)

* [ ] explain optimization for compact displays
* [ ] scoring refinement (closer to desktop core)
* [ ] performance improvements for large datasets
* [ ] route persistence (save/load)
* [ ] route history navigation

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
