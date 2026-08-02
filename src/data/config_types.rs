//! Everything `config.json` deserializes into.
//!
//! Split out of `types.rs` when that file crossed the 800-line limit; the
//! campaign-content catalogues (species, floors, contracts, ...) stayed behind.

use serde::{Deserialize, Serialize};

use super::types::{ResourceAmountData, StatBlockData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfigData {
    pub primary_mode: String,
    pub keyboard_shortcuts_enabled: bool,
    pub keyboard_shortcuts_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfigData {
    pub native_save_path: String,
    pub web_storage_key: String,
    pub native_settings_path: String,
    pub web_settings_key: String,
    pub autosave_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionOptionData {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfigData {
    pub start_fullscreen: bool,
    pub default_resolution_id: String,
    pub available_resolutions: Vec<ResolutionOptionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterMonsterData {
    pub species_id: String,
    pub name: String,
    pub extra_traits: Vec<String>,
    pub stat_bonuses: StatBlockData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompanionSkillProgressionData {
    #[serde(alias = "scouting")]
    pub scouting: u32,
    #[serde(alias = "guarding")]
    pub guarding: u32,
    #[serde(alias = "hospitality")]
    pub hospitality: u32,
    #[serde(alias = "crafting")]
    pub crafting: u32,
    #[serde(alias = "charm")]
    pub charm: u32,
    pub recovery: u32,
    pub bargaining: u32,
    pub navigation: u32,
    pub arcana: u32,
    pub strength: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CompanionWorkHistoryProgressionData {
    #[serde(alias = "kiss_count")]
    pub scouting_runs: u32,
    pub guard_duties: u32,
    pub hospitality_jobs: u32,
    pub craft_jobs: u32,
    #[serde(alias = "contract_count")]
    pub contracts_completed: u32,
    #[serde(alias = "recovery_shift_count")]
    pub recovery_shifts: u32,
    #[serde(alias = "birth_count")]
    pub hatchery_assists: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGameConfigData {
    pub starting_day: u32,
    pub starting_resources: ResourceAmountData,
    pub starting_building_ids: Vec<String>,
    pub starting_room_ids: Vec<String>,
    pub starting_floor_ids: Vec<String>,
    pub starting_species_ids: Vec<String>,
    pub starter_monsters: Vec<StarterMonsterData>,
    pub party_size: u8,
    #[serde(alias = "guild_job_worker_limit")]
    pub town_job_limit: u8,
    pub population_cap: u16,
    pub max_population_cap: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayCycleConfigData {
    pub guild_job_fatigue: u32,
    pub expedition_fatigue: u32,
    pub guild_job_stress: u32,
    pub expedition_stress: u32,
    pub resting_fatigue_recovery: u32,
    pub resting_stress_recovery: u32,
    pub base_injury_recovery: u32,
    pub base_guild_job_success: i32,
    pub base_expedition_success: i32,
    pub preferred_trait_bonus_pct: i32,
    pub preferred_species_bonus_pct: i32,
    pub worker_charm_gold_multiplier: u32,
    pub worker_instinct_residue_multiplier: u32,
    pub expedition_power_materials_multiplier: u32,
    pub expedition_instinct_residue_multiplier: u32,
    pub expedition_endurance_safety_divisor: u32,
    pub expedition_reward_success_divisor: u32,
    /// Base daily wage before rank and skill scaling. Wages are the guild's
    /// answer to a roster that earns more as it gets stronger.
    pub companion_base_wage_gold: u32,
    /// Egg grade scores at which a companion reaches rank 2, 3, 4 and 5.
    #[serde(default = "default_quality_rank_thresholds")]
    pub egg_quality_rank_thresholds: Vec<u32>,
    /// What an escort of each rank earns, indexed rank 1..=5.
    #[serde(default = "default_quality_income_multipliers_pct")]
    pub quality_income_multipliers_pct: Vec<u32>,
    /// What an escort of each rank is paid, indexed rank 1..=5.
    #[serde(default = "default_quality_wage_multipliers_pct")]
    pub quality_wage_multipliers_pct: Vec<u32>,
    /// Gold a sold egg of each rank fetches, indexed rank 1..=5. These were a
    /// `match` in `convert_egg`, which put a balance curve in Rust and meant a
    /// longer ladder silently paid its top two ranks the same.
    #[serde(default = "default_egg_sale_gold_by_rank")]
    pub egg_sale_gold_by_rank: Vec<u32>,
    /// Arcane residue a dissolved egg of each rank yields, indexed rank 1..=5.
    #[serde(default = "default_egg_dissolve_residue_by_rank")]
    pub egg_dissolve_residue_by_rank: Vec<u32>,
    /// Rank at which dissolving an egg also recovers a relic.
    #[serde(default = "default_egg_dissolve_relic_minimum_rank")]
    pub egg_dissolve_relic_minimum_rank: u8,
    /// Injury a companion takes when she comes back from a run hurt.
    ///
    /// It was a bare `6` in `resolve_day` — the last balance number left in the
    /// day cycle's Rust. Every other side of this exchange is authored:
    /// `base_injury_recovery`, `injury_allowance`, `injury_penalty_pct_per_ten`
    /// and `expedition_injury_threshold` all live here, so how hard a bad run
    /// hits was the one term nobody could tune.
    #[serde(default = "default_expedition_injury_amount")]
    pub expedition_injury_amount: u32,
    #[serde(default = "default_skill_wage_divisor")]
    pub skill_wage_divisor: u32,
    /// Divides a species' total base stats into its share of the daily wage.
    #[serde(default = "default_species_stat_wage_divisor")]
    pub species_stat_wage_divisor: u32,
    /// Fee an adventuring party pays when the escort is below the calibre their
    /// patron tier demands. They still hire, but they do not pay full rate.
    #[serde(default = "default_understrength_income_pct")]
    pub understrength_income_pct: u32,
    pub building_maintenance_cost_divisor: u32,
    #[serde(default)]
    pub upkeep_bands: Vec<UpkeepBandData>,
    /// Hazard added per floor of depth. The primary depth dial is each floor's
    /// authored `difficulty`; this is the engine's slope on top of it.
    #[serde(default = "default_depth_hazard_per_floor")]
    pub depth_hazard_per_floor: i32,
    #[serde(default = "default_hazard_tag_risk")]
    pub hazard_tag_risk: i32,
    /// Hazard removed per survey point banked on the floor being run. Depth is
    /// meant to be survivable through familiarity, not only through stats.
    #[serde(default = "default_survey_familiarity_relief")]
    pub survey_familiarity_relief: i32,
    #[serde(default = "default_max_survey_familiarity_relief")]
    pub max_survey_familiarity_relief: i32,
    /// Difficulty past which a floor cannot be beaten by any realistic party, so
    /// authoring one is a content error rather than a hard floor.
    #[serde(default = "default_max_floor_difficulty")]
    pub max_floor_difficulty: u32,
    /// Portion of a floor's difficulty taken off the egg and relic reward bars.
    ///
    /// `success_score` already has `difficulty` subtracted from it, so a flat
    /// threshold charges depth twice and deep floors stop yielding eggs at all —
    /// which severs the one link that lets the tower produce better companions.
    #[serde(default = "default_reward_threshold_depth_relief_pct")]
    pub reward_threshold_depth_relief_pct: i32,
    /// Further portion of a floor's difficulty taken off one reward's bar when
    /// the mission was chosen to look for exactly that reward.
    ///
    /// The same double-charge as above, one level down: a mission's
    /// `success_bonus_pct` feeds `success_score`, and `success_score` gates the
    /// payout, so a stance that is deliberately riskier stops paying the thing
    /// it exists to fetch. Below depth 17 that made Relic Recovery the *worst*
    /// way to bring back a relic — it yielded none at all while the Egg Hunt's
    /// +20 success carried off the floor's entire relic pile.
    ///
    /// Scaled by difficulty rather than flat, because the gap it closes is a
    /// depth effect. A flat relief lands hardest on the shallow floors that were
    /// already clearing their bars, and a safe depth-5 errand paying three
    /// relics for no injury is enough to keep the guild out of the deep tower
    /// entirely.
    #[serde(default = "default_mission_focus_reward_relief_pct")]
    pub mission_focus_reward_relief_pct: i32,
    pub expedition_egg_reward_threshold: i32,
    pub expedition_relic_reward_threshold: i32,
    pub expedition_injury_threshold: i32,
    /// How far a worn-down companion's output falls off.
    #[serde(default = "default_condition_effects")]
    pub condition_effects: ConditionEffectData,
    #[serde(default = "default_role_affinity")]
    pub role_affinity: RoleAffinityData,
    #[serde(default = "default_role_thresholds")]
    pub role_thresholds: RoleThresholdsData,
}

/// Where each rung of `monster_role`'s ladder sits.
///
/// These were six literals inside the classifier, which put the game's whole
/// answer to *what is this companion for* out of the authors' reach. The
/// corruption rung is the one that mattered: at a hardcoded `10` on a meter
/// that only ever climbs and runs past 400 in a long campaign, it caught every
/// companion within about ten working days and made the five rungs below it
/// unreachable for the rest of the run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleThresholdsData {
    /// Corruption at which the meter alone makes a companion an adept, or
    /// `None` when it never does.
    ///
    /// **Prefer `None`.** Corruption is only ever added to, so any value here
    /// is a latch: every companion crosses it eventually and no companion ever
    /// reads as anything else again. Corruption already reaches the role system
    /// by the route that can express change rather than accumulation —
    /// mutation, which rewrites the species and carries `corruption_tuned`
    /// along most of its branches.
    #[serde(default)]
    pub corruption_adept_minimum: Option<u32>,
    /// Banked hatchery shifts that mark a companion as hatchery staff.
    pub hatchery_assist_minimum: u32,
    /// Trained charm at which a companion reads as a performer regardless of
    /// her stat spread.
    pub performer_charm_skill_minimum: u32,
    /// How far charm must lead power for a performer.
    pub performer_charm_margin: u32,
    /// How far power must lead charm for a delver.
    pub delver_power_margin: u32,
    /// Bond at which a companion reads as comfort staff.
    pub comfort_bond_minimum: u32,
}

/// The price of running a companion into the ground.
///
/// Fatigue, stress and injury are written by every job, rest, and expedition,
/// and were read by nothing — a burned-out roster earned exactly what a fresh
/// one did, which made the whole recovery economy (Resting, `recovery_bonus`,
/// the stress/injury recovery buildings) decoration. Each point of condition
/// damage past its allowance now shaves effectiveness off what that companion
/// contributes, floored so an exhausted worker is weak rather than useless.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionEffectData {
    /// Condition damage a companion carries before it costs anything. A day's
    /// work should not immediately dent the payout.
    pub fatigue_allowance: u32,
    pub stress_allowance: u32,
    pub injury_allowance: u32,
    /// Effectiveness lost per ten points past the matching allowance.
    pub fatigue_penalty_pct_per_ten: u32,
    pub stress_penalty_pct_per_ten: u32,
    pub injury_penalty_pct_per_ten: u32,
    /// Floor on effectiveness — even a wreck still shows up for the shift.
    pub min_effectiveness_pct: u32,
    /// Ceiling on every condition meter.
    ///
    /// Without one the meters only climb for anyone the player never explicitly
    /// parks, so a companion who worked a hundred straight days would need a
    /// month of rest before she was worth anything again. A cap keeps the worst
    /// case recoverable and bounds the penalty.
    pub max_meter: u32,
    /// Fatigue and stress an unassigned companion sheds each day. Standing
    /// around the hall is rest, just poorer rest than the Resting assignment —
    /// without it the meters are a one-way ratchet.
    pub idle_fatigue_recovery: u32,
    pub idle_stress_recovery: u32,
}

/// How much a companion's role matters, and how much slack she has outside it.
///
/// The peak bonus is the same for everyone; what differs by species is how much
/// credit she still earns off-role. A slime is worth having on unfamiliar work,
/// a gargoyle is not — high tiers buy raw capability at the price of being
/// narrow, which is what stops an all-high-tier roster being strictly correct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAffinityData {
    /// Paid when the companion's role is the one the work wants.
    pub matched_bonus: i32,
    /// Paid to a `versatile` companion on any work.
    pub versatile_bonus: i32,
    /// Off-role penalty carried by the most capable species; scales to zero at
    /// `flexibility_stat_floor`.
    pub off_role_penalty_max: i32,
    /// Species stat total at or below which a companion is fully flexible and
    /// pays nothing for working outside her role.
    pub flexibility_stat_floor: u32,
    /// Species stat total at or above which a companion pays the full penalty.
    pub flexibility_stat_ceiling: u32,
}

fn default_role_affinity() -> RoleAffinityData {
    RoleAffinityData {
        matched_bonus: 12,
        versatile_bonus: 4,
        off_role_penalty_max: 6,
        flexibility_stat_floor: 12,
        flexibility_stat_ceiling: 30,
    }
}

fn default_role_thresholds() -> RoleThresholdsData {
    RoleThresholdsData {
        corruption_adept_minimum: None,
        hatchery_assist_minimum: 1,
        performer_charm_skill_minimum: 2,
        performer_charm_margin: 2,
        delver_power_margin: 2,
        comfort_bond_minimum: 8,
    }
}

fn default_condition_effects() -> ConditionEffectData {
    ConditionEffectData {
        fatigue_allowance: 20,
        stress_allowance: 14,
        injury_allowance: 0,
        fatigue_penalty_pct_per_ten: 4,
        stress_penalty_pct_per_ten: 5,
        injury_penalty_pct_per_ten: 10,
        min_effectiveness_pct: 40,
        max_meter: 100,
        idle_fatigue_recovery: 6,
        idle_stress_recovery: 4,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpkeepBandData {
    pub min_companions: u32,
    /// Patron tiers this band needs, or `None` when the band does not escalate
    /// on that axis at all.
    ///
    /// `active_upkeep_band` selects with `count >= threshold`, so a plain `0`
    /// does **not** mean "ignore this axis" — it means *always active*, which
    /// would silently pin the guild to this band's multipliers from day one.
    /// The top band was authored `4` against a catalogue holding three tiers,
    /// which is the same thing said the other way: an axis that can never fire,
    /// written as though it could. `None` says it outright, and load-time
    /// validation now rejects both the unreachable threshold and the zero.
    #[serde(default)]
    pub min_patron_tiers: Option<u32>,
    #[serde(alias = "food_multiplier_pct")]
    pub wage_multiplier_pct: u32,
    pub cleaning_multiplier_pct: u32,
    pub maintenance_multiplier_pct: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfigData {
    pub title: String,
    pub content_version: String,
    pub save_version: u32,
    pub input: InputConfigData,
    pub persistence: PersistenceConfigData,
    pub display: DisplayConfigData,
    pub new_game: NewGameConfigData,
    pub day_cycle: DayCycleConfigData,
}

fn default_depth_hazard_per_floor() -> i32 {
    2
}

fn default_hazard_tag_risk() -> i32 {
    3
}

fn default_survey_familiarity_relief() -> i32 {
    1
}

fn default_max_survey_familiarity_relief() -> i32 {
    6
}

fn default_max_floor_difficulty() -> u32 {
    120
}

fn default_quality_rank_thresholds() -> Vec<u32> {
    vec![3, 6, 11, 18]
}

fn default_quality_income_multipliers_pct() -> Vec<u32> {
    vec![100, 175, 300, 500, 800]
}

fn default_quality_wage_multipliers_pct() -> Vec<u32> {
    vec![100, 160, 260, 420, 650]
}

fn default_egg_sale_gold_by_rank() -> Vec<u32> {
    vec![10, 20, 50, 110, 240]
}

fn default_egg_dissolve_residue_by_rank() -> Vec<u32> {
    vec![8, 18, 35, 68, 130]
}

fn default_egg_dissolve_relic_minimum_rank() -> u8 {
    3
}

fn default_expedition_injury_amount() -> u32 {
    6
}

fn default_species_stat_wage_divisor() -> u32 {
    4
}

fn default_skill_wage_divisor() -> u32 {
    4
}

fn default_understrength_income_pct() -> u32 {
    45
}

fn default_reward_threshold_depth_relief_pct() -> i32 {
    60
}

fn default_mission_focus_reward_relief_pct() -> i32 {
    15
}
