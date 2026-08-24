//! `--export-grid`: compose verified-grid preview PNGs from captured pages.
//!
//! A stateless visualization engine. It consumes an ID set scraped from a
//! generated manifest — the single source of truth owned by `manifest-gen` —
//! locates each icon's cell through the platform sidecars, slices
//! native-resolution crops (never resampled), and emits byte-deterministic
//! outputs:
//!
//! * `universal-grid.png` — canonical-platform (`linux`) tiles, 16 columns,
//!   ordered by `(Block::ALL order, codepoint)`, one blank slot-row between
//!   Unicode blocks.
//! * `universal-matrix.png` — one row per icon, columns fixed to
//!   `[macos | windows | linux]`.
//! * `grid-index.json` — position → icon metadata (incl. `unicode_name`
//!   resolved from the vendored UCD extract).
//!
//! No verdict data is read here: set membership is decided upstream.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use icon_catalog::{Candidate, candidates};
use image::GenericImageView;
use image::imageops::FilterType;
use image::{Rgb, RgbImage};
use manifest_gen::{load_ucd_names, previous_ids};
use serde::Serialize;

use crate::font5x7;
use crate::geom::{CellBox, FgModel, estimate_background, suppress_structural_lines};
use crate::schema::{RowEntry, Sidecar};
use crate::validate::CaptureGate;
use crate::{Cli, grayscale};

/// Fixed column count of `universal-grid.png`.
pub(crate) const GRID_COLUMNS: usize = 16;
/// Matrix column order — pinned for cross-run comparability.
pub(crate) const PLATFORM_ORDER: [&str; 3] = ["macos", "windows", "linux"];
/// Platform whose rasterization fills display charts (`universal-grid.png`,
/// `universal-catalog.png`).
const CANONICAL_TILE_PLATFORM: &str = "linux";
const INDEX_FILE: &str = "grid-index.json";
const GRID_PNG: &str = "universal-grid.png";
const MATRIX_PNG: &str = "universal-matrix.png";
const SPECIMEN_PNG: &str = "universal-catalog.png";

/// Specimen-chart look constants: nearest-neighbour zoom factor for crops,
/// target overall width, and cell padding/gap metrics.
const LABEL_SCALE: u32 = 2;
const SPECIMEN_TARGET_WIDTH: f64 = 480.0;
const SPECIMEN_PAD_X: u32 = 6;
const SPECIMEN_PAD_Y: u32 = 5;
const SPECIMEN_LABEL_GAP: u32 = 6;
const LABEL_COLOR: Rgb<u8> = Rgb([200, 200, 200]);

#[derive(Debug)]
pub(crate) struct GridInput {
    pub(crate) label: String,
    pub(crate) root: PathBuf,
}

#[derive(Debug)]
pub(crate) struct GridOptions {
    /// Generated manifest defining the exact rendering set.
    pub(crate) manifest: PathBuf,
    /// Vendored UnicodeData extract supplying `unicode_name`.
    pub(crate) ucd: PathBuf,
}

/// CLI-facing wrapper: translate `--grid-input label=dir` repeats and dispatch.
pub(crate) fn run_export_grid(cli: &Cli) -> anyhow::Result<()> {
    let Some(out_dir) = cli.export_grid.as_deref() else {
        bail!("--export-grid is required");
    };
    let mut inputs = Vec::with_capacity(cli.grid_inputs.len());
    for raw in &cli.grid_inputs {
        let Some((label, dir)) = raw.split_once('=') else {
            bail!("bad --grid-input '{raw}': expected label=dir");
        };
        inputs.push(GridInput {
            label: label.trim().to_ascii_lowercase(),
            root: PathBuf::from(dir.trim()),
        });
    }
    let manifest = cli
        .grid_manifest
        .clone()
        .context("--grid-manifest is required with --export-grid")?;
    let opts = GridOptions {
        manifest,
        ucd: cli.grid_ucd.clone(),
    };
    export_grid(&inputs, &opts, out_dir)
}

pub(crate) fn export_grid(
    inputs: &[GridInput],
    opts: &GridOptions,
    out_dir: &Path,
) -> anyhow::Result<()> {
    // -- Platform roots: exactly one per sanctioned label, distinct.
    if inputs.len() != PLATFORM_ORDER.len() {
        bail!(
            "expected exactly {} --grid-input roots, got {}",
            PLATFORM_ORDER.len(),
            inputs.len()
        );
    }
    let mut by_label: HashMap<&str, &GridInput> = HashMap::new();
    for input in inputs {
        if !PLATFORM_ORDER.contains(&input.label.as_str()) {
            bail!(
                "unknown platform label '{}' (expected one of {PLATFORM_ORDER:?})",
                input.label
            );
        }
        if by_label.insert(input.label.as_str(), input).is_some() {
            bail!("duplicate platform label '{}'", input.label);
        }
    }

    // -- Target set: scrape ids straight out of the generated manifest.
    let ids = previous_ids(&opts.manifest);
    if ids.is_empty() {
        bail!(
            "manifest {} lists no icon ids — refusing to render an empty set",
            opts.manifest.display()
        );
    }
    let catalog = candidates();
    let mut targets: Vec<&Candidate> = ids
        .iter()
        .map(|id| {
            catalog.iter().find(|c| &c.id == id).ok_or_else(|| {
                anyhow::anyhow!("manifest id '{id}' is not present in the icon catalog")
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    targets.sort_by_key(|c| (block_rank(c.block), c.codepoint));

    // -- Sidecars: every platform must describe the identical render.
    let mut sidecars: HashMap<&str, Sidecar> = HashMap::new();
    for input in inputs {
        let path = input.root.join("artifacts").join("sidecar.json");
        let sc: Sidecar = serde_json::from_slice(
            &std::fs::read(&path).with_context(|| format!("read {}", path.display()))?,
        )
        .with_context(|| format!("parse {}", path.display()))?;
        sidecars.insert(input.label.as_str(), sc);
    }
    let canonical = sidecars
        .get(CANONICAL_TILE_PLATFORM)
        .context("canonical platform root missing")?;
    let canonical_seq = candidate_sequence(canonical);
    for label in PLATFORM_ORDER {
        let sc = &sidecars[label];
        if candidate_sequence(sc) != canonical_seq {
            bail!(
                "sidecar candidate sequence for '{label}' differs from '{CANONICAL_TILE_PLATFORM}'"
            );
        }
    }
    let canonical_ids: std::collections::HashSet<&str> =
        canonical_seq.iter().map(|(id, ..)| *id).collect();
    for t in &targets {
        if !canonical_ids.contains(t.id.as_str()) {
            bail!(
                "manifest id '{}' is absent from the captured sidecars",
                t.id
            );
        }
    }

    // -- Sequential page extraction: one page buffer in memory at a time.
    let gate = CaptureGate::default();
    // Keyed by (platform label, catalog id). Ids are `&'static` (catalog
    // lives in a `OnceLock`), labels borrow `inputs`.
    let mut crops: HashMap<(&str, &str), RgbImage> = HashMap::new();

    for input in inputs {
        let sidecar = &sidecars[input.label.as_str()];
        let pages = collect_pages(&input.root.join("pages"), sidecar.pages)?;
        for (idx, path) in pages.iter().enumerate() {
            let page = idx as u32;
            let img = image::open(path).with_context(|| format!("open {}", path.display()))?;
            let stats = crate::validate::measure(&img);
            if !stats.passes(&gate) {
                bail!("{}: implausible capture ({stats:?})", path.display());
            }
            let gray = grayscale(&img);
            drop(img);

            let bg = estimate_background(&gray);
            let model = FgModel { bg, delta: 60.0 };
            let clean = suppress_structural_lines(&gray, &model);
            let expected_rows = sidecar.rows_for_page(page);
            let (cal, bands) = crate::page_geometry(&clean, &model, &expected_rows)
                .map_err(|e| anyhow::anyhow!("{}: {e:?}", path.display()))?;
            drop(clean);

            for row in &expected_rows {
                let RowEntry::Candidate { id, .. } = row else {
                    continue;
                };
                if !ids.contains(id) {
                    continue;
                }
                let band = &bands[usize::from(row.page_row())];
                let crop = clamp_crop(&gray, cal.cell_box_full(3, band));
                crops.insert((input.label.as_str(), id.as_str()), crop);
            }
        }
    }
    if crops.is_empty() {
        bail!("no crops extracted — manifest ids and captures disagree");
    }

    // -- Uniform slot geometry (largest crop + 1 px padding per side).
    let slot_w = crops.values().map(RgbImage::width).max().unwrap_or(1) + 2;
    let slot_h = crops.values().map(RgbImage::height).max().unwrap_or(1) + 2;

    let tile_for = |label: &str, id: &str| -> anyhow::Result<RgbImage> {
        let crop = crops
            .get(&(label, id))
            .with_context(|| format!("missing {label} crop for '{id}'"))?;
        Ok(tile(crop, slot_w, slot_h))
    };

    // -- universal-grid.png: canonical-platform tiles with block separators.
    let (placements, grid_rows) = block_separated_placements(&targets, GRID_COLUMNS);
    let mut grid_canvas = RgbImage::from_pixel(
        GRID_COLUMNS as u32 * slot_w,
        grid_rows as u32 * slot_h,
        Rgb([0, 0, 0]),
    );
    for (ti, r, c) in &placements {
        let tile = tile_for(CANONICAL_TILE_PLATFORM, targets[*ti].id.as_str())?;
        image::imageops::overlay(
            &mut grid_canvas,
            &tile,
            i64::from(*c as u32 * slot_w),
            i64::from(*r as u32 * slot_h),
        );
    }

    // -- universal-matrix.png: [macos | windows | linux] per survivor row.
    let mut matrix_canvas = RgbImage::from_pixel(
        PLATFORM_ORDER.len() as u32 * slot_w,
        targets.len() as u32 * slot_h,
        Rgb([0, 0, 0]),
    );
    for (ti, t) in targets.iter().enumerate() {
        for (p, label) in PLATFORM_ORDER.iter().enumerate() {
            let tile = tile_for(label, t.id.as_str())?;
            image::imageops::overlay(
                &mut matrix_canvas,
                &tile,
                i64::from(p as u32 * slot_w),
                i64::from(ti as u32 * slot_h),
            );
        }
    }

    // -- universal-catalog.png: font-repo specimen sheet (~480 px wide).
    // Each cell = full-cell glyph render (2× nearest-neighbour) above its
    // `U+XXXX` activation label.
    let zoomed: Vec<RgbImage> = targets
        .iter()
        .map(|t| {
            let crop = crops
                .get(&(CANONICAL_TILE_PLATFORM, t.id.as_str()))
                .with_context(|| format!("missing linux crop for '{}'", t.id))?;
            Ok(image::imageops::resize(
                crop,
                crop.width() * LABEL_SCALE,
                crop.height() * LABEL_SCALE,
                FilterType::Nearest,
            ))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let glyph_w = zoomed.iter().map(RgbImage::width).max().unwrap_or(1);
    let glyph_h = zoomed.iter().map(RgbImage::height).max().unwrap_or(1);
    let label_w = font5x7::text_width("U+0000".len(), LABEL_SCALE);
    let cell_w = glyph_w.max(label_w) + SPECIMEN_PAD_X * 2;
    let cell_h = glyph_h + SPECIMEN_LABEL_GAP + font5x7::FONT_H * LABEL_SCALE + SPECIMEN_PAD_Y * 2;
    let specimen_cols = ((SPECIMEN_TARGET_WIDTH / cell_w as f64).floor() as usize).clamp(3, 8);
    let (spec_place, spec_rows) = block_separated_placements(&targets, specimen_cols);
    let mut catalog_canvas = RgbImage::from_pixel(
        specimen_cols as u32 * cell_w,
        spec_rows as u32 * cell_h,
        Rgb([0, 0, 0]),
    );
    for (ti, r, c) in &spec_place {
        let t = targets[*ti];
        let x0 = *c as u32 * cell_w;
        let y0 = *r as u32 * cell_h;
        let glyph = &zoomed[*ti];
        image::imageops::overlay(
            &mut catalog_canvas,
            glyph,
            i64::from(x0 + (cell_w - glyph.width()) / 2),
            i64::from(y0 + SPECIMEN_PAD_Y),
        );
        let text = format!("U+{:04X}", t.codepoint);
        font5x7::draw_text(
            &mut catalog_canvas,
            i64::from(x0 + (cell_w - label_w) / 2),
            i64::from(y0 + SPECIMEN_PAD_Y + glyph_h + SPECIMEN_LABEL_GAP),
            &text,
            LABEL_SCALE,
            LABEL_COLOR,
        );
    }

    // -- grid-index.json with the *computed* slot geometry.
    let names = load_ucd_names(&opts.ucd)?;
    let tiles = targets
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let (_, grid_row, grid_col) = placements[i];
            let (_, specimen_row, specimen_col) = spec_place[i];
            TileMeta {
                index: i,
                grid_row,
                grid_col,
                specimen_row,
                specimen_col,
                id: t.id.as_str(),
                codepoint: t.codepoint,
                block: t.block,
                unicode_name: names.get(&t.codepoint).map(String::as_str).unwrap_or(""),
                matrix_col: PLATFORM_ORDER
                    .iter()
                    .enumerate()
                    .map(|(col, label)| (*label, col))
                    .collect(),
            }
        })
        .collect();
    let index = GridIndexFile {
        schema_version: 3,
        columns: GRID_COLUMNS,
        specimen_columns: specimen_cols,
        cell_width_px: slot_w,
        cell_height_px: slot_h,
        platform_order: PLATFORM_ORDER,
        tiles,
    };

    // -- All-or-nothing output: touch disk only after everything composed.
    std::fs::create_dir_all(out_dir).with_context(|| format!("create {}", out_dir.display()))?;
    for (name, canvas) in [
        (GRID_PNG, &grid_canvas),
        (MATRIX_PNG, &matrix_canvas),
        (SPECIMEN_PNG, &catalog_canvas),
    ] {
        image::save_buffer(
            out_dir.join(name),
            canvas.as_raw(),
            canvas.width(),
            canvas.height(),
            image::ColorType::Rgb8,
        )
        .with_context(|| format!("write {}", out_dir.join(name).display()))?;
    }
    std::fs::write(out_dir.join(INDEX_FILE), serde_json::to_vec_pretty(&index)?)
        .with_context(|| format!("write {}", out_dir.join(INDEX_FILE).display()))?;

    println!(
        "export-grid: {} icons → {}, {}, {}, {}",
        targets.len(),
        GRID_PNG,
        MATRIX_PNG,
        SPECIMEN_PNG,
        INDEX_FILE
    );
    Ok(())
}

/// Sequential block-separated placement of `targets` into a `cols`-wide
/// canvas: `(tile_index, row, col)` per target plus total row count. A fully
/// blank separator slot-row is inserted whenever the Unicode block changes
/// between consecutive tiles.
fn block_separated_placements(
    targets: &[&Candidate],
    cols: usize,
) -> (Vec<(usize, usize, usize)>, usize) {
    let mut placements = Vec::with_capacity(targets.len());
    let mut prev_block = None;
    let (mut row, mut col) = (0usize, 0usize);
    for (ti, t) in targets.iter().enumerate() {
        if prev_block.is_some_and(|pb: icon_catalog::Block| pb != t.block) {
            row += 2; // skip onto the blank separator slot-row
            col = 0;
        }
        placements.push((ti, row, col));
        prev_block = Some(t.block);
        col += 1;
        if col == cols {
            col = 0;
            row += 1;
        }
    }
    let rows = placements.last().map_or(1, |(_, r, _)| r + 1);
    (placements, rows)
}

/// Catalog-block ordering rank (`Block::ALL` manifest order).
fn block_rank(block: icon_catalog::Block) -> usize {
    icon_catalog::Block::ALL
        .iter()
        .position(|b| *b == block)
        .unwrap_or(usize::MAX)
}

/// Ordered `(id, codepoint, page, page_row)` tuples of a sidecar's candidate
/// rows — the cross-platform identity of a capture.
fn candidate_sequence(sc: &Sidecar) -> Vec<(&str, u32, u32, u16)> {
    sc.rows
        .iter()
        .filter_map(|r| match r {
            RowEntry::Candidate {
                id,
                codepoint,
                page,
                page_row,
                ..
            } => Some((id.as_str(), *codepoint, *page, *page_row)),
            RowEntry::Control { .. } => None,
        })
        .collect()
}

/// Collect `pages/shot_page_N.png` paths sorted by parsed numeric suffix,
/// requiring exactly `expected` contiguous pages.
fn collect_pages(dir: &Path, expected: u32) -> anyhow::Result<Vec<PathBuf>> {
    let entries = std::fs::read_dir(dir).with_context(|| format!("read {}", dir.display()))?;
    let mut numbered: Vec<(u32, PathBuf)> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let Some(num) = name
            .strip_prefix("shot_page_")
            .and_then(|r| r.strip_suffix(".png"))
        else {
            continue;
        };
        let n: u32 = num
            .parse()
            .with_context(|| format!("unparseable page image name '{name}'"))?;
        numbered.push((n, entry.path()));
    }
    numbered.sort_by_key(|(n, _)| *n);
    if numbered.len() != expected as usize {
        bail!(
            "{}: expected {} page images, found {}",
            dir.display(),
            expected,
            numbered.len()
        );
    }
    for (i, (n, _)) in numbered.iter().enumerate() {
        if *n != i as u32 {
            bail!(
                "{}: non-contiguous page numbering at shot_page_{n}",
                dir.display()
            );
        }
    }
    Ok(numbered.into_iter().map(|(_, p)| p).collect())
}

/// Bounds-clamped crop: rounding at canvas edges can push `right/bottom` one
/// pixel past the buffer, which would panic inside `view(...)`.
fn clamp_crop(img: &image::GrayImage, b: CellBox) -> RgbImage {
    let x = b.left.min(img.width().saturating_sub(1));
    let y = b.top.min(img.height().saturating_sub(1));
    let w = (b.right - x + 1).min(img.width() - x);
    let h = (b.bottom - y + 1).min(img.height() - y);
    let luma = img.view(x, y, w.max(1), h.max(1)).to_image();
    image::DynamicImage::ImageLuma8(luma).into_rgb8()
}

/// Center a crop on a black-padded uniform slot.
fn tile(crop: &RgbImage, slot_w: u32, slot_h: u32) -> RgbImage {
    let mut slot = RgbImage::from_pixel(slot_w, slot_h, Rgb([0, 0, 0]));
    let dx = slot_w.saturating_sub(crop.width()) / 2;
    let dy = slot_h.saturating_sub(crop.height()) / 2;
    image::imageops::overlay(&mut slot, crop, i64::from(dx), i64::from(dy));
    slot
}

#[derive(Serialize)]
struct GridIndexFile<'a> {
    schema_version: u32,
    columns: usize,
    specimen_columns: usize,
    cell_width_px: u32,
    cell_height_px: u32,
    platform_order: [&'a str; 3],
    tiles: Vec<TileMeta<'a>>,
}

#[derive(Serialize)]
struct TileMeta<'a> {
    index: usize,
    grid_row: usize,
    grid_col: usize,
    specimen_row: usize,
    specimen_col: usize,
    id: &'a str,
    codepoint: u32,
    block: icon_catalog::Block,
    unicode_name: &'a str,
    matrix_col: BTreeMap<&'static str, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{
        CaptureRoot, Geometry, IconKind, PageSpec, capture_root, tempfile_guard,
        write_manifest_file, write_ucd_file,
    };
    use image::{GrayImage, Luma};

    const MACOS_GEOM: Geometry = Geometry {
        origin_x: 14.0,
        pitch: 14.0,
        cell_h: 16,
    };
    const WINDOWS_GEOM: Geometry = Geometry {
        origin_x: 8.0,
        pitch: 22.0,
        cell_h: 24,
    };
    const LINUX_GEOM: Geometry = Geometry {
        origin_x: 10.0,
        pitch: 18.0,
        cell_h: 20,
    };

    fn solid(cp: u32, block: icon_catalog::Block) -> (u32, icon_catalog::Block, IconKind) {
        (cp, block, IconKind::Solid)
    }

    /// Two pages spanning three blocks: box_drawing, geometric_shapes,
    /// misc_symbols — exercising ordering and block-separator layout.
    fn fixture_pages() -> Vec<PageSpec> {
        vec![
            PageSpec {
                candidates: vec![solid(0x2502, icon_catalog::Block::BoxDrawing)],
            },
            PageSpec {
                candidates: vec![
                    solid(0x2500, icon_catalog::Block::BoxDrawing),
                    solid(0x2605, icon_catalog::Block::MiscSymbols),
                    solid(0x25B2, icon_catalog::Block::GeometricShapes),
                ],
            },
        ]
    }

    struct Roots {
        _guards: Vec<CaptureRoot>,
    }

    fn three_roots(tag: &str, pages: &[PageSpec]) -> Roots {
        Roots {
            _guards: vec![
                capture_root(&format!("{tag}_macos"), pages, MACOS_GEOM),
                capture_root(&format!("{tag}_windows"), pages, WINDOWS_GEOM),
                capture_root(&format!("{tag}_linux"), pages, LINUX_GEOM),
            ],
        }
    }

    fn inputs_from(roots: &Roots) -> Vec<GridInput> {
        vec![
            GridInput {
                label: "macos".into(),
                root: roots._guards[0].path().to_path_buf(),
            },
            GridInput {
                label: "windows".into(),
                root: roots._guards[1].path().to_path_buf(),
            },
            GridInput {
                label: "linux".into(),
                root: roots._guards[2].path().to_path_buf(),
            },
        ]
    }

    fn opts_in(dir: &tempfile_guard::DirGuard, ids: &[&str]) -> GridOptions {
        write_manifest_file(&dir.path().join("manifest.rs"), ids);
        write_ucd_file(
            &dir.path().join("ucd.txt"),
            &[
                (0x2500, "BOX DRAWINGS LIGHT HORIZONTAL"),
                (0x2502, "BOX DRAWINGS LIGHT VERTICAL"),
                (0x25B2, "BLACK UP-POINTING TRIANGLE"),
                (0x2605, "BLACK STAR"),
            ],
        );
        GridOptions {
            manifest: dir.path().join("manifest.rs"),
            ucd: dir.path().join("ucd.txt"),
        }
    }

    fn read_index(out: &Path) -> serde_json::Value {
        serde_json::from_slice(&std::fs::read(out.join(INDEX_FILE)).unwrap()).unwrap()
    }

    #[test]
    fn happy_path_dims_order_names_and_matrix_columns() {
        let pages = fixture_pages();
        let roots = three_roots("happy", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("happy_out");
        let opts = opts_in(
            &dir,
            &["box_2502", "box_2500", "misc_2605", "geometric_25B2"],
        );
        let out = dir.path().join("assets");

        export_grid(&inputs, &opts, &out).expect("export succeeds");

        let index = read_index(&out);
        assert_eq!(index["schema_version"], 3);
        assert_eq!(index["columns"], 16);
        assert_eq!(
            index["platform_order"],
            serde_json::json!(["macos", "windows", "linux"])
        );
        let tiles = index["tiles"].as_array().unwrap();
        assert_eq!(tiles.len(), 4);
        // Sorted by (block order, codepoint): box_drawing < geometric < misc.
        let ids: Vec<&str> = tiles.iter().map(|t| t["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["box_2500", "box_2502", "geometric_25B2", "misc_2605"]);
        assert_eq!(tiles[0]["unicode_name"], "BOX DRAWINGS LIGHT HORIZONTAL");
        assert_eq!(tiles[3]["unicode_name"], "BLACK STAR");
        assert_eq!(
            tiles[0]["matrix_col"],
            serde_json::json!({"linux": 2, "macos": 0, "windows": 1})
        );

        // Serialized slot geometry must agree with the actual canvases.
        let cw = index["cell_width_px"].as_u64().unwrap() as u32;
        let ch = index["cell_height_px"].as_u64().unwrap() as u32;
        assert!(cw > 2 && ch > 2, "{cw}x{ch}");
        let matrix = image::open(out.join(MATRIX_PNG)).unwrap();
        assert_eq!((matrix.width(), matrix.height()), (3 * cw, 4 * ch));
        let grid = image::open(out.join(GRID_PNG)).unwrap();
        assert_eq!(grid.width(), (GRID_COLUMNS as u32) * cw);
        // Blocks change twice ⇒ two blank separator rows: 1 + 2 + 2 rows.
        assert_eq!(grid.height(), 5 * ch);
        // Placement bookkeeping matches the canvases.
        assert_eq!(tiles[0]["grid_row"], 0);
        assert_eq!(tiles[1]["grid_col"], 1);
        assert_eq!(tiles[2]["grid_row"], 2);
        assert_eq!(tiles[3]["grid_row"], 4);

        // Specimen chart: ~480 px wide, same block-separated layout, and
        // every tile carries its specimen coordinates.
        assert!(out.join(SPECIMEN_PNG).is_file(), "catalog chart written");
        let spec_cols = index["specimen_columns"].as_u64().unwrap() as u32;
        assert!((3..=8).contains(&spec_cols));
        let catalog = image::open(out.join(SPECIMEN_PNG)).unwrap();
        assert!(
            (400..=520).contains(&catalog.width()),
            "specimen width {} outside font-repo range",
            catalog.width()
        );
        assert_eq!(tiles[0]["specimen_row"], 0);
        assert_eq!(tiles[0]["specimen_col"], 0);
        assert_eq!(
            tiles[2]["specimen_row"], 2,
            "block separator before geometric"
        );
        assert_eq!(tiles[3]["specimen_row"], 4, "block separator before misc");
    }

    /// `universal-grid.png` must be composed from the canonical (linux)
    /// crops: its tiles are byte-identical to the matrix's linux column.
    #[test]
    fn grid_tiles_come_from_canonical_linux_platform() {
        let pages = fixture_pages();
        let roots = three_roots("pinning", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("pinning_out");
        let opts = opts_in(&dir, &["box_2502"]);
        let out = dir.path().join("assets");

        export_grid(&inputs, &opts, &out).unwrap();

        let index = read_index(&out);
        let cw = index["cell_width_px"].as_u64().unwrap() as u32;
        let ch = index["cell_height_px"].as_u64().unwrap() as u32;
        let grid = image::open(out.join(GRID_PNG)).unwrap().to_rgb8();
        let matrix = image::open(out.join(MATRIX_PNG)).unwrap().to_rgb8();

        let linux_col = image::imageops::crop_imm(&matrix, 2 * cw, 0, cw, ch).to_image();
        let grid_tile = image::imageops::crop_imm(&grid, 0, 0, cw, ch).to_image();
        assert_eq!(linux_col.as_raw(), grid_tile.as_raw());

        // Sanity: platforms differ enough that a wrong pin would fail above —
        // macos and windows columns really do differ from linux.
        for p in 0..2usize {
            let other = image::imageops::crop_imm(&matrix, p as u32 * cw, 0, cw, ch).to_image();
            assert_ne!(
                other.as_raw(),
                grid_tile.as_raw(),
                "platform {p} collides with linux"
            );
        }
    }

    #[test]
    fn manifest_filters_the_rendered_set() {
        let pages = fixture_pages();
        let roots = three_roots("filter", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("filter_out");
        let opts = opts_in(&dir, &["misc_2605"]);
        let out = dir.path().join("assets");

        export_grid(&inputs, &opts, &out).unwrap();
        let tiles = read_index(&out)["tiles"].as_array().unwrap().clone();
        assert_eq!(tiles.len(), 1);
        assert_eq!(tiles[0]["id"], "misc_2605");
    }

    #[test]
    fn unknown_manifest_id_is_rejected() {
        let pages = fixture_pages();
        let roots = three_roots("unknownid", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("unknownid_out");
        let opts = opts_in(&dir, &["box_2502", "zzz_9999"]);
        assert!(export_grid(&inputs, &opts, &dir.path().join("o")).is_err());
    }

    #[test]
    fn manifest_id_absent_from_captures_is_rejected() {
        let pages = fixture_pages();
        let roots = three_roots("absent", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("absent_out");
        let opts = opts_in(&dir, &["box_2502", "dingbat_2702"]);
        assert!(export_grid(&inputs, &opts, &dir.path().join("o")).is_err());
    }

    #[test]
    fn empty_manifest_is_rejected() {
        let pages = fixture_pages();
        let roots = three_roots("emptyman", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("emptyman_out");
        std::fs::write(dir.path().join("manifest.rs"), "// nothing\n").unwrap();
        write_ucd_file(&dir.path().join("ucd.txt"), &[]);
        let opts = GridOptions {
            manifest: dir.path().join("manifest.rs"),
            ucd: dir.path().join("ucd.txt"),
        };
        assert!(export_grid(&inputs, &opts, &dir.path().join("o")).is_err());
    }

    #[test]
    fn sidecar_skew_between_platforms_is_rejected() {
        let pages = fixture_pages();
        let roots = three_roots("skew", &pages);
        // Corrupt windows' sidecar with one extra candidate row.
        let win_sidecar = roots._guards[1]
            .path()
            .join("artifacts")
            .join("sidecar.json");
        let mut v: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&win_sidecar).unwrap()).unwrap();
        v["rows"].as_array_mut().unwrap().push(serde_json::json!({
            "kind": "candidate",
            "id": "ascii_0021",
            "codepoint": 33,
            "block": "ascii",
            "page": 0,
            "page_row": 9,
            "expected_end_col": 8,
        }));
        std::fs::write(&win_sidecar, serde_json::to_vec(&v).unwrap()).unwrap();

        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("skew_out");
        let opts = opts_in(&dir, &["box_2502"]);
        assert!(export_grid(&inputs, &opts, &dir.path().join("o")).is_err());
    }

    #[test]
    fn wrong_page_count_is_rejected() {
        let pages = fixture_pages();
        let roots = three_roots("pagecount", &pages);
        // Drop page 1 from the linux root.
        std::fs::remove_file(
            roots._guards[2]
                .path()
                .join("pages")
                .join("shot_page_1.png"),
        )
        .unwrap();

        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("pagecount_out");
        let opts = opts_in(&dir, &["box_2502"]);
        assert!(export_grid(&inputs, &opts, &dir.path().join("o")).is_err());
    }

    #[test]
    fn invalid_platform_labels_are_rejected() {
        let pages = fixture_pages();
        let roots = three_roots("labels", &pages);
        let dir = tempfile_guard::DirGuard::new("labels_out");
        let opts = opts_in(&dir, &["box_2502"]);

        let mut inputs = inputs_from(&roots);
        inputs.truncate(2);
        assert!(export_grid(&inputs, &opts, &dir.path().join("a")).is_err());

        let mut inputs = inputs_from(&roots);
        inputs[0].label = "plan9".into();
        assert!(export_grid(&inputs, &opts, &dir.path().join("b")).is_err());

        let mut inputs = inputs_from(&roots);
        inputs[1].label = "linux".into(); // duplicate
        assert!(export_grid(&inputs, &opts, &dir.path().join("c")).is_err());
    }

    #[test]
    fn clamp_crop_never_panics_at_canvas_edges() {
        let img = GrayImage::from_pixel(10, 8, Luma([5]));
        let crop = clamp_crop(
            &img,
            CellBox {
                left: 8,
                top: 6,
                right: 20,
                bottom: 30,
            },
        );
        assert_eq!((crop.width(), crop.height()), (2, 2));
    }

    #[test]
    fn full_cell_crops_keep_edge_ink_of_full_width_glyphs() {
        // Regression: export used the assertion window (central 70% of the
        // cell), guillotining full-width primitives. Display crops must span
        // the entire cell and retain ink at both extreme columns.
        use crate::geom::{FgModel, estimate_background, suppress_structural_lines};
        let pages = vec![PageSpec {
            candidates: vec![(
                0x2588,
                icon_catalog::Block::BlockElements,
                IconKind::FullSpan,
            )],
        }];
        let root = capture_root("fulledge", &pages, LINUX_GEOM);
        let img = image::open(root.path().join("pages").join("shot_page_0.png")).unwrap();
        let gray = crate::grayscale(&img);
        let model = FgModel {
            bg: estimate_background(&gray),
            delta: 60.0,
        };
        let clean = suppress_structural_lines(&gray, &model);
        let sc: Sidecar = serde_json::from_value(crate::testutil::sidecar_json(&pages)).unwrap();
        let rows = sc.rows_for_page(0);
        let (cal, bands) =
            crate::page_geometry(&clean, &model, &rows).expect("fixture page must calibrate");

        let full = cal.cell_box_full(3, &bands[1]);
        let crop = clamp_crop(&gray, full);
        assert!(crop.width() >= 2 && crop.height() >= 2);
        let mid = crop.height() / 2;
        assert!(
            crop.get_pixel(0, mid)[0] > 128,
            "leftmost display column lost its ink"
        );
        assert!(
            crop.get_pixel(crop.width() - 1, mid)[0] > 128,
            "rightmost display column lost its ink"
        );

        // The old assertion window would NOT have covered the edges:
        let inner = cal.cell_box(3, &bands[1]);
        assert!(inner.left > full.left || inner.right < full.right);
    }

    #[test]
    fn repeated_exports_are_byte_identical() {
        let pages = fixture_pages();
        let roots = three_roots("det", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("det_out");
        let opts = opts_in(
            &dir,
            &["box_2502", "box_2500", "misc_2605", "geometric_25B2"],
        );

        export_grid(&inputs, &opts, &dir.path().join("a")).unwrap();
        export_grid(&inputs, &opts, &dir.path().join("b")).unwrap();

        for f in [GRID_PNG, MATRIX_PNG, SPECIMEN_PNG, INDEX_FILE] {
            let a = std::fs::read(dir.path().join("a").join(f)).unwrap();
            let b = std::fs::read(dir.path().join("b").join(f)).unwrap();
            assert_eq!(a, b, "{f} must be byte-deterministic");
        }
    }
}
