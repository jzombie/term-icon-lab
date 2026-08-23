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
    UNIVERSAL_ICONS, ascii, block_elements, box_drawing, braille, by_codepoint, lookup,
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
        assert_eq!(super::block_elements::BLOCK_2588.glyph, "\u{2588}");
        assert_eq!(super::braille::BRAILLE_2801.glyph, "\u{2801}");
    }

    #[test]
    fn lookups_resolve() {
        assert_eq!(super::lookup("box_2502").map(|i| i.glyph), Some("\u{2502}"));
        assert_eq!(
            super::by_codepoint('\u{2588}').map(|i| i.glyph),
            Some("\u{2588}")
        );
        assert!(super::lookup("nonexistent").is_none());
    }

    #[test]
    fn bootstrap_manifest_covers_full_catalog() {
        // Bootstrap (--from-catalog) includes every candidate; after the
        // first CI run the count may drop but never below a sane floor.
        assert!(super::UNIVERSAL_ICONS.len() >= 400);
    }
}
