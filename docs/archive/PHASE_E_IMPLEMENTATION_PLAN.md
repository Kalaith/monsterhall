# Phase E Implementation Plan

## Goal
Expand the roster and tower content so the current management systems have real breadth to work with.

Phase D added debt and guest pressure. Phase E makes that pressure meaningful by giving the player:
- more species to hatch and raise
- more floor-specific egg outcomes
- stronger reasons to go deeper into the tower
- clearer species, room, and guest specialization

This phase is about **content with supporting structure**, not just dumping more JSON into the project.

## Scope

### In Scope
- additional playable species and species unlock flow
- deeper floor content and egg tables
- floor-to-species progression
- mutation route expansion
- corruption-sensitive species/content
- species, room, and guest synergy pass
- UI updates needed to make the new breadth readable

### Out Of Scope
- Phase F balance polish
- body stat system rewrite
- new debt framework
- new guest framework
- full tutorial/onboarding rewrite
- scene writing beyond the amount needed to support the new species/floor progression

## Design Targets

By the end of Phase E:
- Floor 1 should clearly feel like a slime-focused starter floor.
- Floor 2 and beyond should materially change what eggs and girls are available.
- Different species should feel mechanically different in room use, guest suitability, and expedition use.
- Mutation/corruption paths should feel like deliberate progression options, not background flavor.
- Guest and debt systems should have a broader roster to push against.

## Implementation Steps

### Step 1. Lock The Content Expansion Matrix
Define the concrete roster and floor targets for this phase.

Decide and document:
- which species are considered early, mid, and late Phase E content
- which floors unlock which species pools
- which species are natural hatch outcomes vs mutation outcomes
- which species are corruption-sensitive or corruption-driven

Recommended baseline:
- Early:
  - `slime_girl`
  - `succu_slime`
  - `goblin_maid`
- Mid:
  - `harpy_hostess`
  - `lamia_binder`
- Deeper:
  - `golemkin_warden`
  - `hellhound_bouncer`
  - `moth_priestess`

Acceptance:
- one clear matrix exists mapping floors -> eggs -> hatch outcomes -> mutations

### Locked Phase E Content Matrix

#### Roster Bands

##### Early Species
- `slime_girl`
  - baseline hatch species
  - safe starter room fit
  - main floor 1 identity
- `succu_slime`
  - early corruption-sensitive upgrade path
  - still reachable from floor 1
  - acts as the bridge from starter slime content into more specialized lust content

##### Mid Species
- `goblin_maid`
  - opportunistic service/scavenging girl
  - introduced as the first clearly non-slime utility species
- `harpy_hostess`
  - spectacle/showmanship species
  - first strong public-stage leaning hatch target
- `hellhound_bouncer`
  - rougher, sturdier species
  - first clear “dangerous floor / rough guest” hatch target
- `lamia_binder`
  - ritual/private-control species
  - first strong breeding/private-room specialist

##### Deep Species
- `golemkin_warden`
  - corruption-heavy advanced species
  - deeper-room specialist
  - major mutation payoff species
- `moth_priestess`
  - relic/corruption/elite ritual species
  - deep-floor prestige hatch target

#### Floor Identity Matrix

##### Floor 1: `floor_1_slick_cellars`
- identity:
  - starter slime floor
  - low-risk egg acquisition
  - materials and light lust residue
- egg species:
  - `slime_girl`
  - low-chance `succu_slime`
- role in progression:
  - teaches the hatch loop
  - supports the first roster growth
  - should never become the best place for broad roster expansion once deeper floors are open

##### Floor 2: `floor_2_molten_baths`
- identity:
  - heat/lust/corruption transition floor
  - the first floor where pushing deeper changes the tone of the roster
- egg species:
  - `succu_slime`
  - `harpy_hostess`
  - `hellhound_bouncer`
  - rare fallback `slime_girl`
- role in progression:
  - first strong non-slime roster expansion
  - first real corruption-sensitive floor
  - should feel like the point where “just slime girls” stops being enough

##### Floor 3: `floor_3_gilded_kennels`
- identity:
  - noble-beast handling floor
  - species with stronger service identity and better guest differentiation
- egg species:
  - `goblin_maid`
  - `lamia_binder`
  - lower-rate `succu_slime`
- role in progression:
  - broadens room and guest specialization
  - introduces species that matter more for specific bookings than raw early income

##### Floor 4: `floor_4_heart_vault`
- identity:
  - deep corruption / relic / prestige floor
  - rare and advanced hatch targets
- egg species:
  - `golemkin_warden`
  - `moth_priestess`
- role in progression:
  - advanced roster capstone for Phase E
  - supports elite guest and corruption-sensitive content

#### Hatch Outcome Policy
- floor eggs should be treated as weighted pools by floor identity, not flat equal-value buckets
- earlier floors may still contain limited lower-tier fallback eggs, but deeper floors should primarily reward new roster access
- `slime_girl` remains the default baseline species, but not the dominant long-term answer to Phase E content

#### Mutation Matrix

##### Confirmed Mutation Routes
- `slime_girl -> succu_slime`
  - early corruption route
  - turns starter slime investment into a premium lust worker
- `succu_slime -> golemkin_warden`
  - deeper corruption route
  - acts as the main slime-line advanced payoff

##### Locked Expansion Targets For Phase E
- keep the slime corruption line as the core mutation spine
- add at least one non-slime mutation route during Step 4
- the non-slime mutation route must create a species that is useful for either:
  - a deeper room specialization
  - a high-value guest specialization
  - a relic/corruption-sensitive content branch

##### Chosen Non-Slime Route
- `harpy_hostess -> moth_priestess`
  - turns a spectacle/public-stage species into an elite ritual/relic species
  - gives the roster a second path into prestige content besides pure deep-floor egg access
  - supports later guest and room specialization work around elite ceremonial service

#### Corruption-Sensitive Species Policy
- `succu_slime`
  - early corruption-sensitive species
- `golemkin_warden`
  - advanced corruption species
- `moth_priestess`
  - corruption/relic-sensitive deep species

These species should be the main recipients of corruption-sensitive guest, room, or event content in later Phase E steps.

#### Room / Species Intent
- `slime_girl`
  - starter intimacy and flexible early service
- `succu_slime`
  - essence generation, charm, and premium lust value
- `goblin_maid`
  - practical service, shamelessness, and opportunistic bookings
- `harpy_hostess`
  - spectacle, stage, and high-visibility charm
- `hellhound_bouncer`
  - rough clients, dangerous dives, and protective endurance
- `lamia_binder`
  - ritual/private control scenes and breeding-adjacent specialization
- `golemkin_warden`
  - corruption-heavy specialty content and deeper-room service
- `moth_priestess`
  - relic-linked elite ritual content

#### Step 1 Decision Lock
Phase E content work should now assume:
- Floor 1 is slime-led
- Floor 2 is the lust/corruption transition floor
- Floor 3 is the specialization floor
- Floor 4 is the advanced prestige floor
- slime mutation remains the main early-to-deep corruption ladder
- at least one non-slime mutation path will be added later in Phase E

### Step 2. Expand Floor And Egg Progression Data
Update floor data so tower depth actually drives roster growth.

Required changes:
- add or revise floor entries in `assets/data/floors.json`
- define which species ids each floor can yield as eggs
- ensure floor descriptions and difficulty progression match the new content
- make deeper floors materially better than floor 1, not just numerically different

Support changes:
- adjust mission compatibility if current floors need more than one meaningful expedition target
- ensure egg outcomes feel tied to floor identity

Acceptance:
- each unlocked floor has a clear species identity
- deeper floors produce species the player cannot reliably get from earlier content

### Step 3. Expand Species Catalog And Name Pools
Add or deepen species content so each species is more than a label swap.

Required changes:
- expand `assets/data/species.json`
- expand `assets/data/monster_names.json`
- ensure every species has:
  - description
  - portrait key
  - intentional stats
  - trait set
  - preferred rooms
  - real hatch cost

Acceptance:
- every new or revised species loads cleanly
- every hatchable species has a name pool
- no placeholder stat/cost content

### Step 4. Expand Mutation Routes
Make mutation a real branch of progression instead of a token hook.

Required changes:
- add mutation routes in `assets/data/mutations.json`
- ensure mutation chains have:
  - clear source species
  - target species
  - corruption thresholds
  - trait requirements if needed
  - event text

Recommended Phase E routes:
- strengthen slime corruption line
- add at least one non-slime mutation route
- make at least one mutation strategically valuable for guest or room specialization

Acceptance:
- the game has multiple meaningful mutation routes
- mutation targets are worth pursuing, not just novelty

### Step 5. Do A Species / Room Synergy Pass
Use existing guild room systems to make species differences readable in play.

Required changes:
- review `assets/data/brothel_rooms.json`
- review preferred species lists per room
- adjust room identity so species specialization is visible

Questions this pass should answer:
- which species are good in gentle starter rooms
- which species are best in spectacle rooms
- which species support rougher or corruption-heavy content
- which species are best for private, breeding, or ritual content

Acceptance:
- room assignments reward species choice
- players can look at room/species data and understand why a pairing is good

### Step 6. Do A Species / Guest Synergy Pass
Use existing guest request systems to make the expanded roster matter.

Required changes:
- expand `assets/data/guest_requests.json`
- revise `assets/data/guest_archetypes.json` only if needed
- add guest requests that:
  - prefer specific species
  - require deeper-room access
  - reward specialized girls more than starter slime-only play

Recommended structure:
- early guests should remain slime-compatible
- mid-game guests should require specific species or skill/history combinations
- at least one higher-value guest line should push the player toward corruption or mutation content

Acceptance:
- guest desk contains requests that make the new species roster strategically useful

### Step 7. Deepen Expedition Incentives
Make tower dives feel tied to real roster goals.

Required changes:
- review mission/floor reward flow in:
  - `assets/data/floors.json`
  - `assets/data/missions.json`
  - expedition resolution logic if needed
- ensure expeditions provide:
  - materials
  - eggs
  - corruption opportunities
  - relic relevance where appropriate

Acceptance:
- tower expeditions are the main path to expanded species access
- players have a clear reason to push deeper beyond “more numbers”

### Step 8. Update Town, Chamber, And Planning UI For Breadth
The current UI was built around a small roster. It needs a readability pass once species/floors expand.

Required changes:
- town overview:
  - species summaries remain readable with more than one hatch target
- chamber:
  - egg outcome preview remains understandable with mixed inventories
- expedition planning:
  - deeper floors and their identities are readable
- brothel and guest screens:
  - species specialization is visible enough to make assignment decisions

This is not a full UI architecture refactor. It is a content-readability pass.

Acceptance:
- expanded content is understandable from the screen flow
- no screen becomes unusable once more species/floors are present

### Step 9. Expand Supporting Flavor Content
Add enough supporting text/events so the new species and floors feel like authored content.

Required changes:
- expand `assets/data/events.json`
- update any floor/species/mutation text needed to support the new content
- ensure the event pool does not over-select the same old slime-only flavor

Acceptance:
- new species/floors have matching flavor presence in the event layer

### Step 10. Verify, Tune, And Lock Phase E
Before declaring Phase E complete:
- run build/test/clippy
- do a content consistency sweep
- do at least one manual progression pass from early to deeper floors

Validation checklist:
- every new species is reachable somehow
- every deeper floor has distinct egg value
- guest requests do not demand impossible combinations too early
- mutation routes can actually trigger
- no new JSON domain contains placeholder values

Acceptance:
- Phase E content is fully playable on the existing systems

## Data Files Expected To Change

Primary:
- `assets/data/species.json`
- `assets/data/monster_names.json`
- `assets/data/floors.json`
- `assets/data/mutations.json`
- `assets/data/brothel_rooms.json`
- `assets/data/guest_requests.json`
- `assets/data/events.json`

Possible support changes:
- `assets/data/missions.json`
- `assets/data/config.json`

Code likely to change:
- `src/data/loader.rs`
- `src/engine/day_cycle.rs`
- `src/engine/guest.rs`
- `src/ui/screens.rs` or its replacement modules as the UI refactor progresses

## Recommended Delivery Order

### Pass 1. Content Matrix And Floor Expansion
- Step 1
- Step 2
- Step 3

### Pass 2. Mutation And Synergy
- Step 4
- Step 5
- Step 6

### Pass 3. Expedition And UI Readability
- Step 7
- Step 8
- Step 9

### Pass 4. Verification
- Step 10

## Completion Criteria
- the game has a broader, intentional species roster
- deeper floors materially matter
- mutation is a usable progression path
- rooms and guests reward species specialization
- tower dives are the main route to advanced roster growth
- all new content is data-driven and free of placeholder values
