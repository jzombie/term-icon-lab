//! Candidate registry of single-cell Unicode codepoints for term-icon.
//!
//! The catalog enumerates every candidate icon that enters the verification
//! pipeline, restricted to Unicode blocks whose East Asian Width properties
//! avoid inherent width ambiguity:
//!
//! | Block | Range | Notes |
//! |---|---|---|
//! | Basic ASCII | `U+0021–U+007E` | `U+0020` excluded (blank render) |
//! | Arrows | `U+2190–U+21FF` | |
//! | Box Drawing | `U+2500–U+257F` | |
//! | Block Elements | `U+2580–U+259F` | |
//! | Geometric Shapes | `U+25A0–U+25FF` | |
//! | Miscellaneous Symbols | `U+2600–U+26FF` | |
//! | Dingbats | `U+2700–U+27BF` | |
//! | Braille Patterns | `U+2801–U+28FF` | `U+2800 BRAILLE PATTERN BLANK` excluded (blank render) |

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Start of the Basic ASCII candidate range (`U+0021`, exclusive of space).
pub const ASCII_START: u32 = 0x21;
/// End of the Basic ASCII candidate range.
pub const ASCII_END: u32 = 0x7E;
/// Start of the Arrows candidate range.
pub const ARROWS_START: u32 = 0x2190;
/// End of the Arrows candidate range.
pub const ARROWS_END: u32 = 0x21FF;
/// Start of the Box Drawing candidate range.
pub const BOX_DRAWING_START: u32 = 0x2500;
/// End of the Box Drawing candidate range.
pub const BOX_DRAWING_END: u32 = 0x257F;
/// Start of the Block Elements candidate range.
pub const BLOCK_ELEMENTS_START: u32 = 0x2580;
/// End of the Block Elements candidate range.
pub const BLOCK_ELEMENTS_END: u32 = 0x259F;
/// Start of the Geometric Shapes candidate range.
pub const GEOMETRIC_SHAPES_START: u32 = 0x25A0;
/// End of the Geometric Shapes candidate range.
pub const GEOMETRIC_SHAPES_END: u32 = 0x25FF;
/// Start of the Miscellaneous Symbols candidate range.
pub const MISC_SYMBOLS_START: u32 = 0x2600;
/// End of the Miscellaneous Symbols candidate range.
pub const MISC_SYMBOLS_END: u32 = 0x26FF;
/// Start of the Dingbats candidate range.
pub const DINGBATS_START: u32 = 0x2700;
/// End of the Dingbats candidate range.
pub const DINGBATS_END: u32 = 0x27BF;
/// Start of the Braille Patterns candidate range.
///
/// `U+2800 BRAILLE PATTERN BLANK` is deliberately excluded: it renders zero
/// foreground pixels and would always fail the visibility assertion.
pub const BRAILLE_START: u32 = 0x2801;
/// End of the Braille Patterns candidate range.
pub const BRAILLE_END: u32 = 0x28FF;

/// The verified single-cell Unicode blocks a candidate may belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Block {
    /// Basic ASCII (`U+0021–U+007E`).
    Ascii,
    /// Arrows (`U+2190–U+21FF`).
    Arrows,
    /// Box Drawing (`U+2500–U+257F`).
    BoxDrawing,
    /// Block Elements (`U+2580–U+259F`).
    BlockElements,
    /// Geometric Shapes (`U+25A0–U+25FF`).
    GeometricShapes,
    /// Miscellaneous Symbols (`U+2600–U+26FF`).
    MiscSymbols,
    /// Dingbats (`U+2700–U+27BF`).
    Dingbats,
    /// Braille Patterns (`U+2801–U+28FF`).
    Braille,
}

impl Block {
    /// All blocks, in manifest order.
    pub const ALL: [Block; 8] = [
        Block::Ascii,
        Block::Arrows,
        Block::BoxDrawing,
        Block::BlockElements,
        Block::GeometricShapes,
        Block::MiscSymbols,
        Block::Dingbats,
        Block::Braille,
    ];

    /// Lowercase identifier prefix used in catalog ids and generated modules.
    pub fn prefix(self) -> &'static str {
        match self {
            Block::Ascii => "ascii",
            Block::Arrows => "arrow",
            Block::BoxDrawing => "box",
            Block::BlockElements => "block",
            Block::GeometricShapes => "geometric",
            Block::MiscSymbols => "misc",
            Block::Dingbats => "dingbat",
            Block::Braille => "braille",
        }
    }
}

/// A single candidate icon awaiting empirical verification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Candidate {
    /// Stable identifier: `{block_prefix}_{codepoint:04X}` (e.g. `box_2502`).
    pub id: String,
    /// The candidate codepoint.
    pub codepoint: u32,
    /// The owning Unicode block.
    pub block: Block,
    /// Conservative ASCII substitute.
    pub fallback: &'static str,
}

impl Candidate {
    /// The candidate as a `char`.
    #[must_use]
    pub fn glyph(&self) -> char {
        char::from_u32(self.codepoint).expect("catalog codepoints are valid chars")
    }

    /// SCREAMING_SNAKE constant name for generated manifests.
    #[must_use]
    pub fn const_name(&self) -> String {
        // Ids like `box_2502` -> `BOX_2502`.
        self.id.to_ascii_uppercase()
    }
}

fn box_drawing_fallback(cp: u32) -> &'static str {
    match cp {
        0x2500 => "-",
        0x2502 => "|",
        0x250C..=0x254B => "+", // junctions and corners
        0x2550 => "=",
        0x2551 => "|",
        0x2552..=0x256C => "+",
        0x2571 => "/",
        0x2572 => "\\",
        0x2573 => "x",
        _ => "+",
    }
}

fn block_elements_fallback(cp: u32) -> &'static str {
    match cp {
        0x2580 => "^",
        0x2584 => "_",
        0x2588 => "#",
        0x258C => "[",
        0x2590 => "]",
        0x2591 => ".",
        0x2592 => ":",
        0x2593 => "#",
        _ => "#",
    }
}

fn arrows_fallback(cp: u32) -> &'static str {
    match cp {
        0x2190 => "<",
        0x2191 => "^",
        0x2192 => ">",
        0x2193 => "v",
        0x2194 => "-",
        _ => "*",
    }
}

fn geometric_shapes_fallback(cp: u32) -> &'static str {
    match cp {
        0x25A0 | 0x25A1 => "#",
        0x25B2 => "^",
        0x25BC => "v",
        0x25B6 => ">",
        0x25C0 => "<",
        0x25CB | 0x25CF => "o",
        0x25C6 => "*",
        _ => "#",
    }
}

fn misc_symbols_fallback(cp: u32) -> &'static str {
    match cp {
        0x2605 | 0x2606 => "*",
        0x2611 => "v",
        0x2612 => "x",
        0x2699 => "@",
        0x26A0 => "!",
        _ => "*",
    }
}

fn dingbats_fallback(cp: u32) -> &'static str {
    match cp {
        0x2713 | 0x2714 => "v",
        0x2715..=0x2718 => "x",
        0x2702 => "X",
        0x2794 | 0x27A2 => ">",
        _ => "*",
    }
}

fn fallback_for(block: Block, cp: u32) -> &'static str {
    match block {
        Block::Ascii => "",
        Block::Arrows => arrows_fallback(cp),
        Block::BoxDrawing => box_drawing_fallback(cp),
        Block::BlockElements => block_elements_fallback(cp),
        Block::GeometricShapes => geometric_shapes_fallback(cp),
        Block::MiscSymbols => misc_symbols_fallback(cp),
        Block::Dingbats => dingbats_fallback(cp),
        Block::Braille => "*",
    }
}

fn build_candidates() -> Vec<Candidate> {
    let mut out = Vec::with_capacity(2048);
    for &(start, end, block) in &[
        (ASCII_START, ASCII_END, Block::Ascii),
        (ARROWS_START, ARROWS_END, Block::Arrows),
        (BOX_DRAWING_START, BOX_DRAWING_END, Block::BoxDrawing),
        (
            BLOCK_ELEMENTS_START,
            BLOCK_ELEMENTS_END,
            Block::BlockElements,
        ),
        (
            GEOMETRIC_SHAPES_START,
            GEOMETRIC_SHAPES_END,
            Block::GeometricShapes,
        ),
        (MISC_SYMBOLS_START, MISC_SYMBOLS_END, Block::MiscSymbols),
        (DINGBATS_START, DINGBATS_END, Block::Dingbats),
        (BRAILLE_START, BRAILLE_END, Block::Braille),
    ] {
        for cp in start..=end {
            out.push(Candidate {
                id: format!("{}_{}", block.prefix(), format_args!("{cp:04X}")),
                codepoint: cp,
                block,
                fallback: fallback_for(block, cp),
            });
        }
    }
    out.sort_by_key(|c| c.codepoint);
    out
}

/// The full candidate registry, sorted by codepoint.
#[must_use]
pub fn candidates() -> &'static [Candidate] {
    static CANDIDATES: OnceLock<Vec<Candidate>> = OnceLock::new();
    CANDIDATES.get_or_init(build_candidates)
}

/// Total number of candidates.
#[must_use]
pub fn total_count() -> usize {
    candidates().len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn count_block(block: Block) -> usize {
        candidates().iter().filter(|c| c.block == block).count()
    }

    #[test]
    fn exact_per_block_counts() {
        assert_eq!(count_block(Block::Ascii), 94); // 0x21..=0x7E
        assert_eq!(count_block(Block::Arrows), 112); // 0x2190..=0x21FF
        assert_eq!(count_block(Block::BoxDrawing), 128); // 0x2500..=0x257F
        assert_eq!(count_block(Block::BlockElements), 32); // 0x2580..=0x259F
        assert_eq!(count_block(Block::GeometricShapes), 96); // 0x25A0..=0x25FF
        assert_eq!(count_block(Block::MiscSymbols), 256); // 0x2600..=0x26FF
        assert_eq!(count_block(Block::Dingbats), 192); // 0x2700..=0x27BF
        assert_eq!(count_block(Block::Braille), 255); // 0x2801..=0x28FF
        assert_eq!(total_count(), 1165);
    }

    #[test]
    fn blank_codepoints_excluded() {
        for c in candidates() {
            assert_ne!(c.codepoint, 0x20, "U+0020 must be excluded");
            assert_ne!(
                c.codepoint, 0x2800,
                "U+2800 BRAILLE PATTERN BLANK must be excluded"
            );
        }
        assert!(candidates().iter().any(|c| c.codepoint == 0x21));
        assert!(candidates().iter().any(|c| c.codepoint == 0x2801));
        assert!(candidates().iter().any(|c| c.codepoint == 0x28FF));
    }

    #[test]
    fn sorted_unique_and_well_formed() {
        let all = candidates();
        let mut seen_cp = HashSet::new();
        let mut seen_id = HashSet::new();
        for pair in all.windows(2) {
            assert!(pair[0].codepoint < pair[1].codepoint);
        }
        for c in all {
            assert!(seen_cp.insert(c.codepoint), "duplicate codepoint");
            assert!(seen_id.insert(c.id.as_str()), "duplicate id");
            assert!(char::from_u32(c.codepoint).is_some());
            if c.block == Block::Ascii {
                // ASCII glyphs are their own fallback; empty is correct.
                assert!(c.fallback.is_empty());
            } else {
                assert!(!c.fallback.is_empty());
            }
            assert_eq!(
                c.id,
                format!(
                    "{}_{}",
                    c.block.prefix(),
                    format_args!("{:04X}", c.codepoint)
                )
            );
        }
    }

    #[test]
    fn glyphs_roundtrip_and_fallbacks_curated() {
        let by_cp = |cp: u32| {
            candidates()
                .iter()
                .find(|c| c.codepoint == cp)
                .expect("known codepoint")
        };
        assert_eq!(by_cp(0x2502).glyph(), '│');
        assert_eq!(by_cp(0x2502).fallback, "|");
        assert_eq!(by_cp(0x2588).fallback, "#");
        assert_eq!(by_cp(0x2801).fallback, "*");
        assert_eq!(by_cp(0x0041).fallback, "");
        assert_eq!(by_cp(0x2190).fallback, "<");
        assert_eq!(by_cp(0x2192).fallback, ">");
        assert_eq!(by_cp(0x25B2).fallback, "^");
        assert_eq!(by_cp(0x25CF).fallback, "o");
        assert_eq!(by_cp(0x26A0).fallback, "!");
        assert_eq!(by_cp(0x2714).fallback, "v");
        assert_eq!(by_cp(0x2717).fallback, "x");
    }

    #[test]
    fn const_names_are_screaming_snake() {
        let c = candidates().iter().find(|c| c.codepoint == 0x2502).unwrap();
        assert_eq!(c.const_name(), "BOX_2502");
    }
}
