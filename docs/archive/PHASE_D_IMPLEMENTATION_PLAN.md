# Phase D Implementation Plan: Debt And Guest Systems

## Summary
Phase D turns the current freeform brothel loop into a game with pressure, targets, and differentiated customers.

Right now the player can earn resources, hatch girls, and improve sexual skills, but the business does not yet push back in a meaningful way. Phase D adds that pressure through two connected systems:

1. debt milestones that force the business to keep growing
2. guest requests that demand specific girls, services, and training

This phase should make the player care about:

- who is being trained
- which rooms are being built
- what kinds of girls are available
- whether the current roster can satisfy upcoming demand

It should also establish the first real economic tension:

- slime-girl starter income can keep the business alive briefly
- debt and guest specificity eventually require broader growth

Phase D should not attempt the full late-game story guest library. It should build the runtime structure, data model, UI flow, and first playable content slice for debt and guest-driven progression.

## Core Outcome
After this phase:

- the game has active debt obligations instead of only passive money accumulation
- creditors appear on a predictable cadence with visible milestone targets
- guests are request objects, not only generic room income
- guest satisfaction depends on actual requirements
- girls can be accepted or rejected for a guest based on species, room/service, and sex-skill thresholds
- passing or failing debt and guest checks creates consequences
- the player can see upcoming pressure and plan around it

## Initial Scope

### Debt System Starter Scope
Implement a first-pass debt model with:

- one active debt track
- fixed payment checkpoints
- due-day tracking
- visible consequences for failure
- visible relief/reward for success

Starter debt content should include:

- opening debt established after the intro chapter
- at least 3 milestone payments
- increasing pressure from milestone to milestone

### Guest System Starter Scope
Implement a first-pass guest request model with:

- common guests
- at least one special guest type
- request generation from data
- explicit accept / reject / fail / complete outcomes

Starter guest requirements should support:

- required room or service identity
- required species id or species tag
- minimum sexual skill thresholds
- optional minimum history counts

### Out Of Scope For This Phase
- full body-stat requirement system
- deep loyalty/emotion guest simulation
- jealousy/exclusivity chains
- breeding request system
- late-game elite guest library
- multiple simultaneous creditors
- full narrative debt scene chain

Those remain later content phases.

## Runtime Model Changes

### 1. Debt State
Add explicit debt runtime/save state under `state/`.

Recommended structures:

- `DebtState`
  - `active_milestone_id: String`
  - `current_balance_due: u32`
  - `days_until_due: u32`
  - `missed_payment_count: u32`
  - `resolved_milestone_ids: Vec<String>`
  - `status_message: String`

- `DebtResolution`
  - `PaidOnTime`
  - `PaidLate`
  - `Missed`

Reason:

- debt must be readable in saves
- day advancement needs to decrement due timers
- future story phases can hang events off debt state cleanly

### 2. Guest Request State
Add explicit guest-request runtime/save state.

Recommended structures:

- `GuestRequestState`
  - `request_id: String`
  - `template_id: String`
  - `guest_name: String`
  - `archetype_id: String`
  - `requested_room_id: String`
  - `required_species_ids: Vec<String>`
  - `required_skill_thresholds: GuestSkillRequirementState`
  - `required_history_thresholds: GuestHistoryRequirementState`
  - `reward: ResourceAmountState or equivalent`
  - `penalty_gold: u32`
  - `deadline_day: u32`
  - `status: GuestRequestStatus`
  - `assigned_monster_id: Option<String>`

- `GuestRequestStatus`
  - `Pending`
  - `Accepted`
  - `Completed`
  - `Failed`
  - `Declined`

Recommended helper structs:

- `GuestSkillRequirementState`
  - `scouting`
  - `guarding`
  - `hospitality`
  - `crafting`
  - `charm`

- `GuestHistoryRequirementState`
  - `kiss_count`
  - `sex_count`
  - `creampie_count`
  - `masturbation_count`
  - `birth_count`

Store active requests on `GameState` as:

- `active_guest_requests: Vec<GuestRequestState>`

Phase D does not need an unlimited request board. A small capped active list is enough.

### 3. Story Progress Integration
Extend story/runtime progression only where needed.

Recommended new fields:

- `first_creditor_visit_seen`
- `first_special_guest_seen`

These should remain simple boolean story flags, not replace the real debt/request state.

## Content/Data Changes

### 1. New Debt Data Domain
Add:

- `assets/data/debt_milestones.json`

Starter fields per milestone:

- `id`
- `name`
- `description`
- `amount_due`
- `days_allowed`
- `reward_gold` or `reward_resource_bundle`
- `failure_penalty_gold`
- `failure_stress_flat`
- `next_milestone_id`

This keeps debt pacing data-driven.

### 2. New Guest Data Domains
Add:

- `assets/data/guest_archetypes.json`
- `assets/data/guest_requests.json`

Recommended split:

- `guest_archetypes.json`
  - identity and flavor framing
  - guest tags
  - default payout flavor
  - spawn weighting

- `guest_requests.json`
  - concrete request templates
  - room requirement
  - species requirement
  - skill thresholds
  - history thresholds
  - payout and penalty values

Do not bury request requirements inside hardcoded Rust tables.

### 3. Room Data Integration
Existing guild room data already defines service identity and training behavior. Phase D should reuse that.

Add only the minimum missing fields if needed:

- `guest_tags_supported`
- or `service_tags`

Only add these if request matching actually needs them. Do not expand room schema speculatively.

## Gameplay Logic Changes

### 1. Debt Initialization
When a new campaign leaves the opening chapter:

- initialize the first debt milestone
- set the first due timer
- show a clear debt status message

This should happen through `engine/`, not UI-only messaging.

### 2. Debt Day Tick
On each resolved day:

- decrement `days_until_due`
- check whether payment is due
- if player has enough gold and chooses to pay, resolve the milestone
- if deadline passes unpaid, apply penalties

Minimum Phase D rule:

- missed payment does not end the campaign immediately
- missed payment applies real pain:
  - lose gold
  - add stress to active girls or roster-wide flat stress
  - worsen next milestone pressure

### 3. Guest Request Generation
Implement a simple request-generation loop.

Starter behavior:

- maintain a small active request list
- generate new requests from data templates
- bias early requests toward starter rooms and slime-compatible content
- unlock more demanding requests as rooms, species, and debt milestones advance

Recommended first-pass cap:

- 3 active requests maximum

### 4. Guest Requirement Evaluation
Guest requests should evaluate against actual girl state.

Minimum validation checks:

- requested room is unlocked
- assigned girl matches any required species ids
- assigned girl meets all required sex-skill thresholds
- assigned girl meets any required history thresholds

If a girl does not qualify, the UI should explain why.

### 5. Guest Resolution
Completing a guest request should:

- consume the request
- award gold/resources
- increase room-relevant skills/history
- generate a result line in day results

Failing or missing a guest request should:

- mark it failed or expired
- apply its penalty
- generate a clear result line

### 6. Special Guest Starter Slice
Add one authored special guest request chain.

Purpose:

- prove the system can support more than generic demand
- teach the player that exact requirements matter

Recommended starter shape:

- one guest who requires a specific room
- one minimum `charm` or `hospitality` threshold
- higher payout than common guests

## UI Changes

### 1. Debt Panel
Add visible debt info to the main town screen.

Show:

- current milestone name
- amount due
- days remaining
- consequence preview if missed

The player should never have to guess whether debt is active.

### 2. Guest Desk Screen
Add a dedicated guest-management screen or extend brothel management cleanly.

Recommended choice:

- add a dedicated `GuestManagement` screen

Reason:

- keeps request selection readable
- avoids overloading the existing brothel assignment screen
- supports future special guest chains without UI collapse

Minimum screen sections:

- active requests list
- selected request detail panel
- eligible girl preview
- assign / accept controls
- deadline / reward / penalty summary

### 3. Requirement Feedback
When a girl does not qualify, show exact failure reasons such as:

- requires `Seduction 3`
- requires `Slime Girl`
- requires `Vanilla Suite`
- requires `Sex Count 5`

Do not return generic “requirements not met.”

### 4. Day Results
Debt and guest outcomes must appear in day results.

Add result lines for:

- debt payment made
- debt missed
- guest completed
- guest failed
- new guest arrived

This teaches the player how the new systems work without a separate manual.

## Save And Migration
Phase D changes core runtime/save structure again.

Required:

- bump `save_version`

Do not attempt migration from pre-debt / pre-guest saves.

## Implementation Order

### Step 1. Add Debt State And Data
- add `DebtState` under `state/`
- attach it to `GameState`
- add `debt_milestones.json`
- add loader validation for milestone chains and values

### Step 2. Add Guest Request State And Data
- add `GuestRequestState`
- attach active request list to `GameState`
- add `guest_archetypes.json`
- add `guest_requests.json`
- validate room ids, species ids, and skill/history keys

### Step 3. Initialize Debt In Campaign Flow
- hook first debt milestone into post-opening campaign start
- add status messaging to intro handoff
- ensure new games begin Phase D-ready

### Step 4. Add Request Generation And Matching Helpers
- add guest generation helpers in `engine/`
- add requirement-evaluation helpers
- add “why not eligible” explanation helper

### Step 5. Add Guest Management Screen
- add a dedicated game phase/state
- add request list and selected request details
- add mouse-driven assignment flow
- add visible reward / penalty / deadline info

### Step 6. Resolve Debt And Guest Outcomes In Day Cycle
- decrement debt timer on day advance
- allow debt payment resolution
- apply missed-payment penalties
- resolve accepted guest requests
- add result text for all outcomes

### Step 7. Add Starter Content Slice
- at least 3 debt milestones
- at least 3 common guest templates
- at least 1 special guest chain
- early-game requests that are actually completable from current content

### Step 8. Save Version And Verification
- bump version
- run fmt
- run clippy
- run native build
- run wasm build
- run tests

## Acceptance Criteria
Phase D is complete when:

- a running campaign always has visible debt state
- the player can see upcoming payment pressure and consequences
- guest requests exist as real runtime objects
- requests are generated from JSON data
- requests can require room, species, skill, and history thresholds
- the game explains why a girl is or is not eligible
- completing requests gives rewards and progression
- missing debt or guest deadlines applies penalties
- debt and guest outcomes appear in day results

## Defaults Chosen
- one active creditor track
- one current debt milestone at a time
- up to 3 active guest requests
- requests are data-driven from JSON templates
- guest checks use current starter sex skills and history counters
- body-stat requirements remain out of scope until a later phase
- old saves are invalidated by save-version bump
