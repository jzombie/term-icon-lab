//! pixel-assert: Pass 2 — pixel-level assertions over captured screenshots.
//!
//! Exit codes: `0` all candidates pass, `1` verification failures (glyphs
//! rejected), `2` usage/internal errors.

mod checks;
mod geom;
mod grid;
mod schema;
#[cfg(test)]
mod testutil;
mod validate;
mod verdict;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use image::DynamicImage;

use crate::checks::{BleedVerdict, Thresholds};
use crate::geom::{
    Band, Calibration, FgModel, calibrate, estimate_background, find_text_bands, select_grid_bands,
    suppress_structural_lines,
};
use crate::schema::{Pass1Status, RowEntry, Sidecar};
use crate::validate::CaptureGate;
use crate::verdict::{Verdict, VerdictReport, merge};

#[derive(Parser, Debug)]
#[command(
    name = "pixel-assert",
    about = "Pass 2: pixel-level assertions over captured terminal pages."
)]
struct Cli {
    /// Page screenshots in page order (one per sidecar page).
    #[arg(long = "png", required_unless_present_any = ["validate_only", "export_grid"])]
    pngs: Vec<PathBuf>,

    /// Harness sidecar describing the render.
    #[arg(long, required_unless_present_any = ["validate_only", "export_grid"])]
    sidecar: Option<PathBuf>,

    /// Harness Pass-1 report.
    #[arg(long, required_unless_present_any = ["validate_only", "export_grid"])]
    pass1: Option<PathBuf>,

    /// Output verdicts.json path.
    #[arg(long, required_unless_present_any = ["validate_only", "export_grid"])]
    out: Option<PathBuf>,

    /// Minimum plausible-capture entropy in bits.
    #[arg(long, default_value_t = 0.02)]
    min_entropy: f64,

    /// Capture-gate-only mode: validate one PNG and exit (0 plausible, 1 not).
    /// Used by orchestrators between `.ready` and `.ack`.
    #[arg(long)]
    validate_only: Option<PathBuf>,

    /// Export preview PNGs + index for the icon ids listed in
    /// `--grid-manifest` instead of asserting verdicts.
    #[arg(
        long,
        conflicts_with_all = ["pngs", "sidecar", "pass1", "out", "validate_only"],
        requires = "grid_manifest"
    )]
    export_grid: Option<PathBuf>,

    /// Artifact root as label=dir (repeatable; requires --export-grid).
    #[arg(long = "grid-input", requires = "export_grid")]
    grid_inputs: Vec<String>,

    /// Generated manifest defining the exact rendering set (ids are scraped
    /// from it; the file is the single source of truth for the shipped set).
    #[arg(long, requires = "export_grid")]
    grid_manifest: PathBuf,

    /// Vendored UnicodeData extract supplying `unicode_name` for
    /// grid-index.json.
    #[arg(
        long,
        requires = "export_grid",
        default_value = "crates/manifest-gen/ucd/UnicodeData.txt"
    )]
    grid_ucd: PathBuf,
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    if cli.export_grid.is_some() {
        return match grid::run_export_grid(&cli) {
            Ok(()) => std::process::ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("pixel-assert: {err:#}");
                std::process::ExitCode::from(2)
            }
        };
    }
    if let Some(png) = &cli.validate_only {
        return match validate_one(&cli, png) {
            Ok(true) => std::process::ExitCode::SUCCESS,
            Ok(false) => std::process::ExitCode::from(1),
            Err(err) => {
                eprintln!("pixel-assert: {err:#}");
                std::process::ExitCode::from(2)
            }
        };
    }
    match run(&cli) {
        Ok(all_pass) if all_pass => std::process::ExitCode::SUCCESS,
        Ok(_) => std::process::ExitCode::from(1),
        Err(err) => {
            eprintln!("pixel-assert: {err:#}");
            std::process::ExitCode::from(2)
        }
    }
}

/// Capture-gate-only path used by orchestrators mid-loop.
fn validate_one(cli: &Cli, png: &PathBuf) -> Result<bool, anyhow::Error> {
    let img = image::open(png).with_context(|| format!("open {}", png.display()))?;
    let gate = CaptureGate {
        min_entropy_bits: cli.min_entropy,
        ..CaptureGate::default()
    };
    let stats = validate::measure(&img);
    let ok = stats.passes(&gate);
    println!(
        "capture gate {}: {stats:?} => {}",
        png.display(),
        if ok { "plausible" } else { "REJECTED" }
    );
    Ok(ok)
}

fn run(cli: &Cli) -> Result<bool, anyhow::Error> {
    let sidecar_path = cli
        .sidecar
        .as_deref()
        .expect("--sidecar is required unless --validate-only is set");
    let pass1_path = cli
        .pass1
        .as_deref()
        .expect("--pass1 is required unless --validate-only is set");
    let out_path = cli
        .out
        .as_deref()
        .expect("--out is required unless --validate-only is set");

    let sidecar: Sidecar = serde_json::from_slice(
        &std::fs::read(sidecar_path).with_context(|| format!("read {}", sidecar_path.display()))?,
    )
    .with_context(|| format!("parse {}", sidecar_path.display()))?;
    let pass1: schema::Pass1Report = serde_json::from_slice(
        &std::fs::read(pass1_path).with_context(|| format!("read {}", pass1_path.display()))?,
    )
    .with_context(|| format!("parse {}", pass1_path.display()))?;

    if cli.pngs.len() != sidecar.pages as usize {
        anyhow::bail!(
            "expected {} page images, got {}",
            sidecar.pages,
            cli.pngs.len()
        );
    }

    let gate = CaptureGate {
        min_entropy_bits: cli.min_entropy,
        ..CaptureGate::default()
    };

    // Pass-1 statuses indexed by candidate id.
    let mut pass1_by_id: HashMap<&str, Pass1Status> = HashMap::new();
    for r in &pass1.results {
        pass1_by_id.insert(r.id.as_str(), r.status);
    }

    let mut structural_fail: Option<String> = None;
    let mut verdicts: Vec<Verdict> = Vec::new();

    for (page_idx, png_path) in cli.pngs.iter().enumerate() {
        let page = page_idx as u32;
        let img = image::open(png_path).with_context(|| format!("open {}", png_path.display()))?;
        let gray = grayscale(&img);

        // Capture validity gate: a broken capture must fail loudly here,
        // before it can masquerade as thousands of glyph failures.
        if !structural_fail.is_some() {
            let stats = validate::measure(&img);
            if !stats.passes(&gate) {
                structural_fail = Some(format!("page {page}: implausible capture ({stats:?})"));
                continue;
            }
        }

        let bg = estimate_background(&gray);
        let model = FgModel { bg, delta: 60.0 };
        // Window chrome (borders, scrollbars, title bars) bridges the gaps
        // between text rows; suppress it, locate rows via the boundary-bar
        // strip, and keep the most uniform run if remnants survive.
        let clean = suppress_structural_lines(&gray, &model);
        let expected_rows = sidecar.rows_for_page(page);
        let (cal, bands) = match page_geometry(&clean, &model, &expected_rows) {
            Ok(ok) => ok,
            Err(geom::GeometryError::BandMismatch { found, expected }) => {
                structural_fail = Some(format!(
                    "page {page}: {found} text bands rendered but sidecar declares {expected} rows"
                ));
                continue;
            }
            Err(e) => {
                structural_fail = Some(format!("page {page}: calibration failed: {e:?}"));
                continue;
            }
        };

        match analyze_page(&clean, &model, cal, &bands, &expected_rows) {
            Ok(outcomes) => {
                for outcome in outcomes {
                    let RowEntry::Candidate { id, codepoint, .. } = outcome.entry else {
                        continue;
                    };
                    let status = pass1_by_id
                        .get(id.as_str())
                        .copied()
                        .unwrap_or(Pass1Status::Inconclusive);
                    verdicts.push(merge(
                        id.as_str(),
                        *codepoint,
                        status,
                        outcome.visible,
                        outcome.no_bleed,
                    ));
                }
            }
            Err(e) => {
                structural_fail = Some(format!("page {page}: calibration failed: {e:?}"));
            }
        }
    }

    // Structural failure ⇒ every candidate fails closed.
    if structural_fail.is_some() {
        verdicts.clear();
        for entry in &sidecar.rows {
            if let RowEntry::Candidate { id, codepoint, .. } = entry {
                verdicts.push(merge(
                    id,
                    *codepoint,
                    Pass1Status::Inconclusive,
                    false,
                    false,
                ));
            }
        }
    }

    let report = VerdictReport {
        schema_version: 1,
        platform: sidecar.platform.clone(),
        host: sidecar.host.clone(),
        pass1_support: pass1.pass1_support && structural_fail.is_none(),
        structural_fail,
        results: verdicts,
    };

    let passing = report.passing_ids().len();
    std::fs::write(out_path, serde_json::to_vec_pretty(&report)?)
        .with_context(|| format!("write {}", out_path.display()))?;

    println!(
        "verdicts: {}/{} candidates pass{}",
        passing,
        report.results.len(),
        if report.structural_fail.is_some() {
            " [STRUCTURAL FAIL]"
        } else {
            ""
        }
    );
    Ok(report.structural_fail.is_none() && passing == report.results.len())
}

/// One analyzed row of a page.
struct RowOutcome<'a> {
    entry: &'a RowEntry,
    visible: bool,
    no_bleed: bool,
}

/// Locate the rendered grid: band detection + uniform-run selection, then
/// calibration from the control row. Band-count mismatches surface as
/// `GeometryError::BandMismatch` so callers keep their own error context.
fn page_geometry(
    gray: &image::GrayImage,
    model: &FgModel,
    expected_rows: &[&RowEntry],
) -> Result<(Calibration, Vec<Band>), geom::GeometryError> {
    let mut bands = find_text_bands(gray, model, expected_rows.len());
    if bands.len() > expected_rows.len() {
        bands = select_grid_bands(&bands, expected_rows.len());
    }
    if bands.len() != expected_rows.len() {
        return Err(geom::GeometryError::BandMismatch {
            found: bands.len(),
            expected: expected_rows.len(),
        });
    }
    let control_row0 = expected_rows
        .first()
        .map(|r| usize::from(r.page_row()))
        .unwrap_or(0);
    let control_band = bands
        .get(control_row0)
        .ok_or(geom::GeometryError::MissingBars)?;
    let cal = calibrate(gray, model, control_band)?;
    Ok((cal, bands))
}

/// Assert every candidate row through fixed-grid slicing against a
/// pre-computed calibration (see [`page_geometry`]).
fn analyze_page<'a>(
    gray: &image::GrayImage,
    model: &FgModel,
    cal: Calibration,
    bands: &[Band],
    expected_rows: &'a [&'a RowEntry],
) -> Result<Vec<RowOutcome<'a>>, geom::GeometryError> {
    let control_row0 = expected_rows
        .first()
        .map(|r| usize::from(r.page_row()))
        .unwrap_or(0);
    // Reference sentinel-B mask harvested from the clean `| A B |` slot.
    let control_band = bands
        .get(control_row0)
        .ok_or(geom::GeometryError::MissingBars)?;
    let reference_b = checks::crop_mask(gray, model, &cal.cell_box(4, control_band));

    let mut out = Vec::with_capacity(expected_rows.len());
    for row in expected_rows {
        match row {
            RowEntry::Control { .. } => out.push(RowOutcome {
                entry: row,
                visible: true,
                no_bleed: true,
            }),
            RowEntry::Candidate { block, .. } => {
                let band = &bands[usize::from(row.page_row())];
                let icon_box = cal.cell_box(3, band);
                let visible = checks::check_visibility(
                    gray,
                    model,
                    &icon_box,
                    *block,
                    &Thresholds::default(),
                )
                .is_ok();
                let no_bleed = checks::check_bleed(
                    gray,
                    model,
                    &cal,
                    band,
                    &reference_b,
                    &Thresholds::default(),
                ) == BleedVerdict::Clean;
                out.push(RowOutcome {
                    entry: row,
                    visible,
                    no_bleed,
                });
            }
        }
    }
    Ok(out)
}

/// Rec.601 grayscale, matching the luma used by the capture gate.
fn grayscale(img: &DynamicImage) -> image::GrayImage {
    use image::imageops::colorops::grayscale as ops_grayscale;
    ops_grayscale(img)
}

/// Synthetic end-to-end fixtures: full pages are drawn programmatically, fed
/// through `run()` with real sidecar/pass1/PNG files, and the merged verdicts
/// are asserted. Fixture synthesis lives in [`crate::testutil`] so the
/// export-grid tests reuse it; this module carries the pipeline assertions.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{
        Fixture, IconKind, PageSpec, build_fixture, pass1_json, sidecar_json, tempfile_guard,
    };
    use icon_catalog::Block;
    use image::{GrayImage, Luma};

    const BG: u8 = crate::testutil::BG;

    fn read_verdicts(fx: &Fixture) -> VerdictReport {
        let raw = std::fs::read(fx.cli.out.as_deref().unwrap()).unwrap();
        serde_json::from_slice(&raw).unwrap()
    }

    #[test]
    fn perfect_pages_pass_end_to_end() {
        let pages = vec![
            PageSpec {
                candidates: vec![
                    (0x2502, Block::BoxDrawing, IconKind::Solid),
                    (0x0041, Block::Ascii, IconKind::Solid),
                    (0x2801, Block::Braille, IconKind::BrailleDot), // single dot!
                ],
            },
            PageSpec {
                candidates: vec![
                    (0x2588, Block::BlockElements, IconKind::Solid),
                    (0x2500, Block::BoxDrawing, IconKind::Solid),
                ],
            },
        ];
        let fx = build_fixture("perfect", &pages, 2, pass1_json(&[])); // no pass1 data → inconclusive!
        assert!(
            !run(&fx.cli).unwrap(),
            "missing pass1 data must fail closed"
        );
        let report = read_verdicts(&fx);
        assert!(
            report
                .results
                .iter()
                .all(|v| v.pass1 == "inconclusive" && v.overall == "fail")
        );
    }

    #[test]
    fn perfect_pages_pass_with_pass1_data() {
        let pages = vec![PageSpec {
            candidates: vec![
                (0x2502, Block::BoxDrawing, IconKind::Solid),
                (0x0041, Block::Ascii, IconKind::Solid),
                (0x2801, Block::Braille, IconKind::BrailleDot),
                (0x2591, Block::BlockElements, IconKind::Solid),
            ],
        }];
        // Catalog ids resolved by the fixture sidecar, in row order.
        let statuses = [
            ("box_2502", "pass"),
            ("ascii_0041", "pass"),
            ("braille_2801", "pass"),
            ("block_2591", "pass"),
        ];
        let fx = build_fixture("perfect2", &pages, 1, pass1_json(&statuses));
        let ok = run(&fx.cli).unwrap();
        let report = read_verdicts(&fx);
        eprintln!(
            "DEBUG ok={ok} structural={:?} results={:#?}",
            report.structural_fail, report.results
        );
        assert!(ok);
    }

    #[test]
    fn wide_glyph_and_blank_fail_with_precise_reasons() {
        let pages = vec![PageSpec {
            candidates: vec![
                (0x2502, Block::BoxDrawing, IconKind::Solid),   // passes
                (0x2588, Block::BlockElements, IconKind::Wide), // bleeds
                (0x2801, Block::Braille, IconKind::Blank),      // invisible
            ],
        }];
        let statuses = [
            ("box_2502", "pass"),
            ("block_2588", "fail"), // PTY-level expansion detected by harness
            ("braille_2801", "pass"),
        ];
        let fx = build_fixture("mixed", &pages, 1, pass1_json(&statuses));
        assert!(!run(&fx.cli).unwrap());
        let report = read_verdicts(&fx);
        assert_eq!(report.results[0].overall, "pass");
        assert_eq!(report.results[1].overall, "fail");
        assert!(
            report
                .results
                .iter()
                .all(|v| v.visible || v.id == "braille_2801")
        );
        assert!(!report.results[1].no_bleed);
        assert!(!report.results[2].visible);
    }

    #[test]
    fn wrong_png_count_is_a_usage_error() {
        let pages = vec![PageSpec {
            candidates: vec![(0x2502, Block::BoxDrawing, IconKind::Solid)],
        }];
        let fx = build_fixture("count", &pages, 0, pass1_json(&[])); // 0 pngs, 1 page
        assert!(run(&fx.cli).is_err());
    }

    #[test]
    fn blank_capture_triggers_structural_fail() {
        let pages = vec![PageSpec {
            candidates: vec![(0x2502, Block::BoxDrawing, IconKind::Solid)],
        }];
        let dir = tempfile_guard::DirGuard::new("blankcap");
        let png = dir.path().join("shot_page_0.png");
        let img = GrayImage::from_pixel(200, 40, Luma([BG]));
        image::save_buffer(
            &png,
            img.as_raw(),
            img.width(),
            img.height(),
            image::ColorType::L8,
        )
        .unwrap();
        let sidecar_path = dir.path().join("sidecar.json");
        std::fs::write(&sidecar_path, sidecar_json(&pages).to_string()).unwrap();
        let pass1_path = dir.path().join("pass1.json");
        std::fs::write(&pass1_path, pass1_json(&[]).to_string()).unwrap();

        let cli = Cli {
            pngs: vec![png],
            sidecar: Some(sidecar_path),
            pass1: Some(pass1_path),
            out: Some(dir.path().join("v.json")),
            ..crate::testutil::base_cli()
        };
        assert!(!run(&cli).unwrap());
        let report: VerdictReport =
            serde_json::from_slice(&std::fs::read(dir.path().join("v.json")).unwrap()).unwrap();
        assert!(report.structural_fail.is_some());
        assert!(!report.pass1_support);
        assert!(report.results.iter().all(|v| v.overall == "fail"));
    }
}
