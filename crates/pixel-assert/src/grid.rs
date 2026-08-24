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
/// target overall width, cell padding/gap metrics, and band heights. The
/// canvas mixes three band types with different heights (legend, block
/// headers, tile rows), so Y positions are always tracked cumulatively.
const LABEL_SCALE: u32 = 2;
const SPECIMEN_TARGET_WIDTH: f64 = 480.0;
const SPECIMEN_PAD_X: u32 = 6;
const SPECIMEN_PAD_Y: u32 = 5;
const SPECIMEN_LABEL_GAP: u32 = 6;
const INNER_GAP: u32 = 4;
const LABEL_COLOR: Rgb<u8> = Rgb([200, 200, 200]);
/// Height of the M/W-L platform legend strip at the canvas top.
const LEGEND_BAND_H: u32 = font5x7::FONT_H + 6;
/// Height of a named Unicode-block header band.
const HEADER_BAND_H: u32 = font5x7::FONT_H * 2 + 8;

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
    // Each cell = full-cell renders from ALL THREE platforms (×2 nearest-
    // neighbour, ordered macOS | Windows | Linux) above the `U+XXXX`
    // activation label. A platform legend band tops the canvas; Unicode
    // blocks are separated by NAMED header bands.
    let zoom = |label: &str, id: &str| -> anyhow::Result<RgbImage> {
        let crop = crops
            .get(&(label, id))
            .with_context(|| format!("missing {label} crop for '{id}'"))?;
        Ok(image::imageops::resize(
            crop,
            crop.width() * LABEL_SCALE,
            crop.height() * LABEL_SCALE,
            FilterType::Nearest,
        ))
    };

    let glyph_w = crops.values().map(RgbImage::width).max().unwrap_or(1) * LABEL_SCALE;
    let glyph_h = crops.values().map(RgbImage::height).max().unwrap_or(1) * LABEL_SCALE;
    let label_w = font5x7::text_width("U+0000".len(), LABEL_SCALE);
    let inner_w =
        glyph_w * PLATFORM_ORDER.len() as u32 + INNER_GAP * (PLATFORM_ORDER.len() - 1) as u32;
    let cell_w = inner_w.max(label_w) + SPECIMEN_PAD_X * 2;
    let cell_h = glyph_h + SPECIMEN_LABEL_GAP + font5x7::FONT_H * LABEL_SCALE + SPECIMEN_PAD_Y * 2;
    let specimen_cols = ((SPECIMEN_TARGET_WIDTH / cell_w as f64).floor() as usize).clamp(3, 8);

    // Band walk: legend row 0, then per-block header band + chunked tile
    // rows. Heights differ per band type, so Y is tracked cumulatively —
    // never derived by scalar row multiplication.
    struct SpecimenTile {
        ti: usize,
        col: usize,
        canvas_row: usize,
        pixel_y: u32,
    }
    let mut spec_tiles: Vec<SpecimenTile> = Vec::with_capacity(targets.len());
    let mut header_rows: BTreeMap<usize, (String, u32)> = BTreeMap::new();

    let mut y = LEGEND_BAND_H;
    let mut canvas_row = 1usize; // row 0 is the platform legend
    let mut ti = 0usize;
    while ti < targets.len() {
        let block = targets[ti].block;
        let run_start = ti;
        while ti < targets.len() && targets[ti].block == block {
            ti += 1;
        }
        header_rows.insert(canvas_row, (block_display_name(block).to_string(), y));
        y += HEADER_BAND_H;
        canvas_row += 1;
        for (row_idx, chunk) in targets[run_start..ti].chunks(specimen_cols).enumerate() {
            for (col_idx, _t) in chunk.iter().enumerate() {
                // Global index: run offset + row-major position within the run.
                let target_idx = run_start + (row_idx * specimen_cols) + col_idx;
                spec_tiles.push(SpecimenTile {
                    ti: target_idx,
                    col: col_idx,
                    canvas_row,
                    pixel_y: y,
                });
            }
            y += cell_h;
            canvas_row += 1;
        }
    }
    let catalog_h = y;

    let mut catalog_canvas =
        RgbImage::from_pixel(specimen_cols as u32 * cell_w, catalog_h, Rgb([0, 0, 0]));

    // Platform legend: M W L repeated across every active column, each
    // letter centred over its platform's slot.
    let letter_x = |c: usize, p: usize| -> i64 {
        let x = c as u32 * cell_w
            + SPECIMEN_PAD_X
            + p as u32 * (glyph_w + INNER_GAP)
            + (glyph_w.saturating_sub(font5x7::FONT_W)) / 2;
        i64::from(x)
    };
    let legend_y = i64::from((LEGEND_BAND_H - font5x7::FONT_H) / 2);
    for c in 0..specimen_cols {
        for (p, ch) in ["M", "W", "L"].iter().enumerate() {
            font5x7::draw_text(
                &mut catalog_canvas,
                letter_x(c, p),
                legend_y,
                ch,
                1,
                LABEL_COLOR,
            );
        }
    }

    // Block header bands: named, left-aligned at scale 2.
    for (name, hy) in header_rows.values() {
        font5x7::draw_text(
            &mut catalog_canvas,
            i64::from(SPECIMEN_PAD_X),
            i64::from(*hy + (HEADER_BAND_H - font5x7::FONT_H * 2) / 2),
            name,
            2,
            LABEL_COLOR,
        );
    }

    // Tiles: three zoomed crops + activation label per cell.
    for st in &spec_tiles {
        let t = targets[st.ti];
        let x0 = st.col as u32 * cell_w;
        for (p, label) in PLATFORM_ORDER.iter().enumerate() {
            let g = zoom(label, t.id.as_str())?;

            image::imageops::overlay(
                &mut catalog_canvas,
                &g,
                i64::from(
                    x0 + SPECIMEN_PAD_X
                        + p as u32 * (glyph_w + INNER_GAP)
                        + (glyph_w - g.width()) / 2,
                ),
                i64::from(st.pixel_y + SPECIMEN_PAD_Y + (glyph_h - g.height()) / 2),
            );
        }
        let text = format!("U+{:04X}", t.codepoint);
        font5x7::draw_text(
            &mut catalog_canvas,
            i64::from(x0 + (cell_w - label_w) / 2),
            i64::from(st.pixel_y + SPECIMEN_PAD_Y + glyph_h + SPECIMEN_LABEL_GAP),
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
            let st = &spec_tiles[i];
            TileMeta {
                index: i,
                grid_row,
                grid_col,
                specimen_row: st.canvas_row,
                specimen_col: st.col,
                canvas_row: st.canvas_row,
                pixel_y: st.pixel_y,
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
    let header_map: BTreeMap<usize, &str> = header_rows
        .iter()
        .map(|(row, (name, _))| (*row, name.as_str()))
        .collect();
    let index = GridIndexFile {
        schema_version: 4,
        columns: GRID_COLUMNS,
        specimen_columns: specimen_cols,
        specimen_platform_order: PLATFORM_ORDER,
        block_header_rows: header_map,
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
    specimen_platform_order: [&'a str; 3],
    block_header_rows: BTreeMap<usize, &'a str>,
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
    /// Visual row on the specimen canvas (legend = row 0, headers included).
    canvas_row: usize,
    /// Cumulative pixel Y of this tile row's top edge.
    pixel_y: u32,
    id: &'a str,
    codepoint: u32,
    block: icon_catalog::Block,
    unicode_name: &'a str,
    matrix_col: BTreeMap<&'static str, usize>,
}

/// Human-readable name of a Unicode block, as drawn in specimen headers.
fn block_display_name(block: icon_catalog::Block) -> &'static str {
    match block {
        icon_catalog::Block::Ascii => "ASCII",
        icon_catalog::Block::Arrows => "ARROWS",
        icon_catalog::Block::BoxDrawing => "BOX DRAWING",
        icon_catalog::Block::BlockElements => "BLOCK ELEMENTS",
        icon_catalog::Block::GeometricShapes => "GEOMETRIC SHAPES",
        icon_catalog::Block::MiscSymbols => "MISC SYMBOLS",
        icon_catalog::Block::Dingbats => "DINGBATS",
        icon_catalog::Block::Braille => "BRAILLE",
    }
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
        let v: serde_json::Value =
            serde_json::from_slice(&std::fs::read(out.join(INDEX_FILE)).unwrap()).unwrap();
        if std::env::var_os("TI_DEBUG").is_some() {
            eprintln!("DBGIDX {}", serde_json::to_string(&v).unwrap());
        }
        v
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
        assert_eq!(index["schema_version"], 4);
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

        // Specimen chart (schema v4): three platforms per cell, named
        // block headers, explicit visual coordinates from the band walk.
        assert!(out.join(SPECIMEN_PNG).is_file(), "catalog chart written");
        let spec_cols = index["specimen_columns"].as_u64().unwrap() as u32;
        assert_eq!(spec_cols, 3);
        assert_eq!(
            index["specimen_platform_order"],
            serde_json::json!(["macos", "windows", "linux"])
        );
        let catalog = image::open(out.join(SPECIMEN_PNG)).unwrap();
        assert!(
            (400..=520).contains(&catalog.width()),
            "specimen width {} outside font-repo range",
            catalog.width()
        );

        // Band walk: legend(13) + headers(22 each) + tile rows(cell_h each).
        // Fixture spans three blocks ⇒ three header bands.
        // Tallest fixture native cell = WINDOWS_GEOM.cell_h (24) ⇒ zoomed
        // 48; specimen cell height = 48 + gap 6 + label 14 + pad 10 = 78.
        let spec_cell_h = 78u64;
        let headers: BTreeMap<String, String> =
            serde_json::from_value(index["block_header_rows"].clone()).unwrap();
        assert_eq!(
            headers,
            BTreeMap::from([
                ("1".to_string(), "BOX DRAWING".to_string()),
                ("3".to_string(), "GEOMETRIC SHAPES".to_string()),
                ("5".to_string(), "MISC SYMBOLS".to_string()),
            ]),
            "\nindex was:\n{}",
            serde_json::to_string_pretty(&index).unwrap()
        );
        // tiles[0,1] → BOX DRAWING row; tiles[2] geometric; tiles[3] misc.
        let rows: Vec<u64> = tiles
            .iter()
            .map(|t| t["canvas_row"].as_u64().unwrap())
            .collect();
        assert_eq!(
            rows,
            [2, 2, 4, 6],
            "\nindex:\n{}",
            serde_json::to_string_pretty(&index).unwrap()
        );
        let pys: Vec<u64> = tiles
            .iter()
            .map(|t| t["pixel_y"].as_u64().unwrap())
            .collect();
        assert_eq!(
            pys,
            [
                LEGEND_BAND_H as u64 + HEADER_BAND_H as u64,
                LEGEND_BAND_H as u64 + HEADER_BAND_H as u64,
                LEGEND_BAND_H as u64 + HEADER_BAND_H as u64 + spec_cell_h + HEADER_BAND_H as u64,
                LEGEND_BAND_H as u64
                    + HEADER_BAND_H as u64
                    + spec_cell_h
                    + HEADER_BAND_H as u64
                    + spec_cell_h
                    + HEADER_BAND_H as u64,
            ],
            "\nindex:\n{}",
            serde_json::to_string_pretty(&index).unwrap()
        );
        assert_eq!(tiles[0]["specimen_col"], 0);
        assert_eq!(tiles[1]["specimen_col"], 1);
        // Canvas height: legend + per-block (header band + tile row).
        // Fixture blocks hold 2/1/1 tiles ⇒ one tile row each at cols=3.
        assert_eq!(
            catalog.height() as u64,
            LEGEND_BAND_H as u64 + 3 * (HEADER_BAND_H as u64 + spec_cell_h)
        );
    }

    /// Every specimen cell must show ALL THREE platforms: the ink width of
    /// a `FullSpan` glyph scales linearly with each platform's pitch, so the
    /// three cell thirds must carry strictly ordered ink widths — and the
    /// legend must label every column.
    /// A block run LONGER than one specimen row must wrap onto new rows
    /// without re-rendering its first row — the chunk-offset bug shipped
    /// identical glyphs repeated vertically down long blocks.
    #[test]
    fn long_block_run_wraps_without_duplicates() {
        // 7 BoxDrawing candidates ⇒ chunks of 3/3/1 at specimen_cols = 3.
        let pages = vec![PageSpec {
            candidates: (0x2500u32..=0x2506)
                .map(|cp| (cp, icon_catalog::Block::BoxDrawing, IconKind::FullSpan))
                .collect(),
        }];
        let roots = three_roots("wrap", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("wrap_out");
        let opts = opts_in(
            &dir,
            &[
                "box_2500", "box_2501", "box_2502", "box_2503", "box_2504", "box_2505", "box_2506",
            ],
        );
        let out = dir.path().join("assets");

        export_grid(&inputs, &opts, &out).unwrap();

        let index = read_index(&out);
        assert_eq!(index["schema_version"], 4);
        let tiles = index["tiles"].as_array().unwrap().clone();
        assert_eq!(tiles.len(), 7, "every target rendered exactly once");
        let ids: Vec<&str> = tiles.iter().map(|t| t["id"].as_str().unwrap()).collect();
        assert_eq!(
            ids,
            [
                "box_2500", "box_2501", "box_2502", "box_2503", "box_2504", "box_2505", "box_2506"
            ],
            "no duplicates; codepoint order preserved across wrapped rows"
        );
        // Anti-bug pin: the first tile of row 2 is the FOURTH codepoint —
        // under the chunk-offset bug it was a repeat of the first.
        assert_eq!(tiles[3]["id"], "box_2503");
        let cols_seq: Vec<u64> = tiles
            .iter()
            .map(|t| t["specimen_col"].as_u64().unwrap())
            .collect();
        assert_eq!(cols_seq, [0, 1, 2, 0, 1, 2, 0]);
        let rows_seq: Vec<u64> = tiles
            .iter()
            .map(|t| t["canvas_row"].as_u64().unwrap())
            .collect();
        assert_eq!(rows_seq, [2, 2, 2, 3, 3, 3, 4]);
        // Same block ⇒ consecutive rows, no separators between chunks.

        // Specimen cell height from fixture metrics: tallest native cell is
        // WINDOWS_GEOM.cell_h = 24 ⇒ zoomed 48; + gap 6 + label 14 + pad 10.
        let spec_cell_h = 78u64;
        let pys: Vec<u64> = tiles
            .iter()
            .map(|t| t["pixel_y"].as_u64().unwrap())
            .collect();
        // Rows hold 3/3/1 tiles; each row starts one spec_cell_h after the
        // previous, following the single header band.
        let r0 = LEGEND_BAND_H as u64 + HEADER_BAND_H as u64;
        let expected_pys = [
            r0,
            r0,
            r0,
            r0 + spec_cell_h,
            r0 + spec_cell_h,
            r0 + spec_cell_h,
            r0 + 2 * spec_cell_h,
        ];
        assert_eq!(pys, expected_pys);

        // Pixel-level anti-duplication: each label strip encodes its own id,
        // so identical strips would mean duplicate renders.
        let catalog = image::open(out.join(SPECIMEN_PNG)).unwrap().to_rgb8();
        let col_w = catalog.width() / 3;
        let mut label_strips: Vec<Vec<u8>> = Vec::new();
        for t in &tiles {
            // Tallest zoomed glyph band = WINDOWS native cell_h 24 ×2.
            let py =
                t["pixel_y"].as_u64().unwrap() as u32 + SPECIMEN_PAD_Y + 48 + SPECIMEN_LABEL_GAP;
            // Sample the FULL centred label — clipping it would leave only
            // the shared `U+25…` prefix and mask id differences.
            let lx = t["specimen_col"].as_u64().unwrap() as u32 * col_w
                + (col_w - font5x7::text_width(6, LABEL_SCALE)) / 2;
            let mut strip: Vec<u8> = Vec::new();
            for y in py..py + font5x7::FONT_H * LABEL_SCALE {
                for x in lx..lx + font5x7::text_width(6, LABEL_SCALE) {
                    strip.push(catalog.get_pixel(x, y)[0]);
                }
            }
            label_strips.push(strip);
        }
        for (i, a) in label_strips.iter().enumerate() {
            for (j, b) in label_strips.iter().enumerate().skip(i + 1) {
                assert_ne!(a, b, "tiles {i} and {j} rendered identical pixels");
            }
        }
    }

    /// Specimen chart basics on a single-tile export: all three platform
    /// renders present in their ordered slots, M/W/L legend across every
    /// column, named block header band drawn.
    #[test]
    fn catalog_cells_show_all_three_platforms_with_legend() {
        let pages = vec![PageSpec {
            candidates: vec![(
                0x2588,
                icon_catalog::Block::BlockElements,
                IconKind::FullSpan,
            )],
        }];
        let roots = three_roots("pin3", &pages);
        let inputs = inputs_from(&roots);
        let dir = tempfile_guard::DirGuard::new("pin3_out");
        let opts = opts_in(&dir, &["block_2588"]);
        let out = dir.path().join("assets");

        export_grid(&inputs, &opts, &out).unwrap();

        let index = read_index(&out);
        assert_eq!(index["schema_version"], 4);
        assert_eq!(index["specimen_columns"], 3);
        assert_eq!(
            index["block_header_rows"],
            serde_json::json!({ "1": "BLOCK ELEMENTS" })
        );
        let tile = &index["tiles"][0];
        assert_eq!(tile["canvas_row"], 2);
        assert_eq!(tile["pixel_y"], LEGEND_BAND_H + HEADER_BAND_H);
        assert_eq!(tile["specimen_col"], 0);

        let catalog = image::open(out.join(SPECIMEN_PNG)).unwrap().to_rgb8();

        // Platform slots by pitch: macOS 28px < Linux 36px < Windows 44px
        // of ink (FullSpan zoomed ×LABEL_SCALE), left → right.
        let mut widths = Vec::new();
        for p in 0..3usize {
            let x0 = SPECIMEN_PAD_X + p as u32 * (glyph_w_for_specimen() + INNER_GAP);
            let mut min_x = None;
            let mut max_x = None;
            let top = tile["pixel_y"].as_u64().unwrap() as u32 + SPECIMEN_PAD_Y;
            for y in top..top + 48 {
                for dx in 0..glyph_w_for_specimen() {
                    if catalog.get_pixel(x0 + dx, y)[0] > 128 {
                        min_x = Some(min_x.map_or(dx, |v: u32| v.min(dx)));
                        max_x = Some(max_x.map_or(dx, |v: u32| v.max(dx)));
                    }
                }
            }
            widths.push(max_x.unwrap() - min_x.unwrap() + 1);
        }
        assert_eq!(widths, vec![28, 44, 36], "slots follow PLATFORM_ORDER");

        // Legend letters M/W/L over every column's three slots.
        let col_w = catalog.width() / 3;
        for c in 0..3usize {
            for p in 0..3usize {
                let lx = c as u32 * col_w
                    + SPECIMEN_PAD_X
                    + p as u32 * (glyph_w_for_specimen() + INNER_GAP)
                    + (glyph_w_for_specimen().saturating_sub(font5x7::FONT_W)) / 2;
                let mut strip: Vec<u8> = Vec::new();
                for y in 0..LEGEND_BAND_H {
                    for x in lx..lx + font5x7::FONT_W {
                        strip.push(catalog.get_pixel(x, y)[0]);
                    }
                }
                assert!(
                    strip.iter().any(|v| *v > 128),
                    "legend letter missing at column {c} platform {p}"
                );
            }
        }

        // Header band carries rendered text.
        let mut strip: Vec<u8> = Vec::new();
        for y in LEGEND_BAND_H..LEGEND_BAND_H + HEADER_BAND_H {
            for x in SPECIMEN_PAD_X..SPECIMEN_PAD_X + 60 {
                strip.push(catalog.get_pixel(x, y)[0]);
            }
        }
        assert!(
            strip.iter().any(|v| *v > 128),
            "block header band must contain rendered text"
        );
    }

    /// Zoomed width of the widest platform crop in the specimen chart
    /// (windows FullSpan native 22 px × LABEL_SCALE).
    fn glyph_w_for_specimen() -> u32 {
        22 * LABEL_SCALE
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
