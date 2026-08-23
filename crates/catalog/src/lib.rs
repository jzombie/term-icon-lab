//! Candidate registry of single-cell Unicode codepoints for term-icon.
//!
//! The catalog enumerates every candidate icon that enters the verification
//! pipeline, restricted to Unicode blocks whose East Asian Width properties
//! avoid inherent width ambiguity:
//!
//! | Block | Range | Notes |
//! |---|---|---|
//! | Basic ASCII | `U+0021–U+007E` | `U+0020` excluded (blank render) |
//! | Box Drawing | `U+2500–U+257F` | |
//! | Block Elements | `U+2580–U+259F` | |
//! | Braille Patterns | `U+2801–U+28FF` | `U+2800 BRAILLE PATTERN BLANK` excluded (blank render) |

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Start of the Basic ASCII candidate range (`U+0021`, exclusive of space).
pub const ASCII_START: u32 = 0x21;
/// End of the Basic ASCII candidate range.
pub const ASCII_END: u32 = 0x7E;
/// Start of the Box Drawing candidate range.
pub const BOX_DRAWING_START: u32 = 0x2500;
/// End of the Box Drawing candidate range.
pub const BOX_DRAWING_END: u32 = 0x257F;
/// Start of the Block Elements candidate range.
pub const BLOCK_ELEMENTS_START: u32 = 0x2580;
/// End of the Block Elements candidate range.
pub const BLOCK_ELEMENTS_END: u32 = 0x259F;
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
    /// Box Drawing (`U+2500–U+257F`).
    BoxDrawing,
    /// Block Elements (`U+2580–U+259F`).
    BlockElements,
    /// Braille Patterns (`U+2801–U+28FF`).
    Braille,
}

impl Block {
    /// All blocks, in manifest order.
    pub const ALL: [Block; 4] = [
        Block::Ascii,
        Block::BoxDrawing,
        Block::BlockElements,
        Block::Braille,
    ];

    /// Lowercase identifier prefix used in catalog ids and generated modules.
    pub fn prefix(self) -> &'static str {
        match self {
            Block::Ascii => "ascii",
            Block::BoxDrawing => "box",
            Block::BlockElements => "block",
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

fn fallback_for(block: Block, cp: u32) -> &'static str {
    match block {
        Block::Ascii => "",
        Block::BoxDrawing => box_drawing_fallback(cp),
        Block::BlockElements => block_elements_fallback(cp),
        Block::Braille => "*",
    }
}

fn build_candidates() -> Vec<Candidate> {
    let mut out = Vec::with_capacity(512);
    for &(start, end, block) in &[
        (ASCII_START, ASCII_END, Block::Ascii),
        (BOX_DRAWING_START, BOX_DRAWING_END, Block::BoxDrawing),
        (
            BLOCK_ELEMENTS_START,
            BLOCK_ELEMENTS_END,
            Block::BlockElements,
        ),
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
        assert_eq!(count_block(Block::BoxDrawing), 128); // 0x2500..=0x257F
        assert_eq!(count_block(Block::BlockElements), 32); // 0x2580..=0x259F
        assert_eq!(count_block(Block::Braille), 255); // 0x2801..=0x28FF
        assert_eq!(total_count(), 509);
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
    }

    #[test]
    fn const_names_are_screaming_snake() {
        let c = candidates().iter().find(|c| c.codepoint == 0x2502).unwrap();
        assert_eq!(c.const_name(), "BOX_2502");
    }
}
