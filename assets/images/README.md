# Image Assets

First-pass non-character UI asset pack for Monsterhall.

## Backdrops

Generated environmental backdrops live in `backdrops/`.

- `main_menu.png`: exterior ruined keep and tower at night.
- `title_page.png`: title screen illustration used behind the main menu.
- `town.png`: command-table town / journal mood and shared fallback.
- `town_overview.png`: organic command chamber backdrop for the Town Overview screen.
- `chamber.png`: occult hatchery and egg apparatus.
- `expedition.png`: tower descent staging room and scouting table.

These are source-quality 16:9 PNGs generated at 1672 x 941. They contain no baked UI text or buttons and are intended to sit behind the existing Macroquad panel layout.

## Icons

The first-pass UI icon atlas lives in `icons/`.

- `ui_icon_atlas.png`: 512 x 384 transparent PNG atlas.
- `ui_icon_atlas.json`: 64 x 64 cell coordinate manifest.

The atlas currently includes 43 icons across resources, stats, conditions, assignments, navigation, status, missions, and actions. It is designed as a practical first pass for Macroquad texture-atlas integration rather than final marketing art.

## Integration Notes

- Keep procedural drawing fallbacks in `src/ui/art.rs` until image loading is wired and missing asset handling is tested.
- Backdrops should be drawn with a dark overlay or panel scrim so current text contrast remains stable.
- Icons are transparent and should be drawn over dark UI surfaces. They assume the current Monsterhall palette from `src/ui/theme.rs`.
- If individual icon PNGs are preferred later, export from the atlas using the JSON coordinates instead of redrawing them manually.
