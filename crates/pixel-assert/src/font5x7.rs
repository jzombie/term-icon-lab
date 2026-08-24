//! Embedded 5×7 bitmap font for specimen-chart labels.
//!
//! Exactly the character domain needed by `"U+XXXX"` activation labels —
//! 19 glyphs: `' '`, `'+'`, `'0'..='9'`, `'A'..'F'`, `'U'`. Each glyph is
//! authored as seven 5-character rows (`#` = ink) so the shapes stay
//! human-readable and testable in source form.

use image::{Rgb, RgbImage};

/// Bitmap column count per glyph.
pub(crate) const FONT_W: u32 = 5;
/// Bitmap row count per glyph.
pub(crate) const FONT_H: u32 = 7;

/// One glyph: `(char, [row_0 .. row_6])`, rows are `"#####"`-style strings.
pub(crate) struct Glyph {
    pub(crate) ch: char,
    pub(crate) rows: [&'static str; FONT_H as usize],
}

macro_rules! glyph {
    ($ch:expr, [$($row:expr),+ $(,)?]) => {
        Glyph { ch: $ch, rows: [$($row),+] }
    };
}

/// The full label character domain — 19 entries, no more, no less.
pub(crate) const FONT5X7: &[Glyph] = &[
    glyph!(
        ' ',
        [
            ".....", ".....", ".....", ".....", ".....", ".....", "....."
        ]
    ),
    glyph!(
        '+',
        [
            "..#..", "..#..", "#####", "..#..", "..#..", ".....", "....."
        ]
    ),
    glyph!(
        '0',
        [
            ".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."
        ]
    ),
    glyph!(
        '1',
        [
            "..#..", ".##..", "..#..", "..#..", "..#..", "..#..", ".###."
        ]
    ),
    glyph!(
        '2',
        [
            ".###.", "#...#", "....#", "...#.", "..#..", ".#...", "#####"
        ]
    ),
    glyph!(
        '3',
        [
            ".###.", "#...#", "....#", "..##.", "....#", "#...#", ".###."
        ]
    ),
    glyph!(
        '4',
        [
            "...#.", "..##.", ".#.#.", "#..#.", "#####", "...#.", "...#."
        ]
    ),
    glyph!(
        '5',
        [
            "#####", "#....", "####.", "....#", "....#", "#...#", ".###."
        ]
    ),
    glyph!(
        '6',
        [
            "..##.", ".#...", "#....", "####.", "#...#", "#...#", ".###."
        ]
    ),
    glyph!(
        '7',
        [
            "#####", "....#", "...#.", "..#..", ".#...", ".#...", ".#..."
        ]
    ),
    glyph!(
        '8',
        [
            ".###.", "#...#", "#...#", ".###.", "#...#", "#...#", ".###."
        ]
    ),
    glyph!(
        '9',
        [
            ".###.", "#...#", "#...#", ".####", "....#", "...#.", ".##.."
        ]
    ),
    glyph!(
        'A',
        [
            "..#..", ".#.#.", "#...#", "#...#", "#####", "#...#", "#...#"
        ]
    ),
    glyph!(
        'B',
        [
            "####.", "#...#", "#...#", "####.", "#...#", "#...#", "####."
        ]
    ),
    glyph!(
        'C',
        [
            ".###.", "#...#", "#....", "#....", "#....", "#...#", ".###."
        ]
    ),
    glyph!(
        'D',
        [
            "####.", "#...#", "#...#", "#...#", "#...#", "#...#", "####."
        ]
    ),
    glyph!(
        'E',
        [
            "#####", "#....", "#....", "####.", "#....", "#....", "#####"
        ]
    ),
    glyph!(
        'F',
        [
            "#####", "#....", "#....", "####.", "#....", "#....", "#...."
        ]
    ),
    glyph!(
        'U',
        [
            "#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."
        ]
    ),
];

/// Look up a glyph; unknown characters return `None` (rendered as a blank
/// advance by [`draw_text`] — never a panic).
#[must_use]
pub(crate) fn glyph(ch: char) -> Option<&'static Glyph> {
    FONT5X7.iter().find(|g| g.ch == ch)
}

/// Rendered width of an `n`-character string at `scale`: n glyphs of
/// `FONT_W*scale` separated by `scale` gaps, with **no trailing gap**
/// (`(n*6 − 1) * scale`).
#[must_use]
pub(crate) fn text_width(n_chars: usize, scale: u32) -> u32 {
    if n_chars == 0 {
        0
    } else {
        (n_chars as u32 * (FONT_W + 1) - 1) * scale
    }
}

/// Draw `s` with its top-left corner at `(x, y)`. Character advance is
/// `(FONT_W + 1) * scale` — the trailing 1-px unscaled gap keeps adjacent
/// glyphs from colliding at any scale. Unknown characters advance blankly.
pub(crate) fn draw_text(img: &mut RgbImage, x: i64, y: i64, s: &str, scale: u32, color: Rgb<u8>) {
    let mut pen_x = x;
    for ch in s.chars() {
        if let Some(g) = glyph(ch) {
            for (ry, row) in g.rows.iter().enumerate() {
                for (rx, px) in row.bytes().enumerate() {
                    if px != b'#' {
                        continue;
                    }
                    let bx = pen_x + (rx as u32 * scale) as i64;
                    let by = y + (ry as u32 * scale) as i64;
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let (px_, py_) = (bx + dx as i64, by + dy as i64);
                            if px_ >= 0 && py_ >= 0 {
                                let (ux, uy) = (px_ as u32, py_ as u32);
                                if ux < img.width() && uy < img.height() {
                                    img.put_pixel(ux, uy, color);
                                }
                            }
                        }
                    }
                }
            }
        }
        pen_x += (FONT_W + 1) as i64 * scale as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rendered(s: &str, scale: u32) -> RgbImage {
        let w = text_width(s.chars().count(), scale);
        let mut img = RgbImage::from_pixel(w.max(1), FONT_H * scale, Rgb([0, 0, 0]));
        draw_text(&mut img, 0, 0, s, scale, Rgb([200, 200, 200]));
        img
    }

    #[test]
    fn font_domain_is_exactly_nineteen_label_glyphs() {
        assert_eq!(FONT5X7.len(), 19);
        let mut chars: Vec<char> = FONT5X7.iter().map(|g| g.ch).collect();
        chars.sort_unstable();
        let mut expected: Vec<char> = vec![' ', '+', 'U'];
        expected.extend('0'..='9');
        expected.extend('A'..='F');
        expected.sort_unstable();
        assert_eq!(chars, expected, "{chars:?}");
        // Every row string is 5 wide, 7 tall.
        for g in FONT5X7 {
            assert_eq!(g.rows.len(), FONT_H as usize);
            assert!(g.rows.iter().all(|r| r.len() == FONT_W as usize));
        }
    }

    #[test]
    fn u_renders_expected_pattern() {
        let img = rendered("U", 1);
        // Top row: #...#, bottom row: .###.
        let ink = |x: u32, y: u32| img.get_pixel(x, y)[0] != 0;
        assert!(ink(0, 0) && !ink(1, 0) && ink(4, 0));
        assert!(ink(1, 6) && ink(2, 6) && ink(3, 6));
        assert!(!ink(0, 6) && !ink(4, 6));
    }

    #[test]
    fn stride_leaves_gap_between_adjacent_glyphs() {
        let img = rendered("UU", 2);
        // Advance is 6*scale = 12: glyph ink occupies cols 0..=9, the gap
        // columns 10 and 11 must stay background at every row.
        for y in 0..img.height() {
            assert_eq!(img.get_pixel(10, y)[0], 0, "gap column 10 blank");
            assert_eq!(img.get_pixel(11, y)[0], 0, "gap column 11 blank");
        }
        assert_eq!(img.width(), text_width(2, 2), "(2*6-1)*2 = 22");
    }

    #[test]
    fn unknown_char_advances_blank_without_panic() {
        let img = rendered("U?", 1);
        // '?' contributes only its advance: total width covers 2 chars, but
        // the second slot is entirely background.
        assert_eq!(img.width(), text_width(2, 1));
        for y in 0..img.height() {
            for x in 6..img.width() {
                assert_eq!(img.get_pixel(x, y)[0], 0);
            }
        }
    }
}
