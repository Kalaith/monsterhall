# Monsterhall GDD Transition Plan

This plan transitions the current project from its old adult-management framing into the Monsterhall GDD direction: a warm, dangerous guild-management game where monster companions make tower exploration practical.

## Current Baseline

The existing game already has useful scaffolding:

- Day-based management loop with assignment, recovery, expeditions, results, debt, and upkeep.
- Data-driven catalogs for species, traits, buildings, rooms, requests, floors, missions, events, UI text, and config.
- Screens for town overview, town management, roster/profile, hatch reveal, chamber management, guest management, expedition planning, journal, and day results.
- Runtime state for resources, roster, eggs, buildings, unlocked floors/species, active requests, active expedition, debt, and event log.

The largest mismatch is not engine structure. It is domain language and what each system represents.

## Target System Mapping

Use this mapping to avoid unnecessary rewrites:

- `MonsterGirlState` becomes companion/monsterfolk roster state.
- `GuildJobs` remains the Guild Hall Jobs surface.
- `GuildRoomData` becomes Guild Room / Job Station data.
- `GuestManagement` becomes Contract Desk.
- `GuestRequestState` becomes Contract Request state.
- `ClientTierData` becomes Patron / Reputation Tier data.
- `ChamberManagement` becomes Hatchery / Nursery.
- `arcane_residue` becomes `arcane_residue` or `essence`.
- `companion_skills` become companion skills: scouting, guarding, recovery, hospitality, bargaining, crafting, charm, navigation, arcana, strength.
- `work_history` becomes work history or contract experience.
- `corruption` becomes instability, corruption, or tower exposure depending on final tone.
- `GuildJob` becomes TownJob or GuildJob assignment.
- `Curiosity` expedition priority becomes Curiosity / Discovery or Recovery-Focused.

## Phase 1: Product Identity and Copy Cleanup

Goal: make the game read as Monsterhall everywhere without changing behavior.

- Update README, UI text, opening story, docs, image requirements, and style docs to match the GDD.
- Replace adult-business framing with guild hall, companion, contract, tower-support, and workplace language.
- Rename screen labels first: GuildJobs Desk to Guild Hall, Guest Desk to Contract Desk, Chamber to Hatchery.
- Rename resources in UI from residue Essence to Arcane Residue or Essence.
- Rewrite onboarding text around starter companions, first contract, first guild room, and first expedition support.
- Keep old internal type names temporarily where changing them would create save or validation risk.

Acceptance target:

- A new player should not see old adult-business framing in normal gameplay UI.
- Existing loop should still be playable with the new names.

## Phase 2: Data Catalog Retheme

Goal: convert content data before deeper code changes.

- Rewrite `assets/data/species.json` around starter species from the GDD: slime, lamia, minotaur, harpy, imp, and later golemkin.
- Replace room data with guild rooms: Common Room, Reception Hall, Slime Bar, Nursery, Forge Corner, Alchemy Bench, Recovery Baths, Archive Desk, Watch Post, Route Map Chamber, Packroom, Beast Kennel.
- Replace guest archetypes with patrons: scholars, caravan leaders, local adventurers, traders, nobles, tower researchers, nervous delvers.
- Replace guest requests with contracts: escort preparation, supply packing, alchemical gathering, rare ingredient recovery, lodging support, tower guide services, archive research, beast handling, scouting commissions.
- Rewrite mission data to match GDD mission types: Resource Run, Scout Route, Egg Hunt, Relic Recovery, Contract Fulfilment, Rescue / Retrieval.
- Rewrite events so they express guild operations, tower hazards, staff incidents, local politics, and companion personality.
- Update building unlocks so rooms, jobs, species, and floors support the new progression.

Acceptance target:

- Core data loads with no schema changes.
- Every visible activity has a GDD-compatible explanation.

## Phase 3: Domain Model Renames

Goal: make code ownership match the new game so future work stops fighting old terminology.

- Rename state and data types in focused slices, keeping save migration in mind.
- Start with low-risk UI state names for guild jobs, guest management, and chamber management.
- Then rename data catalogs: `GuildRoomData` to `GuildRoomData`, `GuestRequestData` to `ContractRequestData`, and `ClientTierData` to `PatronTierData`.
- Then rename runtime state: `active_guest_requests` to `active_contracts`, `client_tiers` to `patron_tiers`, `guild_job_worker_limit` to `town_job_limit`.
- Add serde aliases or save migration support where persisted field names change.
- Keep module files under 800 lines while renaming; split by screen or data responsibility if a touched file is already too large.

Acceptance target:

- New code can use GDD language naturally.
- Existing saves either migrate cleanly or fail with a clear intentional version boundary.

## Phase 4: Companion Skill Conversion

Goal: make species matter through practical guild and tower utility.

- Replace `CompanionSkillState` with a general `CompanionSkillState`.
- Recommended initial skills: scouting, guarding, recovery, hospitality, bargaining, crafting, charm, navigation, arcana, strength.
- Replace history counters with role experience or job history: reception_jobs, support_jobs, guard_jobs, scouting_runs, contracts_completed, expeditions_supported.
- Update eligibility scoring so contracts ask for species, traits, and skill thresholds from the new skill set.
- Update town job scoring so each companion has clear best uses.
- Update profile UI to show role fit, readiness, and best next use as first-class information.

Acceptance target:

- Slime, lamia, minotaur, harpy, and imp each have distinct contract and expedition value.
- The profile screen explains why a companion is useful without relying on old categories.

## Phase 5: Town Jobs and Guild Services

Goal: turn passive room assignment into the GDD's town-work economy.

- Replace a single Guild Jobs shift concept with explicit town jobs.
- Add job definitions as data, either by evolving room data or creating a new `town_jobs.json`.
- Initial jobs should include bar staff, reception desk, porter support, training assistant, recovery room staff, alchemy support, forge helper, archive assistant, stable / packroom support, and contract runner.
- Let rooms unlock jobs and modify job output.
- Make town jobs generate gold, materials, contract bonuses, stress/fatigue recovery, expedition preparation quality, or reputation.
- Keep assignment UI simple at first: one screen, selected job, eligible companions, projected outcome.

Acceptance target:

- Companions who are not sent into the tower still matter every day.
- The player has a real reason to balance town work, rest, contracts, and expeditions.

## Phase 6: Contract Desk Upgrade

Goal: make contracts the short-term priority engine described in the GDD.

- Convert guest requests into contract requests with deadlines, species preferences, skill needs, reward, penalty, and optional patron tier.
- Add contract categories from the GDD.
- Add partial success rules based on preparation quality and companion fit.
- Add follow-up contracts for patrons when the player performs well.
- Surface contract pressure on Town Overview as an operational priority, not a guest-booking panel.
- Add contract fulfilment as an expedition mission objective where relevant.

Acceptance target:

- Contracts create daily tradeoffs and reward roster diversity.
- Contract UI can answer: what is needed, who is best, what happens if ignored, and what reward is likely.

## Phase 7: Expedition Risk and Tower Identity

Goal: make the tower feel dangerous and specific, not just a reward generator.

- Replace old priority set with Safe, Balanced, Aggressive, Recovery-Focused, and Curiosity / Discovery.
- Add floor hazard tags that interact with species and skills.
- Add preparation quality as a derived score from buildings, town jobs, contracts, and assigned companions.
- Make incorrect teams increase fatigue, injury, stress, lost time, failed objectives, or instability.
- Add floor notes and discoveries to the journal.
- Add rescue/retrieval and contract fulfilment missions once contracts exist.

Acceptance target:

- Expedition planning should feel like building a support plan for a dangerous place.
- Success should depend on team fit and preparation, not only raw stats.

## Phase 8: Hatchery and Recruitment

Goal: align roster growth with eggs, rescues, applicants, and faction rewards.

- Retheme chamber state and UI as Hatchery / Nursery.
- Keep egg inventory and hatching mechanics, but rewrite copy around care, incubation, rescue, and recruitment.
- Add non-egg recruitment sources after contracts and expeditions support them.
- Add species unlock pacing: early dependable species, later specialized companions.
- Add anticipation choices, such as focusing incubation toward utility, resilience, or social traits.

Acceptance target:

- Roster expansion feels like guild growth, not only resource conversion.
- New species unlocks create new solutions to town and tower problems.

## Phase 9: Economy and Pressure Rebalance

Goal: make debt and upkeep support the GDD's survival/growth tension.

- Rename and rebalance upkeep categories around wages, food, supplies, repairs, recovery, and facility upkeep.
- Keep debt as the campaign pressure spine, but rewrite milestones around founding loans, creditor pressure, permits, or local politics.
- Tune income so no single activity dominates: contracts, town jobs, and expeditions should all matter.
- Rework resource sinks: construction, recovery, hatchery care, expedition preparation, research, and special projects.
- Use existing simulation tests as the balance harness after the system names settle.

Acceptance target:

- The player asks the intended GDD questions: build now, rest now, take stable contract money, or risk a deeper run.

## Phase 10: Presentation and Art Direction

Goal: make the UI and assets support the new tone.

- Update backdrop prompts and generated assets from adult keep/Guild Jobs/chamber imagery to guild hall, contract desk, hatchery, tower route room, and companion workplace scenes.
- Update icon atlas names and meanings: contract, guild job, hatchery, residue, readiness, instability, patron, scouting, guarding, recovery, arcana.
- Update UI style docs around warm workplace fantasy plus credible tower danger.
- Keep screen layouts where useful, but rename navigation and panels to the GDD's major areas.

Acceptance target:

- First screenshots should sell cozy guild logistics plus dangerous tower mystery.

## Recommended Implementation Order

1. Copy and UI terminology pass.
2. Data retheme using existing schemas.
3. Screen/module rename pass.
4. Companion skill schema conversion.
5. Town jobs expansion.
6. Contract desk upgrade.
7. Expedition risk upgrade.
8. Hatchery/recruitment expansion.
9. Economy rebalance.
10. Art and presentation refresh.

## Validation Strategy

- Use `.\publish.ps1` from the project root after each meaningful code or data phase.
- Add targeted data validation for new IDs after catalog rethemes.
- Keep existing simulation validation, but rename reports and metrics once domain names change.
- For save-breaking schema changes, either add migration tests or intentionally bump save version and document the break.
- For UI phases, run the existing screenshot/playtest scripts only when the target screens are expected to be stable.

## First Work Slice

The best first implementation slice is a behavior-preserving retheme:

- Update README and docs.
- Update `assets/data/ui_text.json`.
- Update `assets/data/story_events.json`.
- Update visible navigation labels.
- Update resource label from residue Essence to Arcane Residue / Essence.
- Update expedition priority label from Curiosity to Curiosity / Discovery.
- Leave internal Rust type names alone in this slice.

This gives the project the new identity quickly while keeping the deeper model changes isolated for later phases.
