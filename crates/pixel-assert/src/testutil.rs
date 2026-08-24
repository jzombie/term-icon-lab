//! Synthetic capture fixtures shared by pipeline tests and export-grid tests.
//!
//! Compiled only under `cfg(test)`. Pages are drawn programmatically with
//! parameterizable rasterization (`Geometry`) so multi-platform scenarios
//! with distinct cell metrics can be synthesized headlessly.

use std::path::{Path, PathBuf};

use icon_catalog::{Block, candidates};
use image::{GrayImage, Luma};

use crate::Cli;

pub(crate) const BG: u8 = 10;
pub(crate) const FG: u8 = 220;

/// Rasterization parameters of a synthetic platform capture.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Geometry {
    pub(crate) origin_x: f64,
    pub(crate) pitch: f64,
    pub(crate) cell_h: u32,
}

impl Default for Geometry {
    fn default() -> Self {
        Self {
            origin_x: 10.0,
            pitch: 40.0,
            cell_h: 16,
        }
    }
}

impl Geometry {
    /// Row pitch: cell height plus a blank gap keeping bands separated.
    pub(crate) fn row_stride(&self) -> u32 {
        self.cell_h + 6
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum IconKind {
    Solid,
    Blank,
    Wide,
    BrailleDot,
}

pub(crate) struct PageSpec {
    pub(crate) candidates: Vec<(u32, Block, IconKind)>, // (codepoint, block, kind)
}

pub(crate) fn draw_page(spec: &PageSpec, geom: Geometry) -> GrayImage {
    let rows = spec.candidates.len() + 1; // + control
    let stride = geom.row_stride();
    // Generous top/bottom margins mirror a real terminal window: without
    // them, a short canvas lets boundary-bar ink runs cross the 30%
    // structural-line threshold and get suppressed as fake window chrome.
    let h = 64 + u32::try_from(rows).unwrap() * stride + 64;
    let w = 340;
    let mut img = GrayImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            img.put_pixel(x, y, Luma([BG]));
        }
    }

    let draw_row = |img: &mut GrayImage, row: u32, icon: Option<IconKind>| {
        let top = 6 + row * stride;
        let bottom = top + geom.cell_h - 1;
        // Boundary bars at col 0 and col 6 (thin).
        for col in [0usize, 6] {
            let cx = (geom.origin_x + col as f64 * geom.pitch).round() as i64;
            for dx in [-1i64, 0] {
                for y in top..=bottom {
                    img.put_pixel((cx + dx) as u32, y, Luma([FG]));
                }
            }
        }
        // Sentinel blobs at col 2 ('A') and col 4 ('B').
        for col in [2usize, 4] {
            let cx = (geom.origin_x + col as f64 * geom.pitch).round() as i64;
            let half = (geom.pitch * 0.22) as i64;
            for y in top..=bottom {
                for dx in -half..=half {
                    img.put_pixel((cx + dx) as u32, y, Luma([FG]));
                }
            }
        }
        // Icon slot at col 3.
        if let Some(kind) = icon {
            let cx = (geom.origin_x + 3.0 * geom.pitch).round() as i64;
            match kind {
                IconKind::Solid => {
                    let half = (geom.pitch * 0.3) as i64;
                    for y in top + 2..=bottom - 2 {
                        for dx in -half..=half {
                            img.put_pixel((cx + dx) as u32, y, Luma([FG]));
                        }
                    }
                }
                IconKind::Wide => {
                    // Models a 2-cell render: ink fills the entire next
                    // cell, saturating sentinel B's central crop.
                    let right = geom.pitch as i64;
                    for y in top + 2..=bottom - 2 {
                        for dx in -(geom.pitch * 0.3) as i64..=right {
                            img.put_pixel((cx + dx) as u32, y, Luma([FG]));
                        }
                    }
                }
                IconKind::BrailleDot => {
                    img.put_pixel(cx as u32, top + geom.cell_h / 2, Luma([FG]));
                }
                IconKind::Blank => {}
            }
        }
    };

    draw_row(&mut img, 0, None); // control row | A B |
    for (r, (_, _, kind)) in spec.candidates.iter().enumerate() {
        draw_row(&mut img, r as u32 + 1, Some(*kind));
    }
    img
}

/// Sidecar mirroring the harness schema. Ids are resolved through the real
/// catalog so downstream consumers (verdict correlation, export-grid) see
/// production-shaped identifiers.
pub(crate) fn sidecar_json(pages: &[PageSpec]) -> serde_json::Value {
    let mut rows = Vec::new();
    for (p, page) in pages.iter().enumerate() {
        rows.push(serde_json::json!({
            "kind": "control", "page": p, "page_row": 0,
        }));
        for (ri, (cp, block, _)) in page.candidates.iter().enumerate() {
            let id = catalog_id(*cp);
            rows.push(serde_json::json!({
                "kind": "candidate",
                "id": id,
                "codepoint": cp,
                "block": block,
                "page": p,
                "page_row": ri + 1,
                "expected_end_col": 8,
            }));
        }
    }
    serde_json::json!({
        "schema_version": 1,
        "platform": "test/platform",
        "host": "xterm",
        "pass1_support": true,
        "pass1_method": "batched-dsr",
        "page_rows": 24,
        "pages": pages.len(),
        "rows": rows,
    })
}

/// Catalog id for a fixture codepoint (fixture codepoints must be catalog
/// members).
fn catalog_id(codepoint: u32) -> String {
    candidates()
        .iter()
        .find(|c| c.codepoint == codepoint)
        .unwrap_or_else(|| panic!("fixture codepoint U+{codepoint:04X} must be in the catalog"))
        .id
        .clone()
}

/// `statuses`: id → pass1 status; missing ids become `inconclusive`.
pub(crate) fn pass1_json(statuses: &[(&str, &str)]) -> serde_json::Value {
    let results: Vec<serde_json::Value> = statuses
        .iter()
        .map(|(id, status)| serde_json::json!({ "id": id, "codepoint": 0, "status": status }))
        .collect();
    serde_json::json!({ "pass1_support": true, "results": results })
}

/// A `Cli` with every field neutralized; tests override what they exercise.
pub(crate) fn base_cli() -> Cli {
    Cli {
        pngs: Vec::new(),
        sidecar: None,
        pass1: None,
        out: None,
        // Production capture-gate floor: synthetic pages carry real ink
        // entropy well above it; flat failure frames measure exactly 0.
        min_entropy: 0.02,
        validate_only: None,
        export_grid: None,
        grid_inputs: Vec::new(),
        grid_manifest: None,
        grid_ucd: PathBuf::new(),
    }
}

pub(crate) struct Fixture {
    pub(crate) _dir: tempfile_guard::DirGuard,
    pub(crate) cli: Cli,
}

pub(crate) mod tempfile_guard {
    use std::fs;
    use std::ops::{Deref, DerefMut};
    use std::path::{Path, PathBuf};

    pub(crate) struct DirGuard(PathBuf);
    impl DirGuard {
        pub(crate) fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("ti_px_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            DirGuard(dir)
        }
        pub(crate) fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    impl Deref for DirGuard {
        type Target = PathBuf;
        fn deref(&self) -> &PathBuf {
            &self.0
        }
    }
    impl DerefMut for DirGuard {
        fn deref_mut(&mut self) -> &mut PathBuf {
            &mut self.0
        }
    }
}

pub(crate) fn build_fixture(
    name: &str,
    pages: &[PageSpec],
    png_count: usize,
    pass1: serde_json::Value,
) -> Fixture {
    let dir = tempfile_guard::DirGuard::new(name);
    let mut pngs = Vec::new();
    for (p, page) in pages.iter().enumerate() {
        let img = draw_page(page, Geometry::default());
        let path = dir.path().join(format!("shot_page_{p}.png"));
        image::save_buffer(
            &path,
            img.as_raw(),
            img.width(),
            img.height(),
            image::ColorType::L8,
        )
        .unwrap();
        if p < png_count {
            pngs.push(path);
        }
    }
    let sidecar_path = dir.path().join("sidecar.json");
    std::fs::write(&sidecar_path, sidecar_json(pages).to_string()).unwrap();
    let pass1_path = dir.path().join("pass1.json");
    std::fs::write(&pass1_path, pass1.to_string()).unwrap();

    let cli = Cli {
        pngs,
        sidecar: Some(sidecar_path),
        pass1: Some(pass1_path),
        out: Some(dir.path().join("verdicts.json")),
        ..base_cli()
    };
    Fixture { _dir: dir, cli }
}

/// A CI-shaped artifact tree consumed by `--export-grid`:
/// `pages/shot_page_N.png` plus `artifacts/sidecar.json`.
pub(crate) struct CaptureRoot {
    pub(crate) _dir: tempfile_guard::DirGuard,
}

impl CaptureRoot {
    pub(crate) fn path(&self) -> &Path {
        self._dir.path()
    }
}

pub(crate) fn capture_root(name: &str, pages: &[PageSpec], geom: Geometry) -> CaptureRoot {
    let dir = tempfile_guard::DirGuard::new(name);
    std::fs::create_dir_all(dir.path().join("artifacts")).unwrap();
    std::fs::create_dir_all(dir.path().join("pages")).unwrap();
    for (p, page) in pages.iter().enumerate() {
        let img = draw_page(page, geom);
        let path = dir.path().join("pages").join(format!("shot_page_{p}.png"));
        image::save_buffer(
            &path,
            img.as_raw(),
            img.width(),
            img.height(),
            image::ColorType::L8,
        )
        .unwrap();
    }
    let sidecar_path = dir.path().join("artifacts").join("sidecar.json");
    std::fs::write(&sidecar_path, sidecar_json(pages).to_string()).unwrap();
    CaptureRoot { _dir: dir }
}

/// Write a generated-manifest-shaped file listing `ids` (order is
/// deliberately irrelevant to consumers).
pub(crate) fn write_manifest_file(path: &Path, ids: &[&str]) {
    let mut src = String::from("// @generated by manifest-gen\n");
    for id in ids {
        src.push_str(&format!(
            "pub const PLACEHOLDER: crate::IconEntry = crate::IconEntry {{ id: \"{id}\" }};\n"
        ));
    }
    std::fs::write(path, src).unwrap();
}

/// Write a minimal UCD extract covering `entries`.
pub(crate) fn write_ucd_file(path: &Path, entries: &[(u32, &str)]) {
    let text: String = entries
        .iter()
        .map(|(cp, name)| format!("{cp:04X};{name};Sm;0;ON;;;;;N;;;;;\n"))
        .collect();
    std::fs::write(path, text).unwrap();
}
