# Phase A Implementation Plan: Opening Chapter Rewrite

## Summary
Phase A replaces the current generic `New Campaign -> Town Overview` start with a directed opening chapter that establishes the real game premise:

1. the protagonist camps at the tower
2. he discovers the wet opening
3. he ejaculates into it
4. the tower produces the first egg
5. the first slime girl hatches
6. the player must build the first visitor room
7. the first paid service begins the business loop

This phase is not a content-only pass. It requires structural changes to game phases, save state, onboarding, and the early progression model so the game can support story-driven startup instead of dropping directly into a freeform management shell.

The goal of Phase A is to make the first 10 to 20 minutes of play feel authored, understandable, and tied to the final game fantasy while still feeding into the existing long-form management structure.

## Core Outcome
After this phase, a new game should no longer begin as an already-running business. It should begin as a narrative setup sequence that gradually hands control to the player.

The new opening should teach:

- why the tower matters
- what the chamber does
- why the first slime girl matters
- why a room must be built
- how money starts flowing
- why the player will eventually need more girls, deeper floors, and better services

The player should arrive at freeform management only after the core premise has been established in play.

## Structural Changes

### 1. Replace Generic New Game Start With Opening Chapter Flow
The current start path creates a campaign and sends the player directly into town management. Replace that with a structured campaign-intro flow.

Add explicit opening phases between `MainMenu` and normal town play. Recommended sequence:

- `OpeningCamp`
- `OpeningDiscovery`
- `OpeningIncubation`
- `OpeningHatch`
- `OpeningFirstRoom`
- `OpeningFirstClient`
- then transition into standard management flow

These can be represented either as:

- multiple explicit `GamePhase` variants, or
- one `OpeningChapter` phase with a strongly typed substate enum

Recommended choice:

- use one `OpeningChapter` phase with `OpeningChapterStep`

Reason:

- keeps the top-level phase list manageable
- keeps all scripted intro logic in one place
- avoids scattering one-off startup logic across the general management states

### 2. Add Chapter Progress State To Save Data
The opening chapter must be resumable and must not replay incorrectly after completion.

Add campaign progress fields to runtime/save state:

- intro chapter state
- whether the chamber has been discovered
- whether the first egg has been produced
- whether the first slime girl has hatched
- whether the first room has been built
- whether the first paid client has been completed

Recommended shape:

- a compact `StoryProgressState`
- an `OpeningChapterProgress` enum or struct for intro milestones

Do not encode this through fragile event-log string matching.

### 3. Split “New Game” From “Freeform Town Start”
`create_new_game_state` should no longer create a campaign that already feels like a functioning operation.

New game should start with:

- the protagonist at the tower
- no active room in commercial use
- no existing workforce except the not-yet-created first slime girl
- enough starting resources to complete the intended opening tasks
- story flags that put the player into the intro chapter instead of normal town management

This means new game bootstrap must become chapter-aware rather than assuming the player starts inside the stable management loop.

## New Runtime/Data Requirements

### 1. Story Progress Model
Add story progression state under runtime/save data.

Minimum fields:

- `opening_step`
- `tower_hole_discovered`
- `first_egg_created`
- `first_slimegirl_hatched`
- `first_room_built`
- `first_client_completed`

Recommended follow-up fields, even if not fully used yet:

- `chamber_secret_known`
- `creditor_intro_seen`
- `first_debt_notice_seen`

### 2. Opening Event Content
Add a dedicated story JSON domain for authored intro events and chapter text.

Recommended new data file:

- `assets/data/story_events.json`

This should hold:

- opening scene text
- discovery scene text
- incubation scene text
- first hatch scene text
- first room build explanation
- first client explanation

The opening should not be hardcoded as a pile of inline UI strings inside the game coordinator.

### 3. First Girl Bootstrap Rule
The first slime girl should be created by the opening chapter, not by the generic hatch button in the town screen.

Implementation rule:

- the scripted opening creates the first egg and hatches the first slime girl through a chapter action
- later eggs use the general egg/chamber systems when those are implemented

This keeps the opening deterministic and avoids forcing the player through incomplete future systems too early.

### 4. First Room Progress Rule
The player should not start with a functioning visitor room.

Opening progression should require:

- first slime girl exists
- room does not yet exist for paid service
- player is directed to build the first room as a meaningful goal

Recommended implementation:

- introduce a dedicated first-room unlock/build step
- gate first client access until the room exists

## UI And Flow Changes

### 1. Opening Chapter Screens
The intro should use mouse-driven, screen-based chapter panels rather than trying to force all story beats through the normal town overview.

Recommended UI behavior:

- full-screen narrative panel for each opening beat
- one or two explicit clickable choices when needed
- a single continue/proceed action when the step is linear
- no keyboard shortcut prompts shown

The goal is to make the first minutes legible and paced, not dump the player into an overloaded town screen with a pop-up.

### 2. Transition Into Management
Once the first client is completed:

- transition into normal town overview
- show updated resources and the first slime girl in the roster
- show a clear “what next” status message

Recommended first handoff state:

- `TownOverview` with a message like “The business has begun. Build, train, and go deeper.”

### 3. Onboarding Integration
The current onboarding hints should be rewritten to respect whether the player is:

- still in the scripted chapter
- in the first freeform day after the intro
- already beyond onboarding

Do not show generic “hatch a slime girl” guidance during the scripted opening if the first slime girl is already part of the chapter.

## Business Logic Changes

### 1. New Game Bootstrap
Refactor bootstrap so:

- story start state is initialized
- first slime girl is not preloaded unless the intro step says she exists
- the campaign begins in an intro-aware state

Recommended approach:

- `create_new_game_state` returns a minimal campaign shell
- separate opening-chapter resolver functions perform the scripted transitions

This is cleaner than embedding all intro side effects directly into bootstrap.

### 2. Opening Step Actions
Create explicit business-logic handlers for each intro action.

Recommended functions:

- `advance_opening_step`
- `discover_tower_hole`
- `trigger_first_egg`
- `hatch_first_slimegirl`
- `build_first_room`
- `resolve_first_client`

These should live in `engine/`, not `ui/`.

### 3. Deterministic First Slime Girl
The first slime girl should be deterministic in shape and role for now.

Recommended defaults:

- species: `slime_girl`
- temperament flavor: timid, friendly, sexy
- loyalty starts high because of chamber imprinting
- no random alternate first-girl outcome

Her generated name can still come from the normal name pool if desired.

### 4. First Client Resolution
The first paid service should be simplified and authored.

Recommended behavior:

- one safe early client
- one small gold payout
- one small increase to first-girl experience
- one explicit explanation that this is how the business begins

This should not depend on the full later guest-request system.

## Save And Migration Considerations
This phase changes the meaning of campaign start and likely invalidates old saves.

Required action:

- bump `save_version`

Required behavior:

- incompatible saves should show a clear message that the opening chapter structure changed
- do not attempt a complicated migration from pre-opening-chapter saves

Recommended message:

- older MVP saves are incompatible with the story-start rewrite and a new campaign is required

## Implementation Order

### Step 1. Add Story Progress State
- add intro progress structs/enums to runtime/save state
- thread them through save/load

### Step 2. Add Opening Story Content Domain
- create `story_events.json`
- add loader and validation

### Step 3. Add Opening Chapter Phase
- add `OpeningChapter` to `GamePhase`
- add chapter draw/update routing
- make `StartNewGame` enter the chapter flow instead of town overview

### Step 4. Add Opening Logic Handlers
- scripted discovery
- scripted egg creation
- scripted first hatch
- scripted first room requirement
- scripted first client completion

### Step 5. Rewrite New Game Bootstrap
- remove the assumption that the business is already live
- initialize only what the opening needs

### Step 6. Handoff Into Normal Town Play
- transition to standard management after first client
- update onboarding/status messaging

### Step 7. Save Version + Guardrails
- bump save version
- add clear incompatibility messaging
- make sure the intro is resumable and cannot replay incorrectly after completion

## Files/Subsystems Expected To Change
Primary areas:

- `src/game.rs`
- `src/state/`
- `src/engine/`
- `src/ui/screens.rs`
- `src/data/loader.rs`
- `assets/data/`

Recommended new content file:

- `assets/data/story_events.json`

Recommended new runtime grouping:

- opening-chapter state under `state/`
- opening chapter logic under `engine/`

## Test Plan

### Runtime/State Tests
- new campaign starts in opening chapter, not normal town overview
- opening chapter progress serializes and deserializes correctly
- completed intro does not replay after save/load

### Logic Tests
- first egg can only be created once in the intro path
- first slime girl is created exactly once
- first room step cannot be skipped
- first client cannot resolve before room build step

### UI/Flow Tests
- opening screens are mouse-complete
- no keyboard shortcuts are shown on screen
- after first client, the player lands in normal town play with the correct state

### Save Compatibility Tests
- old save version shows a clear incompatibility message
- new save resumes intro chapter mid-step correctly

## Acceptance Criteria
Phase A is complete when:

- new campaigns always begin with the authored opening chapter
- the first slime girl comes from the scripted premise, not generic town UI
- the first room is a required progression goal
- the first client is an explicit onboarding milestone
- the player reaches normal town management only after understanding the premise
- save/load works correctly across intro progress

## Defaults Chosen
- use one `OpeningChapter` top-level phase with typed substates
- store intro progress explicitly in save state
- create a new `story_events.json` domain for authored opening content
- keep the first slime girl deterministic
- keep the first client authored and simplified
- do not migrate old MVP saves; require a new campaign after save-version bump
