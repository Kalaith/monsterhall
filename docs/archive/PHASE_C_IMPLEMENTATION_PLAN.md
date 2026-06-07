# Phase C Implementation Plan: Chamber-First Egg Logic

## Summary
Phase C replaces the current loose `resource.eggs + hatch species` model with a real egg pipeline. Eggs should exist as records in save state, come from specific tower sources, move through the chamber workflow, and only become monster girls after chamber preparation.

## Core Outcome
After this phase:

- eggs are inventory items, not only a number
- expeditions add raw eggs with source metadata
- the opening egg uses the same pipeline as later eggs
- chamber use is built into hatching
- monster-girl creation visibly depends on chamber use

## Initial Scope

### Egg Runtime
Add starter egg fields:

- `id`
- `source_floor_id`
- `possible_species_ids`
- `selected_species_id`
- `incubation_state`
- `loyalty_imprinted`

Recommended incubation states:

- `Raw`
- `ReadyToHatch`

Phase C does not need a multi-day incubation timer yet.

## Gameplay Logic Changes

### 1. Opening Egg Rewrite
The opening chapter should stop adding a generic egg count and instead:

- create one raw opening egg
- mark it as chamber-eligible
- hatch the first slime girl from that egg

### 2. Expedition Egg Rewards
Expedition eggs should create egg records using the selected floor’s species pool:

- one record per egg gained
- source floor stored on the egg
- `possible_species_ids` copied from the floor
- eggs start as `Raw`

### 3. Chamber Workflow
Town-side girl creation should split into:

- `Hatch`

Minimum Phase C behavior:

- hatching a raw egg uses the chamber automatically
- chamber preparation sets `loyalty_imprinted = true`
- hatching consumes a ready egg and creates the monster girl

### 4. Resource Compatibility
The old `resources.eggs` field can remain temporarily as a derived summary count for UI compatibility, but egg inventory becomes the source of truth.

Required:

- keep egg count synchronized from `egg_inventory`
- stop directly awarding or spending abstract egg counts in business logic

## UI Changes

### 1. Town Overview
Replace the current direct hatch action with chamber workflow actions:

- `Hatch Slime Girl`

The panel should also show:

- raw egg count
- ready egg count
- what species pool is available when hovered

### 2. Opening Chapter
The opening chapter should continue to feel authored, but internally use the same egg/chamber pipeline.

### 3. Error Messages
Failures should be explicit:

- no compatible raw egg
- no ready egg
- species not available from current eggs

## Save And Migration
Phase C changes core save structure again.

Required:

- bump `save_version`
- do not migrate older saves

## Implementation Order

### Step 1. Add Egg And Chamber State
- add egg structs/enums
- attach both to `GameState`

### Step 2. Add Egg Inventory Helpers
- add count-sync helper
- add opening egg creation helper
- add expedition egg creation helper
- add chamber-aware hatch helpers

### Step 3. Rewrite Opening Egg Flow
- use egg inventory during discovery/incubation/hatch

### Step 4. Rewrite Town Hatch Actions
- make hatch handle chamber preparation and loyalty imprinting

### Step 5. Verification
- save version bump
- fmt, clippy, native build, wasm build, tests

## Acceptance Criteria
Phase C is complete when:

- `GameState` stores egg inventory records
- opening egg creation uses inventory, not only `resources.eggs`
- expedition egg rewards produce raw egg records
- direct monster-girl creation no longer bypasses chamber preparation
- egg count shown in UI matches egg inventory

## Defaults Chosen
- egg inventory is the source of truth
- `resources.eggs` remains as a synchronized summary for this phase
- chamber incubation is immediate in Phase C
