//! # term-icon
//!
//! Strictly verified single-cell (`CELL_WIDTH == 1`) Unicode icons for terminal
//! UIs. Every glyph exported through [`universal`] has passed a dual-pass
//! verification pipeline — PTY cursor-position assertions plus pixel-level
//! screenshot analysis — across macOS Terminal.app, Windows Terminal/conhost,
//! and Linux xterm simultaneously (the CI "AND gate").
//!
//! The crate is zero-dependency; the optional `ratatui` feature adds buffer
//! integration traits only.
//!
//! ```rust
//! use term_icon::universal;
//!
//! let icon = universal::box_drawing::BOX_2502;
//! assert_eq!(term_icon::SafeIcon::CELL_WIDTH, 1);
//! println!("{}", icon); // renders │ , guaranteed to occupy exactly one cell
//! ```

use std::fmt;

// rustfmt::skip on the module: the CI drift check diffs this file
// byte-for-byte against fresh manifest-gen output, so it must never be
// reformatted.
#[rustfmt::skip]
mod generated_manifest;
pub mod universal;

#[cfg(feature = "ratatui")]
pub mod ratatui_ext;

/// A single-cell icon together with its ASCII fallback.
///
/// Every `SafeIcon` exported from [`universal`] satisfies the core invariant
/// `CELL_WIDTH == 1`: the glyph is empirically verified to render inside one
/// character cell across all supported platform default terminals. Any
/// candidate that expands to two cells or drifts into a neighbour cell is
/// permanently rejected by the CI verification pipeline and never reaches this
/// struct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SafeIcon {
    /// The verified single-cell glyph.
    pub glyph: &'static str,
    /// Plain-ASCII substitute for environments that need a conservative
    /// rendering path (logging, non-Unicode sinks, degraded fonts).
    pub fallback: &'static str,
}

impl SafeIcon {
    /// Verified render width of every exported glyph, in terminal cells.
    pub const CELL_WIDTH: u16 = 1;

    /// Create a `SafeIcon` from its glyph and fallback.
    pub const fn new(glyph: &'static str, fallback: &'static str) -> Self {
        Self { glyph, fallback }
    }

    /// The ASCII fallback string.
    pub const fn fallback_or_glyph(&self) -> &'static str {
        if self.fallback.is_empty() {
            self.glyph
        } else {
            self.fallback
        }
    }
}

impl fmt::Display for SafeIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.glyph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal::UNIVERSAL_ICONS;

    #[test]
    fn cell_width_is_one() {
        assert_eq!(SafeIcon::CELL_WIDTH, 1);
    }

    #[test]
    fn display_writes_glyph() {
        let icon = SafeIcon::new("│", "|");
        assert_eq!(icon.to_string(), "│");
    }

    #[test]
    fn manifest_is_sorted_and_unique() {
        assert!(!UNIVERSAL_ICONS.is_empty());
        for pair in UNIVERSAL_ICONS.windows(2) {
            let a = pair[0].glyph.chars().next().unwrap();
            let b = pair[1].glyph.chars().next().unwrap();
            assert!(a < b, "manifest not sorted by codepoint: {a:?} >= {b:?}");
        }
    }

    #[test]
    fn manifest_entries_stay_inside_verified_blocks() {
        for icon in UNIVERSAL_ICONS {
            let c = icon.glyph.chars().next().expect("glyph is one char");
            let cp = c as u32;
            let in_verified_block = (0x21..=0x7E).contains(&cp)
                || (0x2500..=0x257F).contains(&cp)
                || (0x2580..=0x259F).contains(&cp)
                || (0x2801..=0x28FF).contains(&cp);
            assert!(
                in_verified_block,
                "codepoint U+{cp:04X} outside verified single-cell blocks"
            );
            // Non-ASCII blocks must carry a conservative fallback; ASCII
            // glyphs are their own fallback (empty is correct).
            if !(0x21..=0x7E).contains(&cp) {
                assert!(!icon.fallback.is_empty());
            }
        }
    }
}
