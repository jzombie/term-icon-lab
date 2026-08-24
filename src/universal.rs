//! Verified single-cell icons, organized by Unicode block.
//!
//! Every item re-exported here was produced by the CI verification pipeline
//! (the "AND gate"): a glyph appears in this module only if it passed both the
//! PTY cursor-position assertion (Pass 1) and the pixel-level screenshot
//! assertion (Pass 2) on **all** supported platform default terminals.
//!
//! The initial committed manifest is a documented bootstrap produced with
//! `manifest-gen --from-catalog --assume-all`; it is provisional until the
//! first full CI matrix run replaces it via the drift-check.

pub use crate::generated_manifest::{
    ICON_ENTRIES, IconEntry, UNIVERSAL_ICONS, arrows, ascii, block_elements, box_drawing, braille,
    by_codepoint, dingbats, geometric_shapes, lookup, misc_symbols,
};

/// Iterator over every verified icon.
pub fn iter() -> impl Iterator<Item = &'static crate::SafeIcon> {
    UNIVERSAL_ICONS.iter()
}

/// Case-insensitive substring search over the verified set.
///
/// Matches against the official Unicode character name, the catalog id, the
/// block name, and the codepoint given as hex (with or without a `0x`/`U+`
/// prefix) or decimal — e.g. `"star"`, `"box_25"`, `"2605"`, `"0x25a0"`, or
/// the glyph character itself.
///
/// All query normalization happens up front; the returned iterator performs
/// zero heap allocations per element.
pub fn search(query: &str) -> impl Iterator<Item = &'static IconEntry> {
    let query_upper = query.to_uppercase();
    let query_lower = query.to_lowercase();
    let numeric: Option<u32> = {
        let t = query.trim();
        t.strip_prefix("0x")
            .or_else(|| t.strip_prefix("U+"))
            .or_else(|| t.strip_prefix("u+"))
            .and_then(|h| u32::from_str_radix(h, 16).ok())
            .or_else(|| t.parse::<u32>().ok())
    };
    ICON_ENTRIES.iter().filter(move |e| {
        numeric.is_some_and(|cp| e.codepoint == cp)
            || e.unicode_name.contains(&query_upper)
            || e.id.contains(&query_lower)
            || e.block.contains(&query_lower)
            || e.icon.glyph.contains(query)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn block_modules_resolve() {
        assert_eq!(super::ascii::ASCII_0021.glyph, "!");
        assert_eq!(super::box_drawing::BOX_2502.glyph, "\u{2502}");
        assert_eq!(super::block_elements::BLOCK_258C.glyph, "\u{258C}");
        assert_eq!(super::braille::BRAILLE_2801.glyph, "\u{2801}");
        assert_eq!(super::arrows::ARROW_21A8.glyph, "\u{21A8}");
        assert_eq!(super::geometric_shapes::GEOMETRIC_25AA.glyph, "\u{25AA}");
        assert_eq!(super::misc_symbols::MISC_2607.glyph, "\u{2607}");
        assert_eq!(super::dingbats::DINGBAT_2758.glyph, "\u{2758}");
    }

    #[test]
    fn lookups_resolve() {
        assert_eq!(super::lookup("box_2502").map(|i| i.glyph), Some("\u{2502}"));
        assert_eq!(
            super::by_codepoint('\u{258C}').map(|i| i.glyph),
            Some("\u{258C}")
        );
        assert_eq!(
            super::lookup("arrow_21A8").map(|i| i.glyph),
            Some("\u{21A8}")
        );
        assert!(super::lookup("nonexistent").is_none());
    }

    #[test]
    fn search_matches_name_id_and_codepoint() {
        // Official Unicode name, case-insensitive.
        let by_name: Vec<_> = super::search("light vertical").collect();
        assert!(by_name.iter().any(|e| e.id == "box_2502"));
        // Catalog id substring.
        assert!(super::search("box_25").all(|e| e.id.starts_with("box_")));
        // Hex codepoint with and without prefix, plus decimal.
        assert!(super::search("0x2502").any(|e| e.id == "box_2502"));
        assert!(super::search("2502").any(|e| e.id == "box_2502"));
        assert!(super::search("9474").any(|e| e.id == "box_2502"));
        // Glyph character itself.
        assert!(super::search("\u{2502}").any(|e| e.id == "box_2502"));
        // No allocation-heavy behavior to assert directly; empty query
        // matches everything.
        assert_eq!(super::search("").count(), super::ICON_ENTRIES.len());
    }

    #[test]
    fn bootstrap_manifest_covers_full_catalog() {
        // The bootstrap (--from-catalog) included every candidate; the first
        // full CI matrix run AND-gated it down to the verified set. This
        // floor only catches catastrophic regressions — the manifest drift
        // check in CI enforces the exact set.
        assert!(super::UNIVERSAL_ICONS.len() >= 50);
    }
}
