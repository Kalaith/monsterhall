//! Floor list panel for expedition planning.
//!
//! The panel holds a fixed number of 40px rows, so a growing tower has to page
//! rather than overflow. The visible page is derived from the selected floor
//! instead of being stored, which keeps the screen stateless: selecting a floor
//! is already an action, and the window simply follows the selection.

use crate::data::TowerFloorData;
use crate::ui::actions::UiAction;
use crate::ui::core::{draw_body_text, primary_button, secondary_button, utility_button};
use crate::ui::theme;
use crate::ui::view_models::fill_template;

pub(super) const FLOOR_ROW_H: f32 = 40.0;
const FLOOR_ROW_BUTTON_H: f32 = 30.0;
const PAGER_BUTTON_W: f32 = 46.0;

/// Which slice of the unlocked floors the panel is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FloorListWindow {
    /// Index of the first floor drawn.
    pub(super) first_index: usize,
    /// How many floors are drawn.
    pub(super) visible_count: usize,
    /// Floors per page. Also the step the pager moves the selection by.
    pub(super) rows_per_page: usize,
    /// Whether a pager row is drawn beneath the floors.
    pub(super) needs_pager: bool,
}

impl FloorListWindow {
    /// `row_capacity` is how many rows physically fit in the panel. When every
    /// floor fits, the pager is skipped entirely and all rows go to floors —
    /// a three-floor tower looks exactly as it did before paging existed.
    pub(super) fn new(floor_count: usize, selected_index: usize, row_capacity: usize) -> Self {
        let row_capacity = row_capacity.max(1);

        if floor_count <= row_capacity {
            return Self {
                first_index: 0,
                visible_count: floor_count,
                rows_per_page: row_capacity,
                needs_pager: false,
            };
        }

        // The pager costs one row, leaving the rest for floors.
        let rows_per_page = (row_capacity - 1).max(1);
        let page = selected_index / rows_per_page;
        let first_index = page * rows_per_page;

        Self {
            first_index,
            visible_count: rows_per_page.min(floor_count - first_index),
            rows_per_page,
            needs_pager: true,
        }
    }

    fn has_previous(&self) -> bool {
        self.first_index > 0
    }

    fn has_next(&self, floor_count: usize) -> bool {
        self.first_index + self.visible_count < floor_count
    }

    /// Index the "previous page" control should select: the first floor of the
    /// page above, so repeated presses walk pages rather than single floors.
    fn previous_page_index(&self) -> usize {
        self.first_index.saturating_sub(self.rows_per_page)
    }

    /// Index the "next page" control should select, clamped to the last floor
    /// so a partial final page still lands somewhere real.
    fn next_page_index(&self, floor_count: usize) -> usize {
        (self.first_index + self.rows_per_page).min(floor_count.saturating_sub(1))
    }
}

/// How many floor rows fit between `first_row_y` and the bottom of the panel.
pub(super) fn floor_row_capacity(panel_y: f32, panel_h: f32, first_row_y: f32) -> usize {
    let available = panel_y + panel_h - first_row_y - 8.0;
    if available < FLOOR_ROW_H {
        return 1;
    }
    (available / FLOOR_ROW_H) as usize
}

/// Draws the floor rows plus, when the tower outgrows the panel, a pager.
/// Returns the selection intent for whichever control was pressed.
#[allow(clippy::too_many_arguments)]
pub(super) fn draw_floor_list(
    floors: &[&TowerFloorData],
    selected_floor_id: &str,
    window: FloorListWindow,
    depth_template: &str,
    x: f32,
    first_row_y: f32,
    width: f32,
) -> Option<UiAction> {
    for offset in 0..window.visible_count {
        let index = window.first_index + offset;
        let Some(floor) = floors.get(index) else {
            break;
        };
        let y = first_row_y + offset as f32 * FLOOR_ROW_H;
        let is_selected = floor.id == selected_floor_id;
        let label = if is_selected {
            fill_template(
                depth_template,
                &[
                    ("{name}", floor.name.clone()),
                    ("{depth}", floor.depth.to_string()),
                ],
            )
        } else {
            floor.name.clone()
        };
        let pressed = if is_selected {
            primary_button(x, y, width, FLOOR_ROW_BUTTON_H, &label)
        } else {
            secondary_button(x, y, width, FLOOR_ROW_BUTTON_H, &label)
        };
        if pressed {
            return Some(UiAction::SelectExpeditionFloor(floor.id.clone()));
        }
    }

    if !window.needs_pager {
        return None;
    }

    let pager_y = first_row_y + window.visible_count as f32 * FLOOR_ROW_H;
    if window.has_previous() && utility_button(x, pager_y, PAGER_BUTTON_W, FLOOR_ROW_BUTTON_H, "<")
    {
        let index = window.previous_page_index();
        return floors
            .get(index)
            .map(|floor| UiAction::SelectExpeditionFloor(floor.id.clone()));
    }
    if window.has_next(floors.len())
        && utility_button(
            x + width - PAGER_BUTTON_W,
            pager_y,
            PAGER_BUTTON_W,
            FLOOR_ROW_BUTTON_H,
            ">",
        )
    {
        let index = window.next_page_index(floors.len());
        return floors
            .get(index)
            .map(|floor| UiAction::SelectExpeditionFloor(floor.id.clone()));
    }

    let range_label = format!(
        "{}-{} of {}",
        window.first_index + 1,
        window.first_index + window.visible_count,
        floors.len()
    );
    draw_body_text(
        &range_label,
        x + PAGER_BUTTON_W + 10.0,
        pager_y + 20.0,
        15.0,
        theme::TEXT_MUTED,
    );

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_floor_is_drawn_when_the_tower_fits_the_panel() {
        let window = FloorListWindow::new(3, 0, 6);

        assert!(!window.needs_pager);
        assert_eq!(window.first_index, 0);
        assert_eq!(window.visible_count, 3);
    }

    #[test]
    fn a_full_panel_still_skips_the_pager() {
        let window = FloorListWindow::new(6, 5, 6);

        assert!(!window.needs_pager);
        assert_eq!(window.visible_count, 6);
    }

    /// The regression this whole module exists for: the panel used to draw
    /// `take(4)`, so floors past the fourth were unreachable.
    #[test]
    fn a_twenty_five_floor_tower_can_reach_its_deepest_floor() {
        let capacity = 6;
        let deepest = 24;
        let window = FloorListWindow::new(25, deepest, capacity);

        assert!(window.needs_pager);
        assert!(window.first_index <= deepest);
        assert!(
            deepest < window.first_index + window.visible_count,
            "the selected floor must be inside the drawn window"
        );
    }

    #[test]
    fn the_window_follows_the_selection_across_pages() {
        let capacity = 6;
        // 5 floors per page once the pager takes a row.
        assert_eq!(FloorListWindow::new(25, 0, capacity).first_index, 0);
        assert_eq!(FloorListWindow::new(25, 4, capacity).first_index, 0);
        assert_eq!(FloorListWindow::new(25, 5, capacity).first_index, 5);
        assert_eq!(FloorListWindow::new(25, 12, capacity).first_index, 10);
    }

    #[test]
    fn a_partial_last_page_never_draws_past_the_final_floor() {
        // 25 floors, 5 per page -> the last page holds exactly 5.
        let window = FloorListWindow::new(23, 22, 6);

        assert_eq!(window.first_index, 20);
        assert_eq!(window.visible_count, 3);
        assert!(!window.has_next(23));
        assert!(window.has_previous());
    }

    #[test]
    fn paging_steps_a_whole_page_and_clamps_at_both_ends() {
        let window = FloorListWindow::new(25, 12, 6);

        assert_eq!(window.first_index, 10);
        assert_eq!(window.previous_page_index(), 5);
        assert_eq!(window.next_page_index(25), 15);

        let first_page = FloorListWindow::new(25, 0, 6);
        assert!(!first_page.has_previous());
        assert_eq!(first_page.previous_page_index(), 0);

        let last_page = FloorListWindow::new(25, 24, 6);
        assert!(!last_page.has_next(25));
        assert_eq!(last_page.next_page_index(25), 24);
    }

    /// A panel too short for even one row must still draw one, not divide by
    /// zero or silently show nothing.
    #[test]
    fn a_cramped_panel_still_draws_a_row() {
        let window = FloorListWindow::new(25, 7, 1);

        assert_eq!(window.rows_per_page, 1);
        assert_eq!(window.visible_count, 1);
        assert_eq!(window.first_index, 7);
    }

    #[test]
    fn row_capacity_counts_only_rows_that_fit_the_panel() {
        // Panel spans y=92..386, rows start at 134 -> 244px of usable run.
        assert_eq!(floor_row_capacity(92.0, 294.0, 134.0), 6);
        // A panel with no room below the first row still reports one.
        assert_eq!(floor_row_capacity(92.0, 40.0, 134.0), 1);
    }
}
