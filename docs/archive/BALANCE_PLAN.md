# Balance Plan

This plan is focused on turning the current 30/90/180/365-day balance into a longer campaign arc where growth stays meaningful, money has pressure after the opening month, and the player can see major cost changes before they land.

## Current Targets

- [x] Keep the 30-day average near 6 girls.
- [x] Make 90 days show steady growth without exhausting the economy.
- [x] Make 180 days reach more than 9 girls in normal play.
- [x] Make 365 days viable as the debt-deadline pacing target.
- [x] Preserve a hard population cap of 20 girls.
- [x] Keep high-tier housing/building upgrades valuable because they unlock a large profit increase.

## Measurement First

- [x] Add report metrics for guest requests generated, completed, expired, and rejected.
- [x] Split guest request metrics by tier.
- [x] Add report metrics for active request pressure by day.
- [x] Add report metrics for expedition attempts, expedition days, egg-focused missions, and egg rewards.
- [x] Add report metrics for expedition opportunity cost: unavailable girls, missed guests, and prep/upkeep cost.
- [x] Split current upkeep reporting into food, cleaning, and maintenance categories.
- [x] Add day 30, 90, 180, and 365 snapshots for roster size, cap, room capacity, reputation, debt, resources, eggs, and buildings.
- [x] Add surplus/shortfall summaries for gold, food, materials, lust, relics, and eggs.
- [x] Add regression assertions for the desired 90/180/365-day population bands, including 365 days reaching the 20-girl cap.

## Hatch Pacing

- [x] Reduce explosive first-week acquisition by adding a small early hatch throttle.
- [x] Keep egg discovery better than generic resource expeditions when the roster is below demand.
- [x] Make midgame acquisition steadier through egg-discovery buildings, mission tiering, or reputation-gated egg opportunities.
- [x] Avoid arbitrary roster-count rules; use demand, capacity, debt pressure, and resource state instead.
- [x] Add a simulation check that the roster does not stall or shrink unexpectedly after the early game.
- [x] Add a simulation check that egg inventory does not pile up because hatch costs are too high.

## Upkeep And Forecasting

- [x] Replace sharp upkeep jumps with scaling bands.
- [x] Use girls as the main food drain.
- [x] Use buildings as the main cleaning and maintenance drain.
- [x] Add higher upkeep bands at key roster or reputation thresholds.
- [x] Forecast the next upkeep band before it starts.
- [x] Show upcoming upkeep increases in the UI before the player commits to a hatch, building, or reputation increase.
- [x] Add report metrics for forecasted upkeep versus actual upkeep.

## Guest Demand Scaling

- [x] Scale request volume from roster size, room capacity, reputation, and unlocked buildings.
- [x] Increase active request pressure as the player grows.
- [x] Add stricter expiration pressure at higher tiers.
- [x] Add richer tier demand variety so late-game requests are not only bigger versions of early requests.
- [x] Make guest demand create meaningful tension with expeditions.
- [x] Add report metrics for patron happiness or equivalent satisfaction pressure.
- [x] Tune so the player cannot ignore guest demand without visible economic or reputation consequences.

## Expedition Opportunity Cost

- [x] Add expedition prep costs that scale by mission tier.
- [x] Add fatigue or cooldown after expeditions.
- [x] Add injury or recovery risk for higher-tier expeditions.
- [x] Add roster lock-in so expedition decisions have a visible scheduling cost.
- [x] Ensure expedition costs are not so punishing that egg hunting becomes a trap.
- [x] Add simulation checks for expedition cadence at 30, 90, 180, and 365 days.

## Late-Game Sinks

- [x] Add scaling upkeep bands that become meaningful after the early game.
- [x] Add expensive upgrades for high-tier buildings.
- [x] Add relic, lust, and material conversion projects.
- [x] Add luxury room maintenance or prestige systems.
- [x] Add optional projects that convert surplus into long-term value instead of immediate roster growth.
- [x] Add expensive quality-of-life upgrades that compete with hatching and housing.
- [x] Tune sinks so late-game wealth is pressured without blocking normal debt payments.

## Building Progression

- [x] Keep tier 1 building costs starting around 500 gold.
- [x] Keep higher-tier costs rounded to clear 50-value steps.
- [x] Add more buildings that improve egg discovery and midgame acquisition cadence.
- [x] Add buildings that unlock higher population cap bands toward the hard cap of 20.
- [x] Add buildings that improve profit enough to justify their higher cost.
- [x] Add buildings that increase cleaning and maintenance burden.
- [x] Validate that high-tier buildings create a large enough profit step to feel worth saving for.
- [x] Add a Town Management Projects grouping so repeatable late-game sinks are visible and easier to evaluate.

## Event Pressure

- [x] Keep special events building-gated so events match the player's current tier.
- [x] Keep event count restrained so they feel notable.
- [x] Add beneficial events that create short-term opportunities.
- [x] Add expensive events that drain gold, materials, lust, relics, or food.
- [x] Add late-game events that interact with prestige, luxury rooms, or conversion projects.
- [x] Add report metrics for event frequency, event cost, and event rewards by campaign phase.

## Validation Checklist

- [x] Run the 30-day simulation report and confirm the average ends near 6 girls.
- [x] Run the 90-day simulation report and confirm growth continues without runaway wealth.
- [x] Run the 180-day simulation report and confirm the roster normally exceeds 9 girls.
- [x] Run the 365-day simulation report and confirm the campaign reaches the intended debt-pressure state and 20-girl cap.
- [x] Review population, money, eggs, and buildings together instead of treating any one metric as success.
- [x] Review whether the sim is sending enough dungeon missions after day 90.
- [x] Review whether accepted guest requests block expedition growth for gameplay reasons, not arbitrary roster thresholds.
- [x] Capture and compare any UI changes that expose upkeep forecasts or new balance systems.

## Suggested Implementation Order

- [x] Phase 1: Add missing report metrics and regression assertions.
- [x] Phase 2: Smooth hatch pacing and verify 30/90/180/365 roster targets.
- [x] Phase 3: Add upkeep bands and player-facing upkeep forecasts.
- [x] Phase 4: Scale guest demand from roster, rooms, reputation, and buildings.
- [x] Phase 5: Add expedition opportunity costs.
- [x] Phase 6: Add late-game sinks and expensive upgrade projects.
- [x] Phase 7: Tune building costs, event pressure, and debt milestones against the full one-year campaign.
