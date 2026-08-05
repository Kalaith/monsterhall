#[derive(Debug, Clone)]
pub struct GuildJobPreview {
    pub success_score: i32,
    pub projected_gold: u32,
    pub projected_arcane_residue: u32,
    pub projected_materials: u32,
    pub projected_reputation: i32,
    pub preparation_quality: u32,
    pub recovery_bonus: u32,
    /// Share of full output this companion delivers given her condition.
    pub effectiveness_pct: u32,
}

#[derive(Debug, Clone)]
pub struct ExpeditionPlanPreview {
    /// Raw capability margin used by reward and injury formulas.
    pub success_score: i32,
    /// Actual percentage chance the expedition roll succeeds.
    pub success_chance_pct: u32,
    pub projected_materials: u32,
    pub projected_arcane_residue: u32,
    pub projected_eggs: u32,
    pub projected_relics: u32,
    /// How far past the injury threshold the most exposed companion is. Above
    /// zero, somebody comes home hurt. `None` with no party assigned — there is
    /// nobody to hurt, so there is no number to give.
    pub injury_risk_score: Option<i32>,
    /// Average share of full output the assigned party can deliver.
    pub party_effectiveness_pct: u32,
}

#[derive(Debug, Clone, Default)]
pub struct UpkeepForecast {
    pub wage_gold: u32,
    pub cleaning_gold: u32,
    pub maintenance_gold: u32,
    pub total_gold: u32,
    pub active_band_min_companions: u32,
    /// Patron tiers the active band needs, or `None` when it does not escalate
    /// on that axis. The Town Overview drops the clause entirely rather than
    /// printing a threshold — the top band advertised "4 patron tiers" against a
    /// catalogue holding three, which the player could never reach.
    pub active_band_min_patron_tiers: Option<u32>,
    pub next_companion_total_gold: u32,
    pub next_companion_delta_gold: u32,
    pub next_building_total_gold: u32,
    pub next_building_delta_gold: u32,
}
