//! Paging for the two-column companion card grids.
//!
//! The Expedition Desk team panel and the Contract Desk candidate panel both
//! drew `game_state.monsters.iter().take(6)` against a population cap of twenty.
//! Six was never a layout measurement — it was the roster size when those panels
//! were written. Once the guild filled up, fourteen companions could not be
//! assigned to an expedition or offered to a contract at all, and nothing on
//! either screen said they existed.
//!
//! The floor list solved the same problem by deriving its page from the selected
//! floor, which kept it stateless. That does not transfer: a roster panel has no
//! selection to follow, and the cards are a height-bounded two-column grid
//! rather than a column of fixed rows. So the page is carried in phase state
//! beside `inventory_scroll`, which is the existing precedent for exactly this —
//! transient, never saved, and preserved across a phase rebuild so assigning
//! somebody does not throw the player back to the first page.
//!
//! Deliberately *not* sorted. Ordering the roster by availability would put the
//! useful cards on page one, but assignments change as the player works, so the
//! cards would reshuffle under the cursor between clicks. A stable order the
//! player can learn beats a clever one they cannot predict.

use crate::ui::actions::UiAction;
use crate::ui::core::{draw_body_text, utility_button};
use crate::ui::theme;

/// Companion cards per row in the two assignment panels. The Town Overview
/// roster strip is three across and passes its own.
pub(super) const ROSTER_COLUMNS: usize = 2;
const PAGER_BUTTON_W: f32 = 46.0;
pub(super) const PAGER_ROW_H: f32 = 26.0;
/// Vertical room a panel must keep clear for a pager it draws.
pub(super) const PAGER_RESERVE_H: f32 = PAGER_ROW_H + 8.0;

/// Which slice of the roster a card panel is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RosterWindow {
    /// Index of the first companion drawn.
    pub(super) first_index: usize,
    /// How many companions are drawn.
    pub(super) visible_count: usize,
    /// Cards on a full page. Also the step the pager moves by.
    pub(super) page_size: usize,
    /// The page actually being shown, after clamping.
    pub(super) page: usize,
    /// How many pages the roster needs.
    pub(super) page_count: usize,
    /// Cards per row, so `card_rows` and the panel height agree with the grid
    /// the caller actually draws.
    columns: usize,
}

impl RosterWindow {
    /// `row_capacity` is how many card rows physically fit in the panel. When
    /// the whole roster fits, no pager is drawn and every row goes to cards, so
    /// a six-companion guild looks exactly as it did before paging existed.
    pub(super) fn new(roster_len: usize, page: usize, row_capacity: usize, columns: usize) -> Self {
        let row_capacity = row_capacity.max(1);
        let columns = columns.max(1);
        let full_capacity = row_capacity * columns;

        if roster_len <= full_capacity {
            return Self {
                first_index: 0,
                visible_count: roster_len,
                page_size: full_capacity,
                page: 0,
                page_count: 1,
                columns,
            };
        }

        // A multi-row panel gives the pager a row of its own; a single-row strip
        // keeps all its cards and takes the pager out of the card height
        // instead. Either way the floor is one full row, because a panel too
        // short to page is still better than one that silently hides the guild.
        let page_size = if row_capacity > 1 {
            (row_capacity - 1) * columns
        } else {
            columns
        };
        let page_count = roster_len.div_ceil(page_size);
        let page = page.min(page_count.saturating_sub(1));
        let first_index = page * page_size;

        Self {
            first_index,
            visible_count: page_size.min(roster_len - first_index),
            page_size,
            page,
            page_count,
            columns,
        }
    }

    pub(super) fn needs_pager(&self) -> bool {
        self.page_count > 1
    }

    /// Rows of cards actually drawn, so the caller can size its panel and place
    /// the pager underneath them.
    pub(super) fn card_rows(&self) -> usize {
        self.visible_count.div_ceil(self.columns)
    }

    fn has_previous(&self) -> bool {
        self.page > 0
    }

    fn has_next(&self) -> bool {
        self.page + 1 < self.page_count
    }
}

/// Draws the pager beneath a roster grid. Returns the page the player asked for.
///
/// `make_action` keeps this shared between the two screens, which carry their
/// page on different phase states.
pub(super) fn draw_roster_pager(
    window: &RosterWindow,
    roster_len: usize,
    x: f32,
    y: f32,
    width: f32,
    make_action: impl Fn(usize) -> UiAction,
) -> Option<UiAction> {
    if !window.needs_pager() {
        return None;
    }

    let mut action = None;
    if window.has_previous()
        && utility_button(x, y, PAGER_BUTTON_W, PAGER_ROW_H, "<")
        && action.is_none()
    {
        action = Some(make_action(window.page - 1));
    }
    if window.has_next()
        && utility_button(
            x + width - PAGER_BUTTON_W,
            y,
            PAGER_BUTTON_W,
            PAGER_ROW_H,
            ">",
        )
        && action.is_none()
    {
        action = Some(make_action(window.page + 1));
    }

    let first_shown = window.first_index + 1;
    let last_shown = window.first_index + window.visible_count;
    draw_body_text(
        &format!("{first_shown}-{last_shown} of {roster_len}"),
        x + PAGER_BUTTON_W + 12.0,
        y + 18.0,
        15.0,
        theme::TEXT_MUTED,
    );

    action
}

/// Height a roster panel needs for `window`, including the pager row when one is
/// drawn. `header_h` is everything above the first card.
pub(super) fn roster_panel_height(window: &RosterWindow, header_h: f32, card_row_h: f32) -> f32 {
    let cards = window.card_rows().max(1) as f32 * card_row_h;
    let pager = if window.needs_pager() {
        PAGER_RESERVE_H
    } else {
        0.0
    };
    header_h + cards + pager
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A guild that fits must render exactly as it did before paging existed.
    #[test]
    fn a_small_roster_shows_everybody_and_no_pager() {
        let window = RosterWindow::new(5, 0, 3, ROSTER_COLUMNS);

        assert_eq!(window.first_index, 0);
        assert_eq!(window.visible_count, 5);
        assert_eq!(window.page_count, 1);
        assert!(!window.needs_pager());
    }

    /// The bug this exists for: twenty companions against a panel that fits six.
    #[test]
    fn every_companion_is_reachable_on_some_page() {
        let roster_len = 20;
        let row_capacity = 3;
        let mut seen = std::collections::HashSet::new();

        let pages = RosterWindow::new(roster_len, 0, row_capacity, ROSTER_COLUMNS).page_count;
        for page in 0..pages {
            let window = RosterWindow::new(roster_len, page, row_capacity, ROSTER_COLUMNS);
            for index in window.first_index..window.first_index + window.visible_count {
                assert!(
                    seen.insert(index),
                    "companion {index} is drawn on more than one page"
                );
            }
        }

        assert_eq!(
            seen.len(),
            roster_len,
            "every companion in the guild must be assignable from some page"
        );
    }

    /// Paging past the end lands on the last page rather than an empty panel.
    #[test]
    fn an_out_of_range_page_clamps_to_the_last_one() {
        let window = RosterWindow::new(20, 99, 3, ROSTER_COLUMNS);

        assert_eq!(window.page, window.page_count - 1);
        assert!(window.visible_count > 0);
        assert!(window.first_index < 20);
    }

    /// Releasing companions shrinks the roster under the current page; the panel
    /// must not go blank.
    #[test]
    fn a_shrinking_roster_never_shows_an_empty_page() {
        for roster_len in 1..=24 {
            let window = RosterWindow::new(roster_len, 3, 3, ROSTER_COLUMNS);
            assert!(
                window.visible_count > 0,
                "roster of {roster_len} showed nothing on page 3"
            );
            assert!(window.first_index + window.visible_count <= roster_len);
        }
    }

    /// Even the shortest panel pages rather than hiding the rest of the guild.
    #[test]
    fn a_one_row_panel_still_pages() {
        let window = RosterWindow::new(20, 0, 1, ROSTER_COLUMNS);

        assert_eq!(window.page_size, ROSTER_COLUMNS);
        assert!(window.needs_pager());
        assert_eq!(window.page_count, 10);
    }

    /// The panel has to grow a row when it draws a pager, or the pager lands on
    /// top of the last card — the failure this project keeps shipping.
    #[test]
    fn a_paged_panel_reserves_room_for_its_pager() {
        let paged = RosterWindow::new(20, 0, 3, ROSTER_COLUMNS);
        let unpaged = RosterWindow::new(4, 0, 3, ROSTER_COLUMNS);

        let paged_h = roster_panel_height(&paged, 46.0, 104.0);
        let unpaged_h = roster_panel_height(&unpaged, 46.0, 104.0);

        assert!(paged.needs_pager() && !unpaged.needs_pager());
        assert!(
            paged_h > 46.0 + paged.card_rows() as f32 * 104.0,
            "a pager needs vertical room of its own"
        );
        assert_eq!(unpaged_h, 46.0 + 2.0 * 104.0);
    }
}
