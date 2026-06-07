# Rust + Macroquad Code Review Guide

## Overall Goal
The codebase should be easy to extend, safe to refactor, and fast enough for real-time gameplay. In a Macroquad project, the biggest risks often come from god files, mixed gameplay/rendering logic, and state that leaks everywhere.

---

## 1. Project Structure
Review whether the code is split into sensible modules.

### Good signs
- `main.rs` is small and mostly bootstraps the game.
- There are separate modules for game state, rendering, input, world/entities, UI, and assets/content loading.
- Shared data types are centralized in a clear location.

### Warning signs
- One massive file doing everything.
- Gameplay, drawing, and input are mixed in the same update block.
- No boundary between runtime logic and authored game data.

---

## 2. Game Loop Discipline
Macroquad encourages a simple loop, but simple can still become spaghetti.

### Review for
- consistent update → render flow
- input handled once per frame in a predictable place
- delta time used correctly for movement/timers
- no expensive loading/parsing inside the main loop

### Watch out for
- frame-dependent movement
- ad hoc timers scattered everywhere
- gameplay state changing inside draw code

---

## 3. State Management
Game state should be explicit, not haunted.

### Good signs
- top-level state structs like `Game`, `World`, `Player`, and `UiState`
- enums for major modes like `Menu`, `Gameplay`, `Pause`
- state transitions handled in one place

### Warning signs
- too many mutable globals
- deeply nested mutable borrowing just to update simple systems
- boolean flag explosions like `is_paused`, `is_dead`, `in_menu`, `dialog_open`, `shop_open`

Prefer enums and typed state over piles of flags.

---

## 4. Rust Code Quality
Rust should be used as a sharp tool, not decorative armor.

### Review for
- meaningful types instead of loose tuples
- `Option` and `Result` handled cleanly
- helper methods on structs instead of repeated logic
- borrowing/cloning choices that make sense

### Watch out for
- unnecessary `.clone()` everywhere
- `unwrap()` in runtime gameplay paths
- giant functions caused by borrow-checker avoidance

A little cloning is fine in games. Panic confetti in normal gameplay is not.

---

## 5. Rendering Separation
Rendering should reflect state, not secretly control it.

### Good signs
- draw functions take read-only state where possible
- world rendering and UI rendering are separated
- camera logic is isolated and readable

### Warning signs
- spawning/despawning entities during draw
- UI buttons directly mutating half the game world
- duplicated draw logic for similar entity types

---

## 6. Data-Driven Design
If the game uses JSON or authored content, review how cleanly code consumes it.

### Review for
- content structs separate from runtime structs
- validation or graceful fallback for bad data
- stable IDs for areas, items, NPCs, recipes, etc.

### Warning signs
- magic strings scattered through gameplay code
- runtime crashes from missing content keys
- authored data tightly coupled to rendering logic

---

## 7. Performance Awareness
Macroquad games usually do not need extreme optimization, but obvious waste should be removed.

### Review for
- no allocations inside hot update loops
- no repeated file reads or parsing during gameplay
- no quadratic scans over entities when avoidable
- cached or structured asset lookups

### Warning signs
- file reads during gameplay
- rebuilding expensive data every frame
- quadratic loops over entities without reason

Focus on clarity first, then optimize hotspots you can actually name.

---

## 8. Maintainability
Ask whether the next enemy, map, crop, or UI panel can be added without surgery.

### Good signs
- reusable patterns for entities and UI
- small functions with clear responsibilities
- comments explain why, not what

### Warning signs
- copy-pasted logic per feature
- hidden assumptions
- fragile code where changing one mechanic breaks unrelated systems

---

## Rating Scale
Use this for each category:
- 5 — Strong: clean, scalable, very little risk
- 4 — Good: solid overall, minor issues only
- 3 — Acceptable: works, but technical debt is forming
- 2 — Weak: noticeable design problems, likely slowing development
- 1 — Poor: major structural issues, high maintenance risk

---

## Fast Review Format
Example quick review:

- Project Structure: 4/5
- Game Loop Discipline: 3/5
- State Management: 2/5
- Rust Code Quality: 4/5
- Rendering Separation: 3/5
- Data-Driven Design: 4/5
- Performance Awareness: 3/5
- Maintainability: 2/5

**Overall:** 25/40

### Biggest strengths
- Clear module separation
- Good use of enums
- Content loading is reasonably structured

### Biggest risks
- Too many boolean flags
- UI and gameplay logic are coupled
- Some files/functions are too large

### Top priority fixes
- Refactor state handling into clearer enums/structs
- Split update and draw responsibilities more cleanly
- Reduce duplication in entity/UI logic

---

## Suggested Score Summary
- 34–40: Strong foundation
- 26–33: Good, but cleanup will pay off
- 18–25: Functional prototype, structural debt is building
- 10–17: Major refactor territory
- 8–9: Held together by optimism and keyboard pressure
