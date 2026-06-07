# UI Game Feel Review And Image Requirements

Fresh review date: 2026-04-28

Reviewed against fresh captures from the native UI harness:

- `review_suite` with the `management` save preset
- `guest_active_request` with the `management_active_guest` save preset
- manual captures for Opening, Journal, and Day Results

The current UI has a coherent dark burgundy management style. It is readable and much more unified than an early prototype, but it still feels closer to an editor/dashboard than a game because most visual surfaces are procedural placeholders, the background is reused heavily, and rewards, danger, room identity, character identity, and story beats are mostly text-led.

## Overall Direction

The target should be a game UI with operational clarity, not a decorative skin. Keep the current restrained Monsterhall Guild Hall palette, but make each screen feel like a place in the tower economy:

- Main Menu: exterior keep fantasy and campaign promise.
- Opening: illustrated story chapter beats.
- Town Overview and Journal: command table, ledger, day pressure, and next move.
- Town Management: build map, unlock web, material investment.
- Guild Hall Management: room fantasy, worker assignment, patron value.
- Contract Desk: patron contract desk, deadline pressure, candidate matching.
- Hatchery Management: occult hatchery, eggs, species outcomes.
- Expedition Planning: dangerous floor scouting and team risk.
- Monster Profile: character identity and progression.
- Day Results: payout, consequences, and event impact.
- Settings: quiet modal overlay, not a full fantasy location.

The biggest improvement will come from replacing procedural silhouettes with real game assets while keeping text and controls clean.

## Immediate UI Findings

### Cross-Screen

- The strongest issue is low content-specific art. Most pages share the same abstract tower/arch backdrop and thin geometric placeholder art, so the pages blur together.
- Current panel hierarchy is consistent, but many large panels read as static boxes. Add image-backed focal areas only where they support the decision.
- Resource, status, mission, condition, and risk information should use small icons with stable labels. This would make the UI feel more like a game without adding text.
- The footer works functionally, but all destinations have similar weight. Add current-screen state, destination icons, and a stronger end-day treatment.
- Text is generally readable, but several labels are tiny. Art should not reduce readable space further.

### Main Menu

Current read: Clear title and actions, but the centered block and placeholder illustration feel like a mockup.

Game-like upgrade:

- Replace the right placeholder with key art of the keep/tower guild at night.
- Let the background carry more mood: ruined keep silhouette, lit windows, distant tower depth, warm interior glow.
- Add subtle animated light/fog/parallax if animation budget allows.
- Keep `New Campaign` as the only dominant action.

Image needs:

- Main menu background, 16:9.
- Main menu key art or logo panel, 4:3 or 1:1.
- Optional title lockup without baked button text.

### Loading

Current read: Simple panel from code review only; loading state is transient.

Game-like upgrade:

- Use a small animated sigil, egg pulse, candle flame, or tower emblem.
- Keep the panel compact and avoid full-screen art unless load time becomes long.

Image needs:

- Loading emblem or spinner sequence.
- Optional small branded crest.

### Opening Chapter

Current read: Good chapter layout, but the story CG is still a repeated two-figure placeholder.

Game-like upgrade:

- Give each opening step a unique image so the tutorial feels authored.
- Keep CGs suggestive and atmospheric in the default management flow; avoid explicit scene detail in small UI frames.
- Use the image to show what changed: ruins, discovery, egg, first hatch, first room, first patron.

Image needs:

- 6 opening story CGs keyed by `story_events.opening_steps[*].id`:
  - `camp`
  - `discovery`
  - `incubation`
  - `hatch`
  - `build_room`
  - `first_patron`

### Town Overview

Current read: The screen answers what matters today, but it is very rectangular and list-like. The roster portraits are placeholders, so the player has little emotional attachment to the roster.

Game-like upgrade:

- Add destination icons to the priority route and footer.
- Make the town snapshot feel like a ledger strip with resource icons and debt pressure markers.
- Upgrade roster cards with actual portraits and tiny condition/status overlays.
- Add a small town/keep state visual behind or beside the priority panel only if it does not compete with the decision.

Image needs:

- Resource icons: gold, materials, eggs, relics, arcane residue.
- Debt icon and due-date warning icon.
- Roster portrait assets for every unlocked species.
- Assignment icons: idle, guild job, resting, expedition, contract.
- Footer destination icons: town, planner, guild hall, patron, hatchery, expedition, journal, end day.

### Town Management

Current read: Good selected-building layout. The building thumbnail is too symbolic and does not sell the fantasy of constructing a town.

Game-like upgrade:

- Replace each building thumbnail with a small painterly/isometric location image.
- Make locked/available/built status visible through badge/icon overlays.
- Consider a later map-style progression view, but keep the current list/detail layout until there is enough art.

Image needs:

- Building thumbnails for every `buildings.json` entry:
  - `slime_pool`
  - `warm_love_nest`
  - `residue_alchemy_bench`
  - `silk_rope_forge`
  - `healing_hot_springs`
  - `aftercare_lounge`
  - `hatchery_scrying_pool`
  - `monster_kink_archive`
  - `tower_charm_cartography`
  - `luxury_room_renovation`
  - `relic_residue_condenser`
  - `prestige_hospitality_wing`
- Building category icons: habitat, workshop, recovery, research, prestige, project.
- Availability overlay icons: available, built out, locked by cost.

### Guild Hall Management

Current read: The screen is functional and closer to a management game, but the room fantasy is underpowered. Room art, worker portraits, and payout icons need to do more.

Game-like upgrade:

- Replace room thumbnails with distinct room images.
- Add room service-tier emblem and trained-skill icons.
- Make assigned and available workers feel like character cards, not generic rows.
- Add a small projected payout visual using gold/residue icons.

Image needs:

- Room thumbnails for every `guild_rooms.json` entry:
  - `vanilla_suite`
  - `packroom_annex`
  - `nursery_wing`
  - `public_stage`
- Skill icons:
  - scouting
  - guarding
  - hospitality
  - crafting
  - charm
- History icons:
  - scouting count
  - guarding count
  - hospitality count
  - crafting count
  - contract count
  - support count
  - rescue count
- Worker card portrait assets shared with roster/profile.

### Contract Desk

Current read: Empty state is clear. Active request state has good information density, but the patron, room, deadline, and candidate fit are mostly text boxes.

Game-like upgrade:

- Give patron archetypes recognizable patron portraits or silhouettes.
- Use contract-style request cards with reward, deadline, room, and requirement icons.
- Candidate cards should show pass/fail badges more visually, especially blocked reasons.
- The selected request should feel like a patron contract, not another data panel.

Image needs:

- Patron archetype portraits for every `guest_archetypes.json` entry:
  - `curious_local`
  - `road_mercenary`
  - `veiled_patron`
- Optional request-specific emblems for every `guest_requests.json` entry:
  - `starter_slime_bedding`
  - `repeat_patron_confidence`
  - `quiet_room_return`
  - `mercenary_stage_tease`
  - `succu_salon_booking`
  - `goblin_service_loop`
  - `harpy_headliner_show`
  - `hellhound_guard_shift`
  - `lamia_binding_rite`
  - `sealed_vault_handling`
  - `moth_relic_vigil`
  - `veiled_suite_audience`
- Deadline, reward, penalty, accepted, blocked, and assigned icons.

### Hatchery Management

Current read: Stronger thematically than most screens because eggs and hatching are naturally visual. The current egg thumbnails are readable, but the hatchery still lacks occult machinery and species anticipation.

Game-like upgrade:

- Use distinct egg art by grade, floor origin, and prepared outcome.
- Give the selected egg a larger, richer egg image with glow, fluid, sigils, or crack state.
- Outcome cards should include species portraits or hatch preview silhouettes.
- The hatchery status strip should feel like apparatus status, not a normal form row.

Image needs:

- Egg thumbnails by grade:
  - origin/starter
  - common
  - unusual
  - rare
  - deepborn
- Egg state overlays:
  - raw
  - reviewed
  - prepared
  - hatchable
  - hatched/resolved, if shown later
- Species outcome mini portraits for all species.
- Hatchery apparatus background or selected-egg frame.

### Expedition Planning

Current read: Clear structure, but the danger and floor identity are too abstract. The floor preview is currently a simple glyph strip.

Game-like upgrade:

- Replace floor preview with a moody floor scene.
- Add mission icons and priority stance icons.
- Turn success/risk/reward into a scouting report with icons and meter styling.
- Add team readiness overlays on character cards.

Image needs:

- Floor previews for every `floors.json` entry:
  - `floor_1_slick_cellars`
  - `floor_2_molten_baths`
  - `floor_3_gilded_kennels`
  - `floor_4_heart_vault`
- Mission icons:
  - `resource_run`
  - `egg_hunt`
  - `relic_raid`
  - `instability_dive`
- Priority icons:
  - balanced
  - aggressive
  - safe
  - curiosity
- Risk, success, injury, materials, residue, eggs, and relic result icons.

### Monster Profile

Current read: The profile has the right information, but the current portrait panel is too generic to support attachment or progression.

Game-like upgrade:

- Use a large character portrait for the selected monster.
- Show condition with icon badges and color, not only letters.
- Add trait icons with tooltips or selected trait detail later.
- Consider alternate portrait treatments for mutation, instability, fatigue, injury, or job assignment once base portraits exist.

Image needs:

- Species portraits keyed by `species.portrait_key`:
  - `slime_girl_portrait`
  - `succu_slime_portrait`
  - `goblin_maid_portrait`
  - `golemkin_warden_portrait`
  - `harpy_hostess_portrait`
  - `lamia_binder_portrait`
  - `hellhound_bouncer_portrait`
  - `moth_priestess_portrait`
- Trait icons keyed by `traits.icon_key`:
  - `stretch`
  - `spark`
  - `ribbon`
  - `whip`
  - `nest`
  - `veil`
  - `feather`
  - `coin`
  - `shield`
  - `fang`
- Stat icons: power, charm, endurance, instinct.
- Condition icons: fatigue, stress, injury, instability.

### Journal

Current read: The Journal is useful and readable, but it feels like a help screen plus log. It needs a campaign-record identity.

Game-like upgrade:

- Add ledger/parchment/notebook styling inside the current palette.
- Use a priority icon and route icon in the current-priority panel.
- Use category icons in the campaign log when event data exposes category.
- Keep the text layout clean; this screen should not become an illustrated story page.

Image needs:

- Journal or ledger background texture, subtle and tile-safe.
- Priority/destination icons shared with the footer.
- Event category icons:
  - guild hall
  - instability
  - expedition
  - special
  - town

### Day Results

Current read: The page communicates the day, but it has one concrete layout bug: event-log text can run past the right edge of the panel instead of wrapping/clipping. The page also lacks reward impact.

Game-like upgrade:

- Fix event-log wrapping first.
- Replace plain lines with result cards that use reward, debt, patron, roster, and event icons.
- Make positive and negative changes visually distinct.
- Add a small daily stamp/banner, such as paid, shortfall, special event, new egg, or debt pressure.

Image needs:

- Result category icons: guild hall, expedition, debt, contracts, roster, event log.
- Delta icons: gained, spent, warning, failed, completed.
- Special-event frame or banner treatment.
- Optional small event illustrations only for major/special events.

### Settings

Current read: Functional modal. It should stay quiet and not become heavily illustrated.

Game-like upgrade:

- Add a small settings cog or crest.
- Strengthen modal dimming and preserve readability over the background.
- Keep save/close/quit visually separated.

Image needs:

- Settings cog icon.
- Save icon.
- Close icon.
- Quit/danger icon.

## Required Asset Classes

### Backdrops

Backdrops replace or augment `BackdropKind` in `src/ui/art.rs`.

Required keys:

- `main_menu`
- `opening`
- `town`
- `town_management`
- `hatchery`
- `guild hall`
- `guest_desk`
- `expedition`
- `profile`
- `results`
- `settings`

Recommended source size:

- 2560 x 1440 PNG or WebP source.
- Must crop safely to 16:9 at 1280 x 720, 1600 x 900, and 1920 x 1080.
- No baked text.
- Keep the central UI-safe region low contrast: approximately x 18% to 82%, y 12% to 88%.
- Strongest detail should live near edges or behind intentional art frames.

### Character Portraits

Required for every species, using existing `portrait_key` values.

Recommended source size:

- 768 x 1024 or 1024 x 1365, 3:4 portrait.
- Transparent PNG preferred for character-only portraits.
- Provide enough padding around head, hair, wings, tail, horns, and other silhouette features.
- Must read at small card sizes around 58 x 72 and profile sizes around 260 x 320.
- No baked name labels.

Minimum variants:

- One neutral portrait per species.

Future variants:

- work
- rest
- expedition
- injured
- high instability
- mutation transition

### Room, Building, And Floor Thumbnails

Required for all current room, building, and floor IDs.

Recommended source size:

- 1024 x 768, 4:3 source.
- Also crop-safe at 16:9 for floor previews.
- No baked text.
- Readable at 140 x 112 and 220 x 120.
- Strong silhouette and color identity per item.

### Story CGs

Required for the 6 opening step IDs.

Recommended source size:

- 1600 x 1200 or 1920 x 1440, 4:3.
- Should crop down cleanly into the current opening frame.
- No baked text.
- Keep explicit detail out of small default UI images; communicate scene and consequence.

### Patron Art

Required minimum:

- One portrait/silhouette per patron archetype.

Optional expanded set:

- One emblem per contract.
- One full request illustration for special requests only.

Recommended source size:

- Archetype portrait: 768 x 1024 transparent PNG.
- Request emblem: 512 x 512 transparent PNG.
- Special request illustration: 1600 x 1200.

### Eggs

Required minimum:

- 5 egg-grade images: origin, common, unusual, rare, deepborn.
- 4 overlays: raw, reviewed, prepared, hatchable.

Recommended source size:

- Egg base: 512 x 512 transparent PNG.
- Overlay: 512 x 512 transparent PNG.
- Selected egg art can use 1024 x 1024 for richer detail.

### Icons

Recommended source size:

- Master: 256 x 256 transparent PNG.
- Runtime display: 16, 20, 24, 32, and 48 px.
- Use a consistent stroke/fill language.
- Icons must remain legible on `panel_0`, `panel_1`, and `panel_2`.

Required icon groups:

- Resources: gold, materials, eggs, relics, arcane residue.
- Stats: power, charm, endurance, instinct.
- Conditions: fatigue, stress, injury, instability.
- Assignments: idle, guild job, resting, expedition, contract.
- Companion skills: scouting, guarding, hospitality, crafting, charm.
- History: scouting, guarding, hospitality, crafting, contract, support, rescue.
- Missions: resource run, egg hunt, relic raid, instability dive.
- Priorities: balanced, aggressive, safe, curiosity.
- Status: available, built out, locked, assigned, blocked, eligible, accepted, completed, failed, warning.
- Navigation: town, planner, guild hall, patron, hatchery, expedition, journal, settings, save, close, quit, end day.
- Event categories: guild hall, instability, expedition, special, town, debt, contracts, roster.

### UI Chrome

Optional but useful after primary art exists:

- Panel frame pieces.
- Selected card frame.
- Gold divider.
- Warning/danger frame.
- Button fills or subtle texture overlays.
- Meter fills for success, risk, debt, fatigue, stress, injury, instability.

Chrome art must not bake text and must scale without distortion.

## Suggested File Layout

The project currently has JSON data under `assets/data/` and procedural art in `src/ui/art.rs`. There is no current texture loading path for these image assets.

Recommended layout:

```text
assets/
  images/
    backdrops/
      main_menu.png
      opening.png
      town.png
      town_management.png
      hatchery.png
      guild hall.png
      guest_desk.png
      expedition.png
      profile.png
      results.png
      settings.png
    story/
      camp.png
      discovery.png
      incubation.png
      hatch.png
      build_room.png
      first_patron.png
    species/
      slime_girl_portrait.png
      succu_slime_portrait.png
      goblin_maid_portrait.png
      golemkin_warden_portrait.png
      harpy_hostess_portrait.png
      lamia_binder_portrait.png
      hellhound_bouncer_portrait.png
      moth_priestess_portrait.png
    rooms/
      vanilla_suite.png
      packroom_annex.png
      nursery_wing.png
      public_stage.png
    buildings/
      slime_pool.png
      warm_love_nest.png
      residue_alchemy_bench.png
      silk_rope_forge.png
      healing_hot_springs.png
      aftercare_lounge.png
      hatchery_scrying_pool.png
      monster_kink_archive.png
      tower_charm_cartography.png
      luxury_room_renovation.png
      relic_residue_condenser.png
      prestige_hospitality_wing.png
    floors/
      floor_1_slick_cellars.png
      floor_2_molten_baths.png
      floor_3_gilded_kennels.png
      floor_4_heart_vault.png
    contracts/
      curious_local.png
      road_mercenary.png
      veiled_patron.png
    eggs/
      grade_origin.png
      grade_common.png
      grade_unusual.png
      grade_rare.png
      grade_deepborn.png
      overlay_raw.png
      overlay_reviewed.png
      overlay_prepared.png
      overlay_hatchable.png
    icons/
      resources/
      stats/
      conditions/
      assignments/
      skills/
      history/
      missions/
      priorities/
      status/
      navigation/
      events/
```

## Implementation Requirements

Before art assets can replace placeholders, the code needs a small art pipeline:

1. Add an async image loader/cache during the loading phase.
2. Define a manifest or deterministic path lookup for all asset keys.
3. Keep procedural placeholder drawing as fallback when an asset is missing.
4. Add image keys to data where they do not exist yet:
   - buildings
   - rooms
   - floors
   - patron archetypes
   - contracts, if request-specific emblems are used
   - debt milestones, if illustrated
5. Update `src/ui/art.rs` draw functions to prefer textures:
   - `draw_backdrop`
   - `draw_story_cg_placeholder`
   - `draw_species_portrait`
   - `draw_guest_silhouette`
   - `draw_room_thumbnail`
   - `draw_floor_preview`
   - `draw_building_thumbnail`
   - `draw_egg_thumbnail`
   - `draw_trait_icons`
6. Run the screenshot suite at 1280 x 720, then spot-check 1600 x 900 and 1920 x 1080.

## Visual Acceptance Criteria

- Every image must still work under the current dark panel system.
- No image may contain baked UI labels, buttons, numbers, or resource amounts.
- Character, patron, and story art should fit the warm guild-management tone and avoid sexualized framing.
- Default management UI art should be suggestive and atmospheric rather than explicit.
- Image focal points must not sit under title bars, footers, or dense text panels.
- Text must remain readable against every backdrop.
- Small icons must be distinguishable at 16 px and 24 px.
- Portraits must preserve identity at both roster-card size and profile size.
- Thumbnails must communicate item identity without needing the item name.
- Missing assets must degrade to the existing procedural placeholder, not crash the UI.

## Priority Order

1. Fix Day Results event-log wrapping.
2. Add icon set for resources, assignments, conditions, stats, navigation, and status.
3. Add species portraits for all 8 current species.
4. Add room, building, and floor thumbnails.
5. Add screen backdrops keyed to `BackdropKind`.
6. Add opening story CGs.
7. Add patron archetype portraits and request emblems.
8. Add egg grade/state art.
9. Add optional chrome textures and special-event illustrations.

This order gives the biggest game-feel gain first while reducing the chance that art work forces layout churn.
