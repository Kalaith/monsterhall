# Phase B Implementation Plan: Sexual Skill Progression Rewrite

## Summary
Phase B replaces the current generic monster-girl `level` and `experience` framing with the first real sexual progression model. This phase does not attempt the full late-game erotic stat simulation. It establishes the core runtime structure and gameplay loop needed for future content.

The first implementation slice should introduce:

- explicit sexual skill tracks
- explicit sexual history counters
- room-driven skill growth
- opening/first-client skill growth
- UI visibility for the new progression model
- save/runtime compatibility for the new fields

This phase should not yet implement the full body-stat or guest-requirement system. It should create the progression spine that later phases can build on.

## Core Outcome
After this phase:

- girls no longer feel like they are using generic RPG levels
- brothel work grows specific sexual skills instead of only producing money
- the first client and later brothel shifts begin building sexual history
- the player can see the difference between:
  - what a girl is trained to do
  - what a girl has actually experienced
- future content can gate scenes and guests on these stats without another structural rewrite

## Initial Scope

### Sexual Skill Stats
Implement these starter fields:

- `scouting`
- `guarding`
- `hospitality`
- `crafting`
- `charm`

These are the only required starter skill fields for Phase B.

### Sexual History Counters
Implement these starter fields:

- `kiss_count`
- `sex_count`
- `creampie_count`
- `masturbation_count`
- `birth_count`

These are enough to support future experience-gated scenes and guest requirements.

### Out Of Scope For This Phase
- body stats
- breast/height tracking
- full guest requirement system
- birth system
- affection/jealousy/emotional simulation
- full room-act matrix

Those remain later phases.

## Runtime Model Changes

### 1. Monster Girl State
Replace or de-emphasize the current `level` / `experience` model in `MonsterGirlState`.

Recommended runtime change:

- remove `level`
- remove generic `experience`
- add `sex_skills`
- add `sex_history`

Suggested structures:

- `SexSkillState`
  - `scouting: u32`
  - `guarding: u32`
  - `hospitality: u32`
  - `crafting: u32`
  - `charm: u32`
- `SexHistoryState`
  - `kiss_count: u32`
  - `sex_count: u32`
  - `creampie_count: u32`
  - `masturbation_count: u32`
  - `birth_count: u32`

These should live under `state/` and serialize as part of saves.

### 2. GuildJobs Room Data
GuildJobs rooms need to declare what they train.

Add room-level data fields:

- `primary_skill_id`
- `secondary_skill_id` as optional, or
- `trained_skill_ids` as a list

Recommended choice:

- `trained_skill_ids: Vec<String>`

Reason:

- keeps room training data extensible
- avoids hardcoding room-to-skill mapping in engine logic
- supports multi-skill growth later

Phase B should only allow the current starter skills as valid ids.

## Gameplay Logic Changes

### 1. GuildJobs Shift Training
Each brothel shift should now do two things:

- generate money/lust as it already does
- train the room’s declared sexual skills

Recommended first-pass rule:

- each trained skill gains a flat amount per completed shift
- `charm` can grow in most service rooms
- the room’s primary penetration/service style should grow faster than support skills

### 2. Sexual History Updates
GuildJobs work and scripted opening content should increment history counters.

Minimum Phase B behavior:

- first client increments `sex_count`
- first client increments `hospitality`
- first client increments `kiss_count`
- standard brothel shifts increment `sex_count`
- standard brothel shifts increment room-relevant history where unambiguous

Because the current room set is still broad, Phase B should avoid fake precision. If a room’s exact act mix is unclear, only update:

- `sex_count`
- obvious matching skills

Do not invent deep act accounting without room/service data to support it.

### 3. Opening Chapter Integration
The opening client should feed the new progression model directly.

It should:

- increase `scouting`
- increase `hospitality`
- increase `charm`
- increment `kiss_count`
- increment `sex_count`
- increment `creampie_count` only if the scene is defined that way in the opening content

### 4. Mutation / Expedition Compatibility
Expedition logic can remain mostly unchanged in Phase B.

However:

- no code should still treat `level` as a meaningful progression field
- existing formulas should continue using visible stats for now

Phase B does not need to tie sexual skills into expeditions yet.

## UI Changes

### 1. Roster Cards
Replace `level` / `experience` style display with:

- short skill summary
- short history summary

Recommended compact display:

- `Skills: K/O/V/A/S`
- `History: kisses / sex / creampies`

### 2. GuildJobs Management
GuildJobs worker preview should explain training, not just payout.

Add:

- which skills the room trains
- what the assigned girl will improve by working there

### 3. Opening / Results
Day results and opening-client results should mention gained skill or history progress when relevant.

This is important because it teaches the player that sex work is progression, not just income.

## Save And Migration
This phase changes core progression state and invalidates old saves.

Required:

- bump `save_version`

Do not attempt migration from pre-skill-track saves.

## Implementation Order

### Step 1. Add New State Structs
- `SexSkillState`
- `SexHistoryState`
- attach them to `MonsterGirlState`
- remove `level` and `experience`

### Step 2. Add Room Training Data
- extend `BrothelRoomData`
- update room JSON
- validate allowed skill ids

### Step 3. Update Bootstrap / Hatch / Opening Creation
- all girl creation paths must initialize the new skill/history structs
- first client should award the first scripted gains

### Step 4. Update GuildJobs Resolution
- apply room-driven skill growth
- apply history counter growth
- add result text for training gains

### Step 5. Update UI
- roster cards
- brothel screen
- opening result messages

### Step 6. Save Version / Verification
- bump version
- run fmt, clippy, native build, wasm build

## Acceptance Criteria
Phase B is complete when:

- `MonsterGirlState` no longer relies on generic `level` / `experience`
- every girl has visible sexual skill stats and history counters
- the first client updates those stats
- brothel shifts update those stats
- rooms declare which skills they train
- the player can see sexual progression in the UI without needing debug output

## Defaults Chosen
- starter skills: `scouting`, `guarding`, `hospitality`, `crafting`, `charm`
- starter history: `kiss_count`, `sex_count`, `creampie_count`, `masturbation_count`, `birth_count`
- room training is data-driven via `trained_skill_ids`
- expeditions stay structurally unchanged in this phase
- old saves are invalidated by save-version bump
