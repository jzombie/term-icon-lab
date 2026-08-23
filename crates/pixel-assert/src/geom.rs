//! Fixed-grid geometry slicing (plan §5).
//!
//! Cell boxes are derived **arithmetically** from the control-row anchors —
//! never from glyph-ink connectivity, which collapses when solid Block
//! Elements touch their sentinels. All spatial math is `f64` end-to-end; the
//! only integer conversion is the final `.round()` at the pixel-slice
//! boundary, so non-integer cell widths (e.g. 9.33 px) cannot accumulate
//! quantization drift across a line.

use image::GrayImage;

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

/// Estimate background luminance from the outermost 2-pixel frame of the
/// capture (the area around the rendered matrix is guaranteed bare SGR
/// background by the harness).
#[must_use]
pub fn estimate_background(gray: &GrayImage) -> f32 {
    let (w, h) = gray.dimensions();
    let mut sum = 0.0f64;
    let mut n = 0u64;
    for y in 0..h {
        for x in 0..w {
            if x < 2 || y < 2 || x >= w.saturating_sub(2) || y >= h.saturating_sub(2) {
                sum += f64::from(gray.get_pixel(x, y)[0]);
                n += 1;
            }
        }
    }
    if n == 0 { 0.0 } else { (sum / n as f64) as f32 }
}

/// A horizontal run of rows containing foreground ink (inclusive bounds).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Band {
    pub top: u32,
    pub bottom: u32,
}

/// Scan for text bands: consecutive rows whose ink-pixel count reaches
/// `min_ink`.
#[must_use]
pub fn find_bands(gray: &GrayImage, model: &FgModel, min_ink: usize) -> Vec<Band> {
    let (w, h) = gray.dimensions();
    let mut bands = Vec::new();
    let mut current: Option<u32> = None;
    for y in 0..h {
        let ink = (0..w).filter(|&x| model.is_fg(gray, x, y)).count();
        if ink >= min_ink {
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

    /// The open gutter between the icon cell's central crop and sentinel B's
    /// central crop — any ink here means the candidate intruded toward B.
    #[must_use]
    pub fn gutter_span(&self) -> (f64, f64) {
        (
            self.cell_center(3) + self.pitch * 0.35,
            self.cell_center(4) - self.pitch * 0.35,
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
        let bands = find_bands(&img, &model, 2);
        assert_eq!(bands.len(), 1);
        let cal = calibrate(&img, &model, &bands[0]).expect("calibration");
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
        let bands = find_bands(&img, &model, 2);
        let cal = calibrate(&img, &model, &bands[0]).unwrap();
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
}
