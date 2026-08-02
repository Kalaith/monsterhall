//! Seeding the game into a named screen for the headless capture harness.
//!
//! Split out of `actions.rs` when that file crossed the 800-line limit. This is
//! tooling rather than gameplay: nothing here runs in a real campaign, and it
//! exists so a screenshot can show a panel in the state where its bugs are
//! visible rather than the state where they are not.

use super::*;

impl Game {
    /// Drives the game to a named screen for the screenshot harness.
    ///
    /// The capture hook boots to the main menu and photographs whatever is
    /// there, so every screen built since has gone unphotographed. This starts a
    /// campaign, plays the opening out, and navigates — the same actions a
    /// player would take, so a scene that cannot be reached this way is a scene
    /// the player cannot reach either.
    pub fn seed_capture_scene(&mut self, scene: &str) {
        self.apply_action(UiAction::StartNewGame);
        // A fresh save prompts for confirmation; the second press is the answer.
        if self.last_error.is_some() {
            self.apply_action(UiAction::StartNewGame);
        }
        // The opening is not one button: BuildRoom and FirstClient each need
        // their own action, and the hatch detours through a reveal. Sending
        // ContinueOpening at every step leaves the campaign stuck on "Make The
        // Hall Useful", which is where the first round of captures were taken.
        for _ in 0..24 {
            match &self.phase {
                GamePhase::OpeningChapter(state) => match state.step {
                    OpeningChapterStep::BuildRoom => self.apply_action(UiAction::BuildOpeningRoom),
                    OpeningChapterStep::FirstClient => {
                        self.apply_action(UiAction::ResolveOpeningClient)
                    }
                    OpeningChapterStep::Complete => break,
                    _ => self.apply_action(UiAction::ContinueOpening),
                },
                GamePhase::HatchReveal(_) => self.apply_action(UiAction::ContinueAfterHatch),
                _ => break,
            }
        }

        // A `_full` suffix fills the guild to its population cap first. The
        // roster panels only misbehave once the guild is crowded, and a fresh
        // campaign has one companion — so without this the harness can only ever
        // photograph the state where the bug is invisible.
        let scene = match scene.strip_suffix("_full") {
            Some(base) => {
                self.fill_roster_for_capture();
                base
            }
            None => scene,
        };

        let action = match scene {
            "town" | "townoverview" => Some(UiAction::ReturnToTownOverview),
            "townmanagement" | "planner" => Some(UiAction::OpenTownManagement),
            "guildhall" | "guildjobs" => Some(UiAction::OpenGuildHallManagement),
            "contracts" | "contractdesk" => Some(UiAction::OpenContractDesk),
            "hatchery" => Some(UiAction::OpenHatcheryManagement),
            "expedition" => Some(UiAction::OpenExpeditionPlanning),
            "journal" => Some(UiAction::OpenJournal),
            "settings" => Some(UiAction::OpenSettings),
            "profile" => self
                .game_state
                .as_ref()
                .and_then(|state| state.monsters.first())
                .map(|monster| UiAction::OpenMonsterProfile(monster.id.clone())),
            "dayresults" => Some(UiAction::ResolveDay),
            _ => None,
        };
        if let Some(action) = action {
            self.apply_action(action);
        }
    }

    /// Pads the roster to the population cap, and the hatchery to more eggs than
    /// its column can show, so a screenshot shows a crowded guild.
    ///
    /// Capture-only: it copies what is already there rather than playing out a
    /// year, because what is being photographed is the panel, not the campaign
    /// that produced it. Every list this fills is one that only misbehaves once
    /// it is full.
    fn fill_roster_for_capture(&mut self) {
        let Some(data) = self.data.as_ref() else {
            return;
        };
        let Some(game_state) = self.game_state.as_mut() else {
            return;
        };
        let Some(template) = game_state.monsters.first().cloned() else {
            return;
        };
        let cap = usize::from(data.config.new_game.max_population_cap);
        let max_rank = crate::engine::max_quality_rank(&data.config.day_cycle);
        for index in game_state.monsters.len()..cap {
            let mut copy = template.clone();
            copy.id = format!("monster_{:03}", index + 1);
            copy.name = format!("{} {}", template.name, index + 1);
            copy.current_job = crate::state::CompanionJobState::Idle;
            // Twenty copies of one companion photograph every screen that
            // discriminates between them as though it held a single opinion:
            // the contract desk drew twenty identical "Eligible" cards, so its
            // refusals, its half-pay candidates and its star ladder could not
            // appear in a capture at all. Spread rank, training and standing.
            copy.quality_rank = (index as u8 % max_rank) + 1;
            let step = index as u32 % 4;
            copy.bond = step * 4;
            copy.reputation = step as i32 * 3;
            copy.skills.scouting = template.skills.scouting * step;
            copy.skills.guarding = template.skills.guarding * step;
            copy.skills.hospitality = template.skills.hospitality * step;
            copy.skills.charm = template.skills.charm * step;
            copy.work_history.scouting_runs = template.work_history.scouting_runs * step;
            copy.work_history.hospitality_jobs = template.work_history.hospitality_jobs * step;
            copy.work_history.contracts_completed = step;
            // And one species twenty times over is the same blindness one level
            // down: wage, role, stats, portrait and every floor's
            // `required_roster` all key off species, and a capture of twenty
            // slimes cannot show any of it. The stats and traits come with the
            // species, because a companion carries the block she hatched with —
            // a slime wearing a gargoyle's name would photograph a creature the
            // game cannot produce.
            if let Some(species) = data.species.species.get(index % data.species.species.len()) {
                copy.species_id = species.id.clone();
                copy.stats = species.base_stats.clone();
                copy.trait_ids = species.starting_traits.clone();
            }
            // And a guild that has been worked. Every copy carried zero
            // fatigue, stress, injury and instability, so the condition badges,
            // the worn notes on the guild-job and expedition cards and anything
            // instability-gated had never been photographed holding a number —
            // the three meters only mean something once they are not zero.
            // Spread across the roster rather than raised uniformly, because
            // what these screens are for is telling one companion from another.
            let wear = index as u32 % 5;
            copy.fatigue = wear * 22;
            copy.stress = wear * 14;
            copy.injury = (wear % 3) * 9;
            copy.corruption = (index as u32 % 7) * 34;
            game_state.monsters.push(copy);
        }

        let egg_template = game_state.egg_inventory.first().cloned();
        for index in game_state.egg_inventory.len()..12 {
            let egg = match &egg_template {
                Some(template) => crate::state::EggState {
                    id: format!("egg_{:03}", index + 1),
                    grade_score: (index as u32 * 3) % 20,
                    ..template.clone()
                },
                None => crate::state::EggState {
                    id: format!("egg_{:03}", index + 1),
                    source_floor_id: "tower_core".to_owned(),
                    possible_species_ids: vec!["slime_companion".to_owned()],
                    selected_species_id: None,
                    incubation_state: crate::state::EggIncubationState::Raw,
                    grade_score: (index as u32 * 3) % 20,
                    preparation_focus: None,
                },
            };
            game_state.egg_inventory.push(egg);
        }
        game_state.resources.eggs = game_state.egg_inventory.len() as u32;

        // A late-campaign purse. Every capture so far has photographed a day-one
        // economy — three-digit gold against a game that reaches seven — so a
        // number too wide for its tile would never have shown up in one. These
        // are the ten-seed day-365 maxima from the balance reports.
        game_state.resources.gold = 2_050_396;
        game_state.resources.arcane_residue = 120_248;
        game_state.resources.relics = 292;
        game_state.resources.tower_materials = 48_610;

        // And the campaign log, which is the list in this game that grows
        // largest and was the last one no capture had ever seen full. It gains
        // about twelve entries a day and is never trimmed, so a day-365 save
        // carries roughly 4,600. A journal photographed at nine entries is a
        // journal photographed in the one state where nothing about it can be
        // wrong.
        let logged_days = 365;
        for day in game_state.event_log.len()..logged_days * 12 {
            game_state.event_log.push(format!(
                "Day {} of the guild's business was recorded.",
                day / 12
            ));
        }

        // And put the guild to work. The day-results panels are filled by
        // companions who *did* something, so a crowded roster of idlers
        // photographs them just as empty as a roster of one. The job limit is
        // raised to match, because on day one it is small enough that the guild
        // cannot fill its own report however many companions it has.
        game_state.town.town_job_limit = game_state.monsters.len() as u8;
        if let Some(room_id) = game_state.town.unlocked_room_ids.first().cloned() {
            let monster_ids = game_state
                .monsters
                .iter()
                .map(|monster| monster.id.clone())
                .collect::<Vec<_>>();
            for monster_id in monster_ids {
                let _ = crate::engine::assign_monster_to_room(game_state, &monster_id, &room_id);
            }
        }

        // Book the first companion onto the first live offer, so the crowded
        // captures show the "On Contract" state the assignment panels grew. A
        // companion who cannot be rostered is only visible once somebody is.
        let booking = game_state
            .active_contracts
            .iter()
            .position(|request| request.status.is_live());
        if let (Some(index), Some(monster_id)) = (
            booking,
            game_state
                .monsters
                .first()
                .map(|monster| monster.id.clone()),
        ) {
            let request = &mut game_state.active_contracts[index];
            request.assigned_monster_id = Some(monster_id);
            request.status = crate::state::ContractStatus::Accepted;
        }

        // And a second booking taken by somebody the contract would refuse,
        // which the engine still accepts and pays half for. That path has
        // existed for the game's whole life and no capture has ever contained
        // it, so the day's report has never been read with a partial completion
        // in it.
        let half_pay_candidate = game_state
            .monsters
            .iter()
            .find(|monster| monster.skills.scouting == 0 && monster.skills.hospitality == 0)
            .map(|monster| monster.id.clone());
        let second_offer = game_state
            .active_contracts
            .iter()
            .position(|request| matches!(request.status, crate::state::ContractStatus::Pending));
        if let (Some(index), Some(monster_id)) = (second_offer, half_pay_candidate) {
            let request = &mut game_state.active_contracts[index];
            request.assigned_monster_id = Some(monster_id);
            request.status = crate::state::ContractStatus::Accepted;
        }
    }
}
