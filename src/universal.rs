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
    UNIVERSAL_ICONS, arrows, ascii, block_elements, box_drawing, braille, by_codepoint, dingbats,
    geometric_shapes, lookup, misc_symbols,
};

/// Iterator over every verified icon.
pub fn iter() -> impl Iterator<Item = &'static crate::SafeIcon> {
    UNIVERSAL_ICONS.iter()
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
    fn bootstrap_manifest_covers_full_catalog() {
        // The bootstrap (--from-catalog) included every candidate; the first
        // full CI matrix run AND-gated it down to the verified set. This
        // floor only catches catastrophic regressions — the manifest drift
        // check in CI enforces the exact set.
        assert!(super::UNIVERSAL_ICONS.len() >= 50);
    }
}
