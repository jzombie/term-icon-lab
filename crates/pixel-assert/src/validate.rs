//! Capture validity gate: reject blank / degenerate screenshots *before* any
//! assertion runs, so a broken capture path fails loudly instead of producing
//! thousands of misleading per-glyph verdicts.

use image::DynamicImage;

/// Measured statistics of a captured frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaptureStats {
    /// Shannon entropy of the 256-bin grayscale histogram, in bits.
    pub entropy_bits: f64,
    /// Standard deviation of grayscale luminance.
    pub luminance_std_dev: f64,
    /// Number of distinct RGB colors observed.
    pub unique_colors: usize,
}

/// Minimum plausible-capture values (defaults; overridable via CLI).
#[derive(Clone, Copy, Debug)]
pub struct CaptureGate {
    pub min_entropy_bits: f64,
    pub min_luminance_std_dev: f64,
    pub min_unique_colors: usize,
}

impl Default for CaptureGate {
    fn default() -> Self {
        // Calibrated against real failure modes: flat swapchain surfaces /
        // blank desktops have zero entropy spread and zero luminance
        // deviation. Crisp monochrome text renders legitimately have *low*
        // entropy (~0.5 bits), so the floors stay conservative — deeper
        // implausibility is caught by band matching downstream.
        Self {
            min_entropy_bits: 0.3,
            min_luminance_std_dev: 8.0,
            min_unique_colors: 2,
        }
    }
}

impl CaptureStats {
    /// A capture is plausible when it shows real rendered content rather than
    /// an empty desktop, a black swapchain surface, or a flat failure frame.
    #[must_use]
    pub fn passes(&self, gate: &CaptureGate) -> bool {
        self.entropy_bits >= gate.min_entropy_bits
            && self.luminance_std_dev >= gate.min_luminance_std_dev
            && self.unique_colors >= gate.min_unique_colors
    }
}

/// Measure a captured RGB(A) frame.
#[must_use]
pub fn measure(img: &DynamicImage) -> CaptureStats {
    let rgb = img.to_rgb8();
    let mut hist = [0u64; 256];
    let mut colors = std::collections::HashSet::new();
    let mut sum = 0.0f64;
    let mut n = 0.0f64;

    for px in rgb.pixels() {
        // Rec.601 luma, matching the grayscale conversion used by analysis.
        let lum = (0.299 * f64::from(px[0]) + 0.587 * f64::from(px[1]) + 0.114 * f64::from(px[2]))
            .round() as u8;
        hist[usize::from(lum)] += 1;
        colors.insert([px[0], px[1], px[2]]);
        sum += f64::from(lum);
        n += 1.0;
    }

    let mean = if n > 0.0 { sum / n } else { 0.0 };
    let var = if n > 0.0 {
        rgb.pixels()
            .map(|px| {
                let l =
                    0.299 * f64::from(px[0]) + 0.587 * f64::from(px[1]) + 0.114 * f64::from(px[2]);
                (l - mean) * (l - mean)
            })
            .sum::<f64>()
            / n
    } else {
        0.0
    };

    let entropy = if n > 0.0 {
        hist.iter()
            .filter(|c| **c > 0)
            .map(|c| {
                let p = f64::from(*c as u32) / n;
                -p * p.log2()
            })
            .sum::<f64>()
    } else {
        0.0
    };

    CaptureStats {
        entropy_bits: entropy,
        luminance_std_dev: var.sqrt(),
        unique_colors: colors.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    fn solid(color: [u8; 3], w: u32, h: u32) -> DynamicImage {
        let mut img = RgbImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                img.put_pixel(x, y, Rgb(color));
            }
        }
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn blank_black_capture_is_rejected() {
        let stats = measure(&solid([0, 0, 0], 320, 200));
        assert!(!stats.passes(&CaptureGate::default()));
    }

    #[test]
    fn blank_white_desktop_capture_is_rejected() {
        let stats = measure(&solid([255, 255, 255], 320, 200));
        assert!(!stats.passes(&CaptureGate::default()));
    }

    #[test]
    fn noise_capture_passes() {
        let mut img = RgbImage::new(320, 200);
        let mut seed = 0x1234_5678_u32;
        for y in 0..200 {
            for x in 0..320 {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let v = (seed >> 24) as u8;
                img.put_pixel(x, y, Rgb([v, v, v]));
            }
        }
        let stats = measure(&DynamicImage::ImageRgb8(img));
        assert!(stats.passes(&CaptureGate::default()), "{stats:?}");
    }

    #[test]
    fn text_like_render_passes() {
        // Dark bg with bright glyph blobs of varying intensity — the
        // realistic antialiased case.
        let mut img = RgbImage::new(320, 200);
        for y in 0..200 {
            for x in 0..320 {
                img.put_pixel(x, y, Rgb([10, 10, 10]));
            }
        }
        for i in 0..40 {
            let v = (120 + (i % 30) * 4) as u8;
            for dy in 0..12 {
                for dx in 0..5 {
                    img.put_pixel(10 + i * 7 + dx, 40 + dy, Rgb([v, v, v]));
                }
            }
        }
        let stats = measure(&DynamicImage::ImageRgb8(img));
        assert!(stats.passes(&CaptureGate::default()), "{stats:?}");
    }
}
