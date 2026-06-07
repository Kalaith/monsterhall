# Complete Game Plan: Monsterhall

## Summary
This document defines the direction for taking **Monsterhall** from a functional MVP into a complete game. The MVP proved the town, brothel, expedition, save/load, and data-driven runtime structure. The next step is to turn that structure into a real game with a stronger fantasy, a clearer progression spine, and content that escalates for reasons the player can feel.

This plan supersedes the old “light narrative, systems-first” framing in the current GDD. The game is still a management sim, but now the management loop is anchored by a stronger premise:

- the tower is seen as worthless and lethal
- the protagonist is desperate and broke
- the tower contains a hidden reproductive/incubation mechanism
- monster girls are the tower's only meaningful commercial output
- sexual growth, debt pressure, and guest demand are the real progression engines

The goal is not to replace the current Rust architecture. The current state machine, JSON-first content structure, and Macroquad runtime approach still fit. What changes is the design target for the content and systems built on top of that runtime.

## High Concept
The protagonist is a down-on-his-luck man with nowhere else to go. He camps at a tower everyone else considers a death trap and a dead end. No one brings back treasure, no one establishes a business there, and the local assumption is simple: enter the tower, die, or come back empty.

By accident, desperation, curiosity, or reckless lust, he discovers a wet opening in the tower and ejaculates into it. In response, the tower produces an egg. That egg hatches into the first slime girl: timid, affectionate, eager to please, and immediately valuable.

From that moment, the game’s real fantasy becomes clear:

- the tower can produce eggs
- the hidden incubation chamber can convert selected eggs into loyal monster girls
- those girls can be trained through sex work and conditioning
- deeper tower access creates rarer girls, rarer services, and richer clients
- debt and guest expectations force the player to expand beyond the comfort of early slime-girl income

The player is building more than a brothel. He is exploiting a forbidden reproductive mechanism and building a debt-fueled erotic business on top of a place everyone else dismissed as useless.

## Narrative Premise

### The Protagonist
The protagonist is not a hero, warrior, or dungeon conqueror. He is a desperate opportunist with enough instinct to notice what everyone else missed and enough shamelessness to exploit it. He is mechanically important, but he is not a front-line combatant.

He matters because he:

- activates the chamber
- imprints monster girls through incubation
- negotiates with creditors and special guests
- decides which eggs are worth converting through the chamber
- controls expansion and risk

He does **not** personally raid the tower as a fighter.

### The Tower
The tower has a reputation for being a worthless monster den. It kills explorers, yields little obvious treasure, and has no clear path to profit. This public reputation is important because it protects the protagonist’s secret in the early game. No one expects the tower to be the foundation of a viable business.

### The Inciting Incident
The opening sequence is fixed:

1. the protagonist camps near the tower
2. he discovers the wet opening in the structure
3. he ejaculates into it
4. an egg is produced
5. the egg hatches into the first slime girl
6. he realizes she is valuable and decides to profit from her
7. he must build the first room for clients after she already exists

This sequence is the first chapter and the foundation of the whole game.

### Why The Chamber Matters
The chamber is not just a production device. It is the source of the protagonist’s leverage and the reason he can turn a dead tower into a business.

### Why Monster Girls Matter
Eggs only become useful to the player’s business when processed through the incubation chamber into monster girls. Those girls are:

- socially manageable
- sexually trainable
- commercially valuable
- personally loyal to the protagonist because of the chamber process

This distinction is not flavor. It is one of the game’s central rules.

## Core Progression Loop
The complete-game loop should be:

1. **Acquire eggs and resources from the tower**
   - Floor 1 mainly yields slime eggs
   - deeper floors add broader species pools and rarer tower outputs

2. **Choose what to do with each egg**
   - process it through the chamber to create a monster girl
   - preserve or process it for later systems if added

3. **Raise and train the resulting girls**
   - assign them to rooms, services, guests, conditioning, or expedition preparation
   - level them through sexual experience, not combat

4. **Expand the business**
   - build rooms for specific services
   - build support structures that improve recovery, hatchery, loyalty, and tower access
   - improve the chamber and supporting systems

5. **Meet debt and guest pressure**
   - satisfy scheduled obligations
   - unlock higher-value bookings
   - deal with clients whose demands require more specialized girls

6. **Push deeper into the tower**
   - find rarer eggs
   - unlock stronger or stranger species
   - gain corruption, relics, and advanced guest opportunities

7. **Repeat with increasing complexity**
   - more species
   - more service specialization
   - deeper tower risks
   - larger debt stakes
   - more demanding guests

This loop replaces the MVP’s “generic management loop” with a loop driven by narrative and erotic progression.

## Major Game Systems

### 1. Incubation And Egg Conversion
Egg production and girl creation must become a first-class system.

Rules:

- Tower expeditions primarily recover eggs and materials.
- Eggs do **not** naturally hatch into monster girls.
- The incubation chamber is the only reliable way to create monster girls.
- The protagonist must ejaculate into the chamber to trigger conversion.
- Chamber-created girls begin with strong loyalty or imprinting toward the protagonist.

System needs:

- explicit egg inventory/state
- incubation workflow
- chamber capacity and upgrade path
- clear chamber conversion rules

### 2. Monster Vs Monster-Girl Distinction
This only needs to exist as a foundational rule: the chamber creates the commercially and mechanically relevant outcome, which is the monster girl. The game should not spend design energy treating non-chamber hatching as a parallel progression path.

### 3. Sexual Skill Progression
The game should move away from generic level-ups and use explicit sexual experience tracks.

For clarity, monster-girl progression data should be split into four categories:

#### A. Body Stats
These describe what the girl physically is.

- height
- breast size
- body shape
- nipple sensitivity
- anatomy descriptors needed for scene logic

These are mostly persistent physical traits, though some may change through corruption, breeding, mutation, conditioning, or special scenes.

#### B. Sexual Skill Stats
These describe what the girl is trained to do well.

- scouting
- guarding
- hospitality
- crafting
- charm/presentation
- breeding aptitude

These are the main service-performance progression values and should grow through work, training, and milestone scenes.

#### C. Sexual History Counters
These describe what the girl has actually experienced over time.

- times kissed
- total sex acts
- creampies received
- masturbation scenes completed
- births given
- other scene-count stats where needed

These should be used for content gating, guest expectations, and flavor persistence. They are records of lived history, not substitutes for skill.

#### D. Relationship / Mental State Stats
These describe how the girl relates to the protagonist and how stable she currently is.

- loyalty/imprinting
- affection or attachment if added
- stress
- fatigue
- injury
- corruption

These determine obedience, exclusivity, instability, and access to certain scenes or guests.

Minimum tracks:

- `scouting`
- `guarding`
- `hospitality`
- `crafting`

Recommended support tracks:

- `charm` or `presentation`
- `endurance`
- `hatchery` or `breeding aptitude`
- `body conditioning`

The game should also track sexual history and bodily development as persistent stats, not just hidden flavor:

- number of kisses given
- number of sex acts completed
- number of creampies received
- number of masturbation scenes
- number of births
- scene counts by act type where useful

This is important for two reasons:

- certain scenes, guests, and progression beats can require a girl to have enough lived experience, not just raw skill values
- girls should feel like they have an erotic history shaped by the player’s choices

Body-specific progression should also be visible and persist over time. At minimum, the design should plan for:

- height
- breast size
- nipple sensitivity or nipple experience
- body shape or conditioning descriptors where useful

These are not only cosmetic. They can be used for:

- guest requirements
- room compatibility
- scene unlocks
- special service value
- breeding or corruption outcomes

The important implementation rule is:

- **Body stats** describe the body
- **Skill stats** describe trained capability
- **History counters** describe what has happened
- **Relationship/mental stats** describe attachment, wear, and instability

The game should not collapse these into a single generic “experience” or “level” value.

For the first full implementation pass, use a deliberately small starter field list instead of trying to support every possible erotic variable immediately.

#### Starter Body Stat Fields
- `height`
- `breast_size`
- `nipple_sensitivity`
- `body_shape`

This is enough to support:

- visual/body differentiation
- basic guest requirements
- scene gating
- future mutation/corruption changes

#### Starter Sexual Skill Fields
- `scouting`
- `guarding`
- `hospitality`
- `crafting`
- `charm`

This is enough to support:

- first room/service differentiation
- early guest specificity
- skill-driven growth that feels more granular than one level stat

`breeding_aptitude` and similar niche values can be added after the core service loop works.

#### Starter Sexual History Fields
- `kiss_count`
- `sex_count`
- `creampie_count`
- `masturbation_count`
- `birth_count`

This is enough to support:

- “experienced enough for this scene” checks
- flavor persistence
- guest requirements based on actual use history

Act-specific counters beyond this should only be added when content actually needs them.

#### Starter Relationship / Mental Fields
- `loyalty`
- `stress`
- `fatigue`
- `injury`
- `corruption`

This is enough to support:

- chamber imprinting
- overwork and recovery
- corruption progression
- guest and scene gating based on trust or instability

`affection`, jealousy, or more detailed emotional fields should remain future expansions unless content immediately depends on them.

The preferred implementation order is:

1. relationship/mental stats
2. sexual skill stats
3. sexual history counters
4. body stats

This order keeps the first playable version focused on progression and gating before richer body-specific content is layered on top.

Rules:

- girls level primarily through sexual work, scene participation, special guests, and directed training
- different rooms and clients grow different tracks
- track thresholds gate room usage, guest satisfaction, and special services
- high-value clients ask for exact competencies, not just generic stats
- some scenes should additionally require minimum history counts or body-state requirements

### 4. Body Conditioning And Tower Readiness
Sexual skill is not enough. Girls must also be physically prepared for deeper tower exposure.

This system should cover:

- strain tolerance
- flexibility or body adaptation
- corruption tolerance
- depth readiness

The purpose is to connect brothel growth and tower progress:

- sexual training makes girls better workers
- body conditioning makes them viable for deeper expeditions and rougher clients

### 5. Loyalty And Imprinting
Chamber-born girls begin with strong attachment to the protagonist. This should affect:

- obedience
- scene availability
- resistance to outside influence
- special services involving the protagonist
- guest jealousy or “exclusive” bookings

Loyalty should be a real system, not just flavor text. It can gate content and create strategic tension:

- high loyalty is useful but can limit certain guest arrangements
- low loyalty or overwork can create instability

Scene design should be allowed to gate on combinations such as:

- loyalty threshold
- scouting or act experience threshold
- body metric threshold
- corruption threshold
- species or trait requirement

### 6. GuildJobs Growth And Service Tiers
The brothel must evolve from one improvised room into a structured erotic business.

Progression should include:

- starter private room
- basic room differentiation
- specialized fetish/service rooms
- elite suites for high-paying or unusual guests
- service categories tied to specific sexual skill tracks

The room plan should answer:

- what acts are possible here
- what skills are trained here
- what guests are allowed here
- what species/traits perform best here

### 7. Expeditions
Expeditions remain mission-based but become more tightly connected to breeding and business growth.

Expeditions should primarily support:

- egg retrieval
- tower materials
- relics
- corruption exposure
- species access

They are not the primary source of girl leveling. They are the source of future girls, future danger, and future specialization.

### 8. Corruption And Mutation
Corruption should become a major mid-game and late-game growth layer.

Corruption can:

- alter species
- unlock advanced service compatibility
- affect guest demand
- trigger event chains
- enable stronger incubations or rarer outcomes

Mutation should remain data-driven and tied to:

- floor depth
- trait prerequisites
- corruption thresholds
- chamber choices
- special guest outcomes

### 9. Debt System
Debt is the game’s soft pressure system.

It should:

- create deadlines and escalating expectations
- push the player to grow beyond safe slime-girl income
- unlock new narrative beats and guest types
- punish stagnation without producing frequent hard fail states

Consequences of missing obligations should be things like:

- penalties
- loss of reputation
- seized income
- blocked upgrades
- harsher guest terms
- more aggressive creditor oversight

Debt should not usually delete the run outright.

### 10. Guest System
Guests should become a major content and progression pillar.

Guest classes:

- local basics
- repeat clients
- wealthy elites
- fetish specialists
- creditors and their agents
- monster or non-human VIPs
- special story guests

Guests should require combinations of:

- species
- room type
- service type
- skill thresholds
- corruption state
- kink tags
- loyalty state

This is what forces roster diversification.

## Roster And Species Roadmap
The complete game should scale species access by tower depth and business sophistication.

### Early Game
- `slime_girl`
- early slime variants
- soft introduction to loyalty, basic services, and chamber logic

### Mid Game
- more humanoid or commercially flexible girls
- girls suited to performance, breeding, or rougher specialty scenes
- species that broaden guest compatibility

### Late Game
- rarer and stranger girls
- high-corruption or high-loyalty transformations
- species linked to elite guests and dangerous tower zones

The current working direction should support at least:

- early slime core
- mid-game diversification through goblin, harpy, lamia, hellhound-style species
- late-game corruption/ritual species such as priestess, nymph, and rarer tower-born variants

Species unlocks must come from a combination of:

- tower floor access
- chamber usage
- debt progression
- guest progression
- mutation paths

## Guest And Debt Progression

### Common Guests
These stabilize the early game and establish the first service economy.

They pay modestly and mainly teach:

- what rooms matter
- what acts matter
- what traits improve income

### Debt Collectors And Creditors
These provide recurring pressure. They should introduce:

- repayment windows
- special penalties
- humiliating or compromising fallback arrangements
- narrative escalation if the player underperforms

### High-Value Special Guests
These are the main reason “one slime girl forever” stops being viable.

They should ask for:

- precise acts
- precise traits
- specific species
- specific corruption or loyalty states
- advanced room types

### Progression Intent
The business should evolve like this:

1. make money from basic services
2. repay enough debt to survive
3. realize guest complexity outpaces simple slime labor
4. go deeper for better eggs
5. train specialists
6. satisfy rarer guests
7. leverage the chamber for long-term dominance

## Content Production Plan

### Must-Have For The Complete Game
- opening story chapter and setup events
- debt milestone chain
- chamber discovery beats
- first-room/first-client sequence
- defined sexual skill tracks and training content
- room/service progression by act type
- guest archetype library
- special guest request system
- expanded species ladder
- egg conversion rules and chamber workflow
- deeper tower species table
- corruption and mutation event writing
- loyalty/imprinting event writing

### Desirable But Secondary
- more authored side scenes
- richer faction politics
- additional protagonist-specific scenes
- more elaborate chamber upgrade fiction

## Implementation Roadmap

### Phase A. Rewrite The Opening Game
Replace the current generic “start a campaign” framing with a directed first chapter:

- scripted discovery of the tower opening
- first ejaculation event
- first egg creation
- first slime girl hatch
- first room build requirement
- first paying client or first survival objective

This phase should establish the real fantasy within the first few minutes.

### Phase B. Rebuild Progression Around Sex Skills
Refactor growth so girls no longer level mainly as broad management units.

Add:

- sexual skill tracks
- conditioning/depth-readiness track
- training sources tied to services and conditioning actions
- guest requirements based on exact thresholds

### Phase C. Implement Chamber-First Egg Logic
Build the real egg pipeline:

- egg inventory and states
- chamber-assisted hatch workflow
- chamber loyalty/imprinting

### Phase D. Expand Debt And Guest Systems
Introduce:

- debt milestones
- creditor visit cadence
- guest archetypes and request templates
- guest requirements tied to service, species, and skill tracks
- penalties and rewards

### Phase E. Expand Species And Tower Content
Grow the roster and make deeper floors matter through:

- species tables by floor
- special egg outcomes
- mutation routes
- corruption-sensitive species
- room/species/guest synergy pass

### Phase F. Hardening And Balance
After the systems above exist:

- balance debt pacing
- balance guest payouts
- balance skill growth rates
- improve onboarding around chamber use and girl creation
- add migration/versioning for save compatibility
- add tests for egg conversion, guest gating, and debt resolution

## Future Data Model Targets
Implementation should move toward these runtime/content models.

### Monster Girl Runtime
`MonsterGirlState` should eventually include:

- species id
- body stat block
- sexual skill stat block
- sexual history counter block
- relationship/mental state block
- body-conditioning or depth-readiness value
- trait/kink tags
- current assignments

Recommended structure:

- `body_stats`
  - height
  - breast size
  - nipple sensitivity
  - other stable body descriptors
- `sex_skills`
  - scouting
  - guarding
  - hospitality
  - crafting
  - charm/presentation
  - breeding aptitude if tracked separately
- `sex_history`
  - kisses given
  - sex count
  - creampie count
  - masturbation count
  - birth count
  - other scene counters as needed
- `relationship_state`
  - loyalty/imprinting
  - stress
  - fatigue
  - injury
  - corruption

### Egg Runtime/Data
Egg data should distinguish:

- source species pool
- tower floor source
- incubation eligibility
- chamber conversion outcome
- corruption or relic modifiers

### Guest Data
Guest/request records should support:

- guest archetype
- room requirement
- service/act requirement
- sexual skill thresholds
- sexual history thresholds
- body metric requirements
- relationship or loyalty requirements
- species constraints
- kink/corruption constraints
- payout
- failure consequences

### Debt Data
Debt data should support:

- creditor identity
- milestone dates or cadence
- payment amount
- failure penalties
- unlock rewards
- escalation level

### Story/Event Data
Event content should cover:

- opening scenes
- chamber-related scenes
- debt scenes
- guest scenes
- corruption scenes
- mutation scenes
- loyalty scenes
- experience-gated scenes
- body-state or development-gated scenes

## Acceptance Criteria
This direction is complete when:

- the game’s fantasy is immediately understandable from the opening
- the player clearly understands why the chamber matters
- slime girls are useful but obviously not enough for long-term success
- progression is driven by sex skill growth, body readiness, debt pressure, and guest demand
- deeper tower access clearly translates into better eggs, better girls, and better business opportunities
- the protagonist feels mechanically present without becoming a combat avatar

## Relationship To Existing Docs
The existing `gdd.md` still remains useful for:

- general architecture direction
- town-around-the-brothel structure
- room/building categories
- expedition framing
- broad content boundaries

But it should no longer be treated as correct for:

- story weight
- progression philosophy
- how girls are created
- how leveling works
- why the player goes deeper
- how debt and guest pressure drive expansion

This document is the design target for the next stage of development.
