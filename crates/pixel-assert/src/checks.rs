//! Pass-2 glyph assertions over fixed-grid cell boxes.
//!
//! * **Visibility** uses block-aware adaptive thresholds: ASCII / Box Drawing /
//!   Block Elements require ≥ 2 % ink coverage, Braille only ≥ 0.2 % (a
//!   single-dot pattern like `U+2801` covers under 1 % of its cell). The
//!   tofu-outline heuristic is exempt for Braille — a lone dot is inherently
//!   a hollow outline.
//! * **Bleed** compares sentinel B's crop against the clean reference from
//!   the control row and scans the icon→B gutter for intrusion.

use crate::geom::{Band, Calibration, CellBox, FgModel};
use icon_catalog::Block;
use image::GrayImage;

/// Assertion thresholds (defaults per plan; CLI-overridable).
#[derive(Clone, Copy, Debug)]
pub struct Thresholds {
    /// Minimum fg ratio for ASCII/Box Drawing/Block Elements cells.
    pub eps_default: f64,
    /// Minimum fg ratio for Braille cells (single dots cover < 1 %).
    pub eps_braille: f64,
    /// Symmetric-difference tolerance vs the reference B crop.
    pub bleed_max: f64,
    /// Tofu detection: bbox must occupy at least this share of the cell box.
    pub tofu_bbox_min_ratio: f64,
    /// Tofu detection: maximum interior fill ratio of the outline bbox.
    pub tofu_interior_max_ratio: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            eps_default: 0.02,
            eps_braille: 0.002,
            bleed_max: 0.35,
            tofu_bbox_min_ratio: 0.55,
            tofu_interior_max_ratio: 0.15,
        }
    }
}

/// Why a candidate's icon cell failed visibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvisibleReason {
    Blank,
    TofuBox,
}

#[must_use]
fn eps_for(block: Block, t: &Thresholds) -> f64 {
    if matches!(block, Block::Braille) {
        t.eps_braille
    } else {
        t.eps_default
    }
}

fn fg_count(gray: &GrayImage, model: &FgModel, b: &CellBox) -> u64 {
    let mut n = 0u64;
    for y in b.top..=b.bottom.min(gray.height().saturating_sub(1)) {
        for x in b.left..=b.right.min(gray.width().saturating_sub(1)) {
            if model.is_fg(gray, x, y) {
                n += 1;
            }
        }
    }
    n
}

/// Visibility check for one icon cell.
pub fn check_visibility(
    gray: &GrayImage,
    model: &FgModel,
    icon_box: &CellBox,
    block: Block,
    t: &Thresholds,
) -> Result<(), InvisibleReason> {
    let area = icon_box.area();
    if area == 0 {
        return Err(InvisibleReason::Blank);
    }
    let ratio = (fg_count(gray, model, icon_box) as f64) / (area as f64);
    if ratio < eps_for(block, t) {
        return Err(InvisibleReason::Blank);
    }
    // Tofu-outline heuristic (non-Braille only): the missing-glyph box renders
    // as a thin hollow rectangle. A lone Braille dot is inherently hollow and
    // tiny, so it must never be evaluated here.
    if !matches!(block, Block::Braille) && looks_like_tofu(gray, model, icon_box, t) {
        return Err(InvisibleReason::TofuBox);
    }
    Ok(())
}

fn looks_like_tofu(gray: &GrayImage, model: &FgModel, icon_box: &CellBox, t: &Thresholds) -> bool {
    // Bounding box of ink inside the cell crop.
    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;
    for y in icon_box.top..=icon_box.bottom.min(gray.height() - 1) {
        for x in icon_box.left..=icon_box.right.min(gray.width() - 1) {
            if model.is_fg(gray, x, y) {
                min_x = Some(min_x.map_or(x, |v: u32| v.min(x)));
                max_x = Some(max_x.map_or(x, |v: u32| v.max(x)));
                min_y = Some(min_y.map_or(y, |v: u32| v.min(y)));
                max_y = Some(max_y.map_or(y, |v: u32| v.max(y)));
            }
        }
    }
    let (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) = (min_x, max_x, min_y, max_y) else {
        return false;
    };
    let bw = f64::from(max_x - min_x + 1);
    let bh = f64::from(max_y - min_y + 1);
    let cell_w = f64::from(icon_box.right - icon_box.left + 1);
    let cell_h = f64::from(icon_box.bottom - icon_box.top + 1);

    // The outline should span most of the cell…
    if bw / cell_w < t.tofu_bbox_min_ratio || bh / cell_h < t.tofu_bbox_min_ratio {
        return false;
    }

    // …with a mostly-empty interior and dense perimeter.
    let iw = i64::from(max_x) - i64::from(min_x) + 1;
    let ih = i64::from(max_y) - i64::from(min_y) + 1;
    if iw <= 2 || ih <= 2 {
        return false;
    }
    let interior_area = ((iw - 2) * (ih - 2)) as f64;

    let ix0 = i64::from(min_x) + 1;
    let ix1 = i64::from(max_x) - 1;
    let iy0 = i64::from(min_y) + 1;
    let iy1 = i64::from(max_y) - 1;
    let mut interior_ink = 0u64;
    for y in iy0..=iy1 {
        for x in ix0..=ix1 {
            if model.is_fg(gray, x as u32, y as u32) {
                interior_ink += 1;
            }
        }
    }
    (interior_ink as f64 / interior_area) <= t.tofu_interior_max_ratio
}

/// Result of the bleed check on sentinel B.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BleedVerdict {
    Clean,
    Intrusion,
}

/// Compare the candidate's B-cell crop against the control-row reference crop
/// and scan the icon→B gutter for intrusion.
///
/// `reference_b` is harvested once per page from the control band; solid
/// glyphs that legitimately fill their own cell never enter B's central crop,
/// while a wide (2-cell) render physically does.
pub fn check_bleed(
    gray: &GrayImage,
    model: &FgModel,
    cal: &Calibration,
    band: &Band,
    reference_b: &[bool],
    t: &Thresholds,
) -> BleedVerdict {
    let b_box = cal.cell_box(4, band);

    // Gutter between the icon's central crop and B's central crop. The scan
    // runs strictly INSIDE the two rounded boundaries: the icon's crop-right
    // column and B's crop-left column are shared edges with legitimate
    // glyph/AA ink and must not count as intrusion.
    let (g0, g1) = cal.gutter_span();
    let gutter_left = (g0.round() as i64).max(0) + 1;
    let gutter_right = (g1.round() as i64).min(i64::from(gray.width().saturating_sub(1))) - 1;
    if gutter_left <= gutter_right {
        for y in band.top..=band.bottom.min(gray.height() - 1) {
            for x in gutter_left..=gutter_right {
                if model.is_fg(gray, x as u32, y) {
                    return BleedVerdict::Intrusion;
                }
            }
        }
    }

    // Symmetric difference against the reference B mask.
    let mask = crop_mask(gray, model, &b_box);
    let mut diff = 0usize;
    for (a, r) in mask.iter().zip(reference_b.iter()) {
        if a != r {
            diff += 1;
        }
    }
    let denom = reference_b.len().max(1);
    if diff as f64 / denom as f64 > t.bleed_max {
        return BleedVerdict::Intrusion;
    }
    BleedVerdict::Clean
}

/// Boolean fg mask of a cell box, row-major.
#[must_use]
pub fn crop_mask(gray: &GrayImage, model: &FgModel, b: &CellBox) -> Vec<bool> {
    let mut out = Vec::with_capacity(b.area() as usize);
    for y in b.top..=b.bottom.min(gray.height().saturating_sub(1)) {
        for x in b.left..=b.right.min(gray.width().saturating_sub(1)) {
            out.push(model.is_fg(gray, x, y));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    const BG: u8 = 10;
    const FG: u8 = 220;

    struct Canvas {
        img: GrayImage,
        model: FgModel,
    }

    impl Canvas {
        fn new(w: u32, h: u32) -> Self {
            let mut img = GrayImage::new(w, h);
            for y in 0..h {
                for x in 0..w {
                    img.put_pixel(x, y, Luma([BG]));
                }
            }
            Self {
                img,
                model: FgModel {
                    bg: f32::from(BG),
                    delta: 60.0,
                },
            }
        }

        fn rect(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, val: u8) {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    self.img.put_pixel(x, y, Luma([val]));
                }
            }
        }

        fn cal(&self, origin_x: f64, pitch: f64) -> Calibration {
            Calibration { origin_x, pitch }
        }
    }

    #[test]
    fn braille_single_dot_passes_adaptive_threshold_and_fails_default() {
        // 8x20 central crop: a single-pixel dot is 0.625% — above the Braille
        // epsilon (0.002) but far below the default (0.02).
        let mut c = Canvas::new(60, 24);
        let band = Band { top: 2, bottom: 21 };
        let cal = c.cal(5.0, 10.0);
        let icon = cal.cell_box(3, &band); // center 35 → box 32..39
        c.rect(icon.left + 3, 12, icon.left + 3, 12, FG);

        assert!(
            check_visibility(
                &c.img,
                &c.model,
                &icon,
                Block::Braille,
                &Thresholds::default()
            )
            .is_ok()
        );
        assert_eq!(
            check_visibility(
                &c.img,
                &c.model,
                &icon,
                Block::Ascii,
                &Thresholds::default()
            ),
            Err(InvisibleReason::Blank)
        );
    }

    #[test]
    fn blank_cell_fails_every_block() {
        let c = Canvas::new(60, 24);
        let band = Band { top: 2, bottom: 21 };
        let cal = c.cal(5.0, 10.0);
        let icon = cal.cell_box(3, &band);
        for block in [
            Block::Ascii,
            Block::BoxDrawing,
            Block::BlockElements,
            Block::Braille,
        ] {
            assert_eq!(
                check_visibility(&c.img, &c.model, &icon, block, &Thresholds::default()),
                Err(InvisibleReason::Blank)
            );
        }
    }

    #[test]
    fn tofu_outline_detected_as_invisible() {
        let mut c = Canvas::new(60, 24);
        // Hollow rectangle spanning most of the cell.
        let band = Band { top: 2, bottom: 21 };
        let cal = c.cal(5.0, 10.0);
        let icon = cal.cell_box(3, &band);
        let (x0, x1) = (icon.left, icon.right);
        let (y0, y1) = (icon.top + 1, icon.bottom - 1);
        c.rect(x0, y0, x1, y1, FG);
        // Clear everything strictly inside the outline: only the 1 px ring
        // remains as ink.
        c.rect(x0 + 1, y0 + 1, x1 - 1, y1 - 1, BG);
        assert_eq!(
            check_visibility(
                &c.img,
                &c.model,
                &icon,
                Block::BoxDrawing,
                &Thresholds::default()
            ),
            Err(InvisibleReason::TofuBox)
        );
    }

    #[test]
    fn wide_glyph_bleeds_into_gutter() {
        let mut c = Canvas::new(120, 24);
        let band = Band { top: 2, bottom: 21 };
        let cal = c.cal(10.0, 14.0);
        // Icon cell centered at col 3 (=52): fill through the gutter into B.
        c.rect(46, 4, 66, 20, FG);
        let reference_b = vec![false; 200];
        assert_eq!(
            check_bleed(
                &c.img,
                &c.model,
                &cal,
                &band,
                &reference_b,
                &Thresholds::default()
            ),
            BleedVerdict::Intrusion
        );
    }

    #[test]
    fn solid_block_filling_own_cell_is_clean() {
        let mut c = Canvas::new(120, 24);
        let band = Band { top: 2, bottom: 21 };
        let cal = c.cal(10.0, 14.0);
        // █ fills exactly the central crop region of col 3, stopping before
        // the shared rounded boundary with the gutter.
        let box_l = (cal.cell_center(3) - cal.pitch * 0.35).round() as u32;
        let box_r = (cal.cell_center(3) + cal.pitch * 0.35).round() as u32;
        c.rect(box_l, 4, box_r.saturating_sub(1), 20, FG);
        let reference_b = vec![false; 200];
        assert_eq!(
            check_bleed(
                &c.img,
                &c.model,
                &cal,
                &band,
                &reference_b,
                &Thresholds::default()
            ),
            BleedVerdict::Clean
        );
    }
}
