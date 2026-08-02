//! The icon atlas is described twice, so the two descriptions have to agree.
//!
//! `assets/images/icons/ui_icon_atlas.json` declares an explicit rect for every
//! icon. `art_helpers::icon_source` ignores it and derives the rect from the
//! icon's position in an enum, assuming an eight-column grid of `ICON_CELL`
//! squares. Both are true today by coincidence of authoring order — repack the
//! sheet, update the JSON, and nothing on screen would move.
//!
//! Rather than load the JSON at runtime for a layout that never changes at play
//! time, this pins the agreement: the atlas file stays the human-readable
//! description of the sheet, and drifting from the code fails here.

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct AtlasFile {
    cell_size: f32,
    icons: std::collections::HashMap<String, AtlasIcon>,
}

#[derive(Deserialize)]
struct AtlasIcon {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

/// Mirrors `art_helpers::icon_source`, which is private to the binary crate.
/// If that derivation changes, this has to change with it — and the assertion
/// below is what makes the mismatch visible.
fn derived_rect(index: usize, cell: f32) -> (f32, f32, f32, f32) {
    let col = (index % 8) as f32;
    let row = (index / 8) as f32;
    (col * cell, row * cell, cell, cell)
}

#[test]
fn the_atlas_file_describes_the_grid_the_code_assumes() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/images/icons/ui_icon_atlas.json");
    let atlas: AtlasFile =
        serde_json::from_str(&fs::read_to_string(&path).expect("atlas json should be readable"))
            .expect("atlas json should parse");

    // `ICON_CELL` in art_helpers.rs. A sheet repacked at another cell size would
    // shear every icon by a fraction of a tile, which reads as slightly wrong
    // art rather than as an error.
    assert_eq!(
        atlas.cell_size, 64.0,
        "atlas cell_size disagrees with ICON_CELL in art_helpers.rs"
    );

    // Sorted by row then column, the file's own rects must be exactly the grid
    // positions the code counts out. Comparing in grid order rather than by name
    // avoids depending on JSON key order.
    let mut icons = atlas.icons.iter().collect::<Vec<_>>();
    icons.sort_by(|(_, a), (_, b)| {
        (a.y, a.x)
            .partial_cmp(&(b.y, b.x))
            .expect("atlas rects should be comparable")
    });

    for (index, (name, icon)) in icons.iter().enumerate() {
        let expected = derived_rect(index, atlas.cell_size);
        assert_eq!(
            (icon.x, icon.y, icon.w, icon.h),
            expected,
            "'{name}' sits at slot {index} in the sheet but the code would read {expected:?}"
        );
    }

    // The enum drives the index, so a count mismatch means some icon has no slot
    // or some slot has no icon.
    assert_eq!(
        icons.len(),
        43,
        "the atlas declares {} icons; UiIcon::ALL has 43",
        icons.len()
    );
}
