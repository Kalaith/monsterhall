use super::*;

#[derive(Debug, Clone)]
pub struct GuildJobPreview {
    pub success_score: i32,
    pub projected_gold: u32,
    pub projected_arcane_residue: u32,
    pub projected_materials: u32,
    pub projected_reputation: i32,
    pub preparation_quality: u32,
    pub recovery_bonus: u32,
    pub projected_work_history_gains: CompanionWorkHistoryState,
}

#[derive(Debug, Clone)]
pub struct ExpeditionPlanPreview {
    pub success_score: i32,
    pub projected_materials: u32,
    pub projected_arcane_residue: u32,
    pub projected_eggs: u32,
    pub projected_relics: u32,
    pub injury_risk_score: i32,
}

#[derive(Debug, Clone, Default)]
pub struct UpkeepForecast {
    pub food_gold: u32,
    pub cleaning_gold: u32,
    pub maintenance_gold: u32,
    pub total_gold: u32,
    pub active_band_min_girls: u32,
    pub active_band_min_patron_tiers: u32,
    pub next_girl_total_gold: u32,
    pub next_girl_delta_gold: u32,
    pub next_building_total_gold: u32,
    pub next_building_delta_gold: u32,
}
