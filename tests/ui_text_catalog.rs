//! The text catalog has to stay honest.
//!
//! `ui_text.json` is the project's single source for player-facing English, and
//! it had drifted to 110 dead keys out of 366 — nearly a third. A key nothing
//! reads is worse than no key: an author edits the wording, sees no change, and
//! has no way to tell whether the screen was rewritten in hardcoded English or
//! they simply typed the wrong name.

use std::fs;
use std::path::{Path, PathBuf};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn collect_rust_sources(dir: &Path, skip_file_name: &str, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, skip_file_name, out);
        } else if path.extension().is_some_and(|ext| ext == "rs")
            && path.file_name().is_some_and(|name| name != skip_file_name)
        {
            if let Ok(text) = fs::read_to_string(&path) {
                out.push(text);
            }
        }
    }
}

/// Field names declared on the catalog's structs, in declaration order.
fn declared_fields(catalog_source: &str) -> Vec<String> {
    catalog_source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let rest = trimmed.strip_prefix("pub ")?;
            let name = rest.split(':').next()?.trim();
            if name.is_empty()
                || name == "version"
                || !name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
            {
                return None;
            }
            Some(name.to_owned())
        })
        .collect()
}

#[test]
fn every_text_key_is_read_by_a_screen() {
    let root = manifest_dir();
    let catalog_path = root.join("src/data/ui_text.rs");
    let catalog_source = fs::read_to_string(&catalog_path).expect("ui_text.rs should be readable");

    let mut sources = Vec::new();
    collect_rust_sources(&root.join("src"), "ui_text.rs", &mut sources);
    let all_source = sources.join("\n");

    // A field is "read" if it is accessed as `.name` anywhere outside the
    // catalog's own declaration. Shared names across sub-structs are counted
    // conservatively — the point is to catch whole keys nothing touches, not to
    // prove which struct a given access belongs to.
    let unread = declared_fields(&catalog_source)
        .into_iter()
        .filter(|name| !all_source.contains(&format!(".{name}")))
        .collect::<Vec<_>>();

    assert!(
        unread.is_empty(),
        "{} text key(s) in ui_text.rs are never read by any screen — wire them up or delete them: {unread:?}",
        unread.len()
    );
}
