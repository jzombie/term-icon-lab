//! Fixed-grid geometry slicing (plan §5).
//!
//! Cell boxes are derived **arithmetically** from the control-row anchors —
//! never from glyph-ink connectivity, which collapses when solid Block
//! Elements touch their sentinels. All spatial math is `f64` end-to-end; the
//! only integer conversion is the final `.round()` at the pixel-slice
//! boundary, so non-integer cell widths (e.g. 9.33 px) cannot accumulate
//! quantization drift across a line.

use image::{GrayImage, Luma};

/// Foreground/background separation model.
#[derive(Clone, Copy, Debug)]
pub struct FgModel {
    /// Estimated background luminance.
    pub bg: f32,
    /// Absolute luminance distance that counts as foreground ink.
    pub delta: f32,
}

impl FgModel {
    #[must_use]
    pub fn is_fg(&self, gray: &GrayImage, x: u32, y: u32) -> bool {
        let p = gray.get_pixel(x, y)[0];
        (f32::from(p) - self.bg).abs() > self.delta
    }
}

/// Estimate background luminance as the modal grayscale value.
///
/// The terminal background dominates any capture — the matrix occupies a
/// small fraction of the frame. Frame sampling is unreliable here: window
/// captures (macOS Terminal.app) carry title-bar chrome, alpha padding, and
/// rounded corners whose pixels are not bare SGR background.
#[must_use]
pub fn estimate_background(gray: &GrayImage) -> f32 {
    let mut hist = [0u64; 256];
    for p in gray.pixels() {
        hist[p[0] as usize] += 1;
    }
    let (val, _) = hist
        .iter()
        .enumerate()
        .max_by_key(|&(_, n)| *n)
        .unwrap_or((0, &0));
    f32::from(val as u8)
}

/// Longest foreground run along an axis spanning more than this fraction of
/// the image is window chrome — borders, scrollbars, title-bar rules — not
/// glyph ink, which is bounded by the line pitch.
const STRUCTURAL_RUN_RATIO: f64 = 0.3;

/// Blank out window chrome that survives capture: full-height vertical lines
/// (window borders, scrollbars) and full-width horizontal lines (title bars,
/// separator rules). Without this, chrome bridges the gaps between text rows
/// and band detection merges the whole frame into one band.
#[must_use]
pub fn suppress_structural_lines(gray: &GrayImage, model: &FgModel) -> GrayImage {
    let (w, h) = gray.dimensions();
    let mut out = gray.clone();
    let bg = model.bg.round().clamp(0.0, 255.0) as u8;
    let limit_h = (f64::from(h) * STRUCTURAL_RUN_RATIO) as u64;
    let limit_w = (f64::from(w) * STRUCTURAL_RUN_RATIO) as u64;

    for x in 0..w {
        let mut run = 0u64;
        let mut worst = 0u64;
        for y in 0..h {
            run = if model.is_fg(gray, x, y) { run + 1 } else { 0 };
            worst = worst.max(run);
        }
        if worst > limit_h {
            for y in 0..h {
                out.put_pixel(x, y, Luma([bg]));
            }
        }
    }
    for y in 0..h {
        let mut run = 0u64;
        let mut worst = 0u64;
        for x in 0..w {
            run = if model.is_fg(gray, x, y) { run + 1 } else { 0 };
            worst = worst.max(run);
        }
        if worst > limit_w {
            for x in 0..w {
                out.put_pixel(x, y, Luma([bg]));
            }
        }
    }
    out
}

/// A horizontal run of rows containing foreground ink (inclusive bounds).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Band {
    pub top: u32,
    pub bottom: u32,
}

/// Detect rendered rows via the boundary-bar strip: the leftmost column
/// cluster whose ink **run count** reaches `expected_rows` (a bar produces
/// one short run per rendered row; braille dot columns fragment into more
/// runs but sit right of the bar, and columns of stacked full-cell glyphs
/// produce one long run).
///
/// The bars are vertically isolated by the line pitch — unlike full-cell
/// glyphs (█), whose ink bridges adjacent rows and defeats whole-row
/// projection.
#[must_use]
pub fn find_text_bands(gray: &GrayImage, model: &FgModel, expected_rows: usize) -> Vec<Band> {
    if expected_rows == 0 {
        return Vec::new();
    }
    let (w, h) = gray.dimensions();
    let mut run_counts = vec![0u32; w as usize];
    for x in 0..w {
        let mut len = 0u32;
        for y in 0..h {
            if model.is_fg(gray, x, y) {
                len += 1;
            } else {
                if len >= 2 {
                    run_counts[x as usize] += 1;
                }
                len = 0;
            }
        }
        if len >= 2 {
            run_counts[x as usize] += 1;
        }
    }
    let high = (expected_rows * 3 / 4).max(1) as u32;
    let low = (expected_rows / 4).max(1) as u32;
    let Some(x0) = run_counts.iter().position(|&r| r >= high) else {
        return Vec::new();
    };
    let mut x1 = x0;
    while x1 + 1 < w as usize && run_counts[x1 + 1] >= low && x1 - x0 < 5 {
        x1 += 1;
    }

    let mut bands = Vec::new();
    let mut current: Option<u32> = None;
    for y in 0..h {
        let ink = (x0 as u32..=x1 as u32)
            .filter(|&x| model.is_fg(gray, x, y))
            .count();
        if ink >= 1 {
            if current.is_none() {
                current = Some(y);
            }
        } else if let Some(top) = current.take() {
            bands.push(Band {
                top,
                bottom: y.saturating_sub(1),
            });
        }
    }
    if let Some(top) = current {
        bands.push(Band {
            top,
            bottom: h.saturating_sub(1),
        });
    }
    bands
}

/// When chrome remnants survive suppression as extra bands, keep the
/// consecutive run of `expected` bands with the most uniform pitch — the
/// rendered grid is perfectly regular, chrome is not.
#[must_use]
pub fn select_grid_bands(bands: &[Band], expected: usize) -> Vec<Band> {
    if expected == 0 || bands.len() <= expected {
        return bands.to_vec();
    }
    let mut best: Option<(f64, usize)> = None;
    for s in 0..=(bands.len() - expected) {
        let grp = &bands[s..s + expected];
        let pitches: Vec<f64> = grp
            .windows(2)
            .map(|p| f64::from(p[1].top) - f64::from(p[0].top))
            .collect();
        if pitches.is_empty() {
            if best.is_none() {
                best = Some((0.0, s));
            }
            continue;
        }
        let mean = pitches.iter().sum::<f64>() / pitches.len() as f64;
        let var = pitches.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / pitches.len() as f64;
        if best.is_none() || var < best.expect("checked above").0 {
            best = Some((var, s));
        }
    }
    match best {
        Some((_, s)) => bands[s..s + expected].to_vec(),
        None => bands.to_vec(),
    }
}

/// Horizontal cluster `(start_x, end_x)` of ink columns within a band,
/// inclusive on both ends.
pub type Cluster = (u32, u32);

/// Project a band's ink onto the x axis and split into contiguous runs.
#[must_use]
pub fn column_clusters(gray: &GrayImage, model: &FgModel, band: &Band) -> Vec<Cluster> {
    let w = gray.width();
    let mut clusters = Vec::new();
    let mut start: Option<u32> = None;
    for x in 0..w {
        let has_ink = (band.top..=band.bottom).any(|y| model.is_fg(gray, x, y));
        if has_ink {
            if start.is_none() {
                start = Some(x);
            }
        } else if let Some(s) = start.take() {
            clusters.push((s, x.saturating_sub(1)));
        }
    }
    if let Some(s) = start {
        clusters.push((s, w.saturating_sub(1)));
    }
    clusters
}

/// Control-row-derived grid calibration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Calibration {
    /// Center of the column-0 boundary bar.
    pub origin_x: f64,
    /// Horizontal distance between adjacent cell centers.
    pub pitch: f64,
}

/// Geometry failures that invalidate the whole run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryError {
    MissingBars,
    BarsTooWide,
    ImplausiblePitch,
    SentinelSpacingMismatch,
    /// Detected text-band count disagrees with the sidecar-declared row count.
    BandMismatch {
        found: usize,
        expected: usize,
    },
}

/// Calibrate against the control band `| A B |` (pure ASCII, no touching
/// glyphs possible).
///
/// Errors when the two thin boundary bars cannot be identified or the
/// sentinel spacing contradicts the derived pitch.
pub fn calibrate(
    gray: &GrayImage,
    model: &FgModel,
    control: &Band,
) -> Result<Calibration, GeometryError> {
    let clusters = column_clusters(gray, model, control);
    if clusters.len() < 2 {
        return Err(GeometryError::MissingBars);
    }
    let first = clusters[0];
    let last = *clusters.last().expect("len >= 2 checked");
    let origin_x = (f64::from(first.0) + f64::from(first.1)) / 2.0;
    let end_x = (f64::from(last.0) + f64::from(last.1)) / 2.0;
    let pitch = (end_x - origin_x) / 6.0;
    if !(pitch.is_finite() && pitch >= 4.0) {
        return Err(GeometryError::ImplausiblePitch);
    }

    // Boundary bars are single thin glyphs; anything wider means the cluster
    // merged with neighbouring ink and the anchors are untrustworthy.
    let max_bar_w = pitch * 0.75;
    let w_of = |c: &Cluster| f64::from(c.1 - c.0) + 1.0;
    if w_of(&first) > max_bar_w || w_of(&last) > max_bar_w {
        return Err(GeometryError::BarsTooWide);
    }

    // With exactly four clusters we can additionally verify sentinel spacing:
    // A sits at +2P, B at +4P.
    if clusters.len() == 4 {
        let center = |c: &Cluster| (f64::from(c.0) + f64::from(c.1)) / 2.0;
        let a = center(&clusters[1]);
        let b = center(&clusters[2]);
        let tol = pitch * 0.5;
        if ((a - (origin_x + 2.0 * pitch)).abs() > tol)
            || ((b - (origin_x + 4.0 * pitch)).abs() > tol)
        {
            return Err(GeometryError::SentinelSpacingMismatch);
        }
    }

    Ok(Calibration { origin_x, pitch })
}

/// An axis-aligned pixel rectangle produced by one final rounding pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellBox {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl CellBox {
    /// Pixel area of the box.
    #[must_use]
    pub fn area(&self) -> u64 {
        u64::from(self.right.saturating_sub(self.left) + 1)
            * u64::from(self.bottom.saturating_sub(self.top) + 1)
    }
}

impl Calibration {
    /// Center of grid column `col0` (0-based offset from the boundary bar),
    /// in f64 — never truncated.
    #[must_use]
    pub fn cell_center(&self, col0: u16) -> f64 {
        self.origin_x + f64::from(col0) * self.pitch
    }

    /// The central `MARGIN_RATIO` crop of cell `col0`, vertically spanning
    /// `band`. This is the only place f64 becomes integers.
    ///
    /// The central-crop margin keeps antialiased fringes of *neighbouring*
    /// cells out of visibility statistics while leaving solid glyphs fully
    /// covered.
    #[must_use]
    pub fn cell_box(&self, col0: u16, band: &Band) -> CellBox {
        const MARGIN_RATIO: f64 = 0.35;
        let center = self.cell_center(col0);
        let half = self.pitch * MARGIN_RATIO;
        CellBox {
            left: (center - half).round() as u32,
            right: (center + half).round() as u32,
            top: band.top,
            bottom: band.bottom,
        }
    }

    /// The full single-cell extent of `col0` — for *display* crops.
    ///
    /// No safety insets: capture rows sandwich the glyph between
    /// side-bearing ASCII sentinels (`| A<glyph>B |`), so only the glyph's
    /// own ink reaches these bounds, and full-width primitives (`█ ─ ▐`)
    /// span them exactly. Verification keeps its narrower assertion window
    /// ([`Calibration::cell_box`]).
    #[must_use]
    pub fn cell_box_full(&self, col0: u16, band: &Band) -> CellBox {
        let center = self.cell_center(col0);
        CellBox {
            left: (center - self.pitch / 2.0).round() as u32,
            right: (center + self.pitch / 2.0).round() as u32 - 1,
            top: band.top,
            bottom: band.bottom,
        }
    }

    /// The gutter between the icon cell's right **boundary** and sentinel B's
    /// central crop — any ink here means the candidate rendered past its own
    /// cell. The icon's full cell width (out to `center3 + pitch/2`) is
    /// legitimate ink territory: full-width primitives (█ ▐ ─) and wide
    /// letters fill it completely.
    #[must_use]
    pub fn gutter_span(&self) -> (f64, f64) {
        (
            self.cell_center(3) + self.pitch * 0.5,
            self.cell_center(4) - self.pitch * 0.35,
        )
    }

    /// Mirror of [`Calibration::gutter_span`] on the sentinel-A side: from
    /// A's central-crop right edge to the icon cell's **left** boundary.
    /// Overflow toward A is just as much a spatial-contract violation as
    /// overflow toward B.
    #[must_use]
    pub fn left_gutter_span(&self) -> (f64, f64) {
        (
            self.cell_center(2) + self.pitch * 0.35,
            self.cell_center(3) - self.pitch * 0.5,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    /// Draw a synthetic control row with the given geometry.
    fn draw_control_row(
        img: &mut GrayImage,
        origin_x: f64,
        pitch: f64,
        top: u32,
        height: u32,
        bg: u8,
        fg: u8,
    ) {
        let (w, _) = img.dimensions();
        for y in top..top + height {
            for x in 0..w {
                img.put_pixel(x, y, Luma([bg]));
            }
        }
        // Bars at cols 0 and 6 (thin, 2 px wide).
        for col in [0usize, 6] {
            let cx = (origin_x + col as f64 * pitch).round() as i64;
            for dx in [-1i64, 0] {
                let x = cx + dx;
                if x >= 0 && (x as u32) < w {
                    for dy in 0..height {
                        img.put_pixel(x as u32, top + dy, Luma([fg]));
                    }
                }
            }
        }
        // 'A' blob at col 2, 'B' blob at col 4 (solid blocks ~40% of cell).
        for col in [2usize, 4] {
            let cx = (origin_x + col as f64 * pitch).round() as i64;
            let half = (pitch * 0.22) as i64;
            for dy in 0..height {
                for dx in -half..=half {
                    let x = cx + dx;
                    if x >= 0 && (x as u32) < w {
                        img.put_pixel(x as u32, top + dy, Luma([fg]));
                    }
                }
            }
        }
    }

    #[test]
    fn calibrates_integer_pitch() {
        let mut img = GrayImage::new(200, 20);
        draw_control_row(&mut img, 12.0, 10.0, 4, 12, 10, 220);
        let model = FgModel {
            bg: 10.0,
            delta: 60.0,
        };
        let cal = calibrate(&img, &model, &Band { top: 4, bottom: 15 }).expect("calibration");
        assert!((cal.origin_x - 12.0).abs() < 1.0, "{:?}", cal.origin_x);
        assert!((cal.pitch - 10.0).abs() < 1.0, "{:?}", cal.pitch);
    }

    #[test]
    fn fractional_pitch_survives_without_drift() {
        let mut img = GrayImage::new(300, 24);
        draw_control_row(&mut img, 8.0, 9.33, 6, 14, 10, 220);
        let model = FgModel {
            bg: 10.0,
            delta: 60.0,
        };
        let cal = calibrate(&img, &model, &Band { top: 6, bottom: 19 }).unwrap();
        assert!((cal.pitch - 9.33).abs() < 0.5, "{:?}", cal.pitch);

        // Boxes of adjacent cells must never overlap each other's centers,
        // no matter how far down the line they sit.
        for col in 1..30u16 {
            let prev_right = cal.cell_box(col - 1, &Band { top: 6, bottom: 19 }).right;
            let next_center = (cal.cell_center(col)).round() as u32;
            assert!(
                prev_right <= next_center,
                "box {col} drifted past neighbour center"
            );
        }
    }

    #[test]
    fn missing_anchors_are_reported() {
        let mut img = GrayImage::new(100, 20);
        for y in 0..20 {
            for x in 0..100 {
                img.put_pixel(x, y, Luma([10]));
            }
        }
        let model = FgModel {
            bg: 10.0,
            delta: 60.0,
        };
        let band = Band { top: 2, bottom: 10 };
        assert_eq!(
            calibrate(&img, &model, &band),
            Err(GeometryError::MissingBars)
        );
    }

    #[test]
    fn merged_bar_cluster_rejected() {
        // Two wide slabs: clusterable, but far too fat to be boundary bars.
        let mut img = GrayImage::new(160, 16);
        for y in 0..16 {
            for x in 0..30 {
                img.put_pixel(x, y, Luma([220]));
            }
            for x in 100..130 {
                img.put_pixel(x, y, Luma([220]));
            }
        }
        let model = FgModel {
            bg: 10.0,
            delta: 60.0,
        };
        let band = Band { top: 0, bottom: 15 };
        assert_eq!(
            calibrate(&img, &model, &band),
            Err(GeometryError::BarsTooWide)
        );
    }

    /// Draw `rows` text lines of ink height `ink` at pitch `pitch`, plus
    /// optional window chrome: a border rectangle, a title-bar slab, and a
    /// scrollbar thumb.
    fn draw_chrome_capture(
        rows: usize,
        pitch: u32,
        ink: u32,
        border: bool,
        titlebar: bool,
        scrollbar: bool,
    ) -> GrayImage {
        let h = 40 + rows as u32 * pitch + 30;
        let mut img = GrayImage::new(400, h);
        for p in img.pixels_mut() {
            *p = Luma([10]);
        }
        for r in 0..rows {
            let top = 40 + r as u32 * pitch;
            for y in top..top + ink {
                for x in 20..60 {
                    img.put_pixel(x, y, Luma([220]));
                }
            }
        }
        if border {
            for x in 0..400 {
                img.put_pixel(x, 0, Luma([128]));
                img.put_pixel(x, h - 1, Luma([128]));
            }
            for y in 0..h {
                img.put_pixel(0, y, Luma([128]));
                img.put_pixel(399, y, Luma([128]));
            }
        }
        if titlebar {
            for y in 4..30 {
                for x in 30..370 {
                    img.put_pixel(x, y, Luma([200]));
                }
            }
        }
        if scrollbar {
            for y in 35..155 {
                img.put_pixel(390, y, Luma([180]));
            }
        }
        img
    }

    #[test]
    fn chrome_is_suppressed_and_rows_survive() {
        let model = FgModel {
            bg: 10.0,
            delta: 60.0,
        };
        let img = draw_chrome_capture(24, 15, 13, true, true, true);
        let clean = suppress_structural_lines(&img, &model);
        let bands = find_text_bands(&clean, &model, 24);
        assert_eq!(bands.len(), 24, "chrome must not merge or add bands");
        // Pitch must be preserved for the grid arithmetic.
        let pitches: Vec<u32> = bands.windows(2).map(|p| p[1].top - p[0].top).collect();
        assert!(pitches.iter().all(|&p| p == 15), "{pitches:?}");
    }

    #[test]
    fn bridged_glyph_rows_found_via_bar_strip() {
        // Full-cell glyph ink that touches across the inter-row gap: whole-row
        // projection would fuse everything, the bar strip must not.
        let model = FgModel {
            bg: 10.0,
            delta: 60.0,
        };
        let mut img = draw_chrome_capture(6, 15, 13, false, false, false);
        for r in 0..6 {
            let top = 40 + r as u32 * 15;
            for y in top..top + 15 {
                for x in 70..90 {
                    img.put_pixel(x, y, Luma([220]));
                }
            }
        }
        let bands = find_text_bands(&img, &model, 6);
        assert_eq!(bands.len(), 6);
    }

    /// Display boxes span the full cell, strictly contain their assertion
    /// windows, and never overlap the neighbouring cells' display boxes —
    /// across integer and fractional pitches alike.
    #[test]
    fn full_cell_box_contains_assertion_box_without_neighbour_overlap() {
        let band = Band { top: 6, bottom: 19 };
        for &origin_x in &[8.0, 12.7] {
            for &pitch in &[9.33, 14.0, 22.5] {
                let cal = Calibration { origin_x, pitch };
                let prev = cal.cell_box_full(2, &band);
                let full = cal.cell_box_full(3, &band);
                let next = cal.cell_box_full(4, &band);
                let inner = cal.cell_box(3, &band);

                assert!(full.left <= inner.left && full.right >= inner.right);
                assert!(
                    full.left > prev.right,
                    "left neighbour overlap: {full:?} vs {prev:?}"
                );
                assert!(
                    next.left > full.right,
                    "right neighbour overlap: {next:?} vs {full:?}"
                );
            }
        }
    }

    #[test]
    fn selection_keeps_most_uniform_run() {
        // Chrome band above the grid, grid, speck below: the consecutive run
        // of 24 with uniform 15px pitch must win.
        let mut bands = vec![Band { top: 2, bottom: 20 }];
        for r in 0..24 {
            let top = 40 + r as u32 * 15;
            bands.push(Band {
                top,
                bottom: top + 12,
            });
        }
        bands.push(Band {
            top: 40 + 24 * 15,
            bottom: 40 + 24 * 15 + 2,
        });
        let sel = select_grid_bands(&bands, 24);
        assert_eq!(sel.len(), 24);
        assert_eq!(sel[0].top, 40);
        let pitches: Vec<u32> = sel.windows(2).map(|p| p[1].top - p[0].top).collect();
        assert!(pitches.iter().all(|&p| p == 15), "{pitches:?}");
        // Fewer bands than expected: returned unchanged, caller fails on count.
        assert_eq!(select_grid_bands(&bands[..23], 24).len(), 23);
    }

    /// Regression: real CI capture (linux/xterm, full desktop incl. window
    /// border) previously collapsed to 1 band.
    #[test]
    fn linux_ci_capture_yields_grid() {
        let bytes = include_bytes!("../tests/fixtures/linux_ci_page0.png");
        let img = image::load_from_memory(bytes).expect("decode").to_luma8();
        let model = FgModel {
            bg: estimate_background(&img),
            delta: 60.0,
        };
        let clean = suppress_structural_lines(&img, &model);
        let bands = find_text_bands(&clean, &model, 24);
        assert_eq!(bands.len(), 24);
        calibrate(&clean, &model, &bands[0]).expect("control row must calibrate");
    }

    /// Regression: real CI capture (macOS Terminal.app, title bar + scrollbar
    /// + alpha padding) previously collapsed to 1-3 bands.
    #[test]
    fn macos_ci_capture_yields_grid() {
        let bytes = include_bytes!("../tests/fixtures/macos_ci_page0.png");
        let img = image::load_from_memory(bytes).expect("decode").to_luma8();
        let model = FgModel {
            bg: estimate_background(&img),
            delta: 60.0,
        };
        let clean = suppress_structural_lines(&img, &model);
        let bands = select_grid_bands(&find_text_bands(&clean, &model, 24), 24);
        assert_eq!(bands.len(), 24);
    }

    /// Regression: partial last page (control + 3 candidates) on linux. The
    /// strip heuristic can latch onto braille dot columns here; production
    /// recovers through uniform-run selection, so mirror that path.
    #[test]
    fn linux_ci_partial_page_yields_rows() {
        let bytes = include_bytes!("../tests/fixtures/linux_ci_page22.png");
        let img = image::load_from_memory(bytes).expect("decode").to_luma8();
        let model = FgModel {
            bg: estimate_background(&img),
            delta: 60.0,
        };
        let clean = suppress_structural_lines(&img, &model);
        let mut bands = find_text_bands(&clean, &model, 4);
        if bands.len() > 4 {
            bands = select_grid_bands(&bands, 4);
        }
        assert_eq!(bands.len(), 4);
    }
}
