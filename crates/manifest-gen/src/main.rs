//! manifest-gen: intersect per-platform verdicts through the AND gate and
//! deterministically generate `src/generated_manifest.rs`.
//!
//! Modes:
//! * `--inputs a.json b.json c.json` — intersect real CI verdicts.
//! * `--from-catalog` — bootstrap a provisional manifest from the unverified
//!   catalog (replaced by the first CI matrix run).
//!
//! Exit codes: `0` success, `2` usage / validation / internal error.

mod codegen;

use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use icon_catalog::{Block, candidates};
use serde::Deserialize;

/// Sanctioned host emulators per platform prefix.
const EXPECTED_HOSTS: &[(&str, &[&str])] = &[
    ("windows", &["wt", "conhost"]),
    ("linux", &["xterm", "gnome-terminal"]),
    ("macos", &["terminal-app"]),
];

#[derive(Parser, Debug)]
#[command(
    name = "manifest-gen",
    about = "AND-intersect platform verdicts and generate generated_manifest.rs."
)]
struct Cli {
    /// Per-platform verdicts.json files to intersect.
    #[arg(long, required_unless_present = "from_catalog", num_args = 1..)]
    inputs: Vec<PathBuf>,

    /// Bootstrap mode: assume the whole catalog passes (provisional!).
    #[arg(long)]
    from_catalog: bool,

    /// Output path for the generated Rust source.
    #[arg(long, default_value = "src/generated_manifest.rs")]
    out: PathBuf,
}

#[derive(Deserialize)]
struct VerdictsFile {
    platform: String,
    host: String,
    pass1_support: bool,
    structural_fail: Option<String>,
    results: Vec<VerdictEntry>,
}

#[derive(Deserialize)]
struct VerdictEntry {
    id: String,
    overall: String,
}

fn validate_host(platform: &str, host: &str) -> Result<(), String> {
    let lower = platform.to_ascii_lowercase();
    for (prefix, allowed) in EXPECTED_HOSTS {
        if lower.starts_with(prefix) {
            if allowed.contains(&host) {
                return Ok(());
            }
            return Err(format!(
                "platform '{platform}': host '{host}' not in sanctioned set {allowed:?}"
            ));
        }
    }
    Ok(())
}

fn entry_for(candidate: &icon_catalog::Candidate) -> codegen::Entry {
    let module = match candidate.block {
        Block::Ascii => "ascii",
        Block::Arrows => "arrows",
        Block::BoxDrawing => "box_drawing",
        Block::BlockElements => "block_elements",
        Block::GeometricShapes => "geometric_shapes",
        Block::MiscSymbols => "misc_symbols",
        Block::Dingbats => "dingbats",
        Block::Braille => "braille",
    };
    codegen::Entry::new(
        candidate.id.clone(),
        candidate.codepoint,
        module.to_owned(),
        candidate.const_name(),
        candidate.glyph(),
        candidate.fallback,
    )
}

fn generate_from_catalog() -> Result<Vec<codegen::Entry>, anyhow::Error> {
    eprintln!(
        "WARNING: --from-catalog produces a PROVISIONAL manifest from unverified
candidates; it is replaced by the first successful CI matrix run."
    );
    Ok(candidates().iter().map(entry_for).collect())
}

fn generate_from_inputs(inputs: &[PathBuf]) -> Result<Vec<codegen::Entry>, anyhow::Error> {
    let mut passing_sets: Vec<HashSet<String>> = Vec::new();
    let mut universes: Vec<HashSet<String>> = Vec::new();

    for path in inputs {
        let raw = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let file: VerdictsFile =
            serde_json::from_slice(&raw).with_context(|| format!("parse {}", path.display()))?;

        validate_host(&file.platform, &file.host)
            .map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;

        // Defense-in-depth: fail-fast in the harness makes both of these
        // unreachable in healthy pipelines.
        if !file.pass1_support {
            anyhow::bail!(
                "{}: pass1_support=false — a degraded run can never pass aggregation",
                path.display()
            );
        }
        if file.structural_fail.is_some() {
            anyhow::bail!(
                "{}: structural failure {:?} — nothing passes",
                path.display(),
                file.structural_fail
            );
        }

        let mut universe = HashSet::new();
        let mut passing = HashSet::new();
        for r in &file.results {
            universe.insert(r.id.clone());
            if r.overall == "pass" {
                passing.insert(r.id.clone());
            }
        }
        universes.push(universe);
        passing_sets.push(passing);
    }

    // Universes must be identical across platforms (guards catalog skew).
    for pair in universes.windows(2) {
        if pair[0] != pair[1] {
            anyhow::bail!("input verdict universes differ — catalogs out of sync between runs");
        }
    }

    // THE AND GATE: keep ids passing in EVERY platform input.
    let mut survivors: HashSet<String> = passing_sets.first().cloned().unwrap_or_default();
    for passing in &passing_sets {
        survivors = survivors.intersection(passing).cloned().collect();
    }

    Ok(candidates()
        .iter()
        .filter(|c| survivors.contains(&c.id))
        .map(entry_for)
        .collect())
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();

    let attempted = if cli.from_catalog {
        generate_from_catalog()
    } else if cli.inputs.is_empty() {
        Err(anyhow::anyhow!(
            "provide --inputs <verdicts.json>... or --from-catalog"
        ))
    } else {
        generate_from_inputs(&cli.inputs)
    };

    let mut entries = match attempted {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("manifest-gen: {err:#}");
            return std::process::ExitCode::from(2);
        }
    };

    // Deterministic order: by codepoint.
    entries.sort_by_key(|e| e.codepoint);

    let source = codegen::generate(&entries);
    if let Err(err) = std::fs::write(&cli.out, source) {
        eprintln!("manifest-gen: cannot write {}: {err}", cli.out.display());
        return std::process::ExitCode::from(2);
    }
    println!(
        "manifest-gen: wrote {} entries to {}",
        entries.len(),
        cli.out.display()
    );
    std::process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verdicts_json(platform: &str, host: &str, passing: &[&str], all: &[&str]) -> String {
        let results: Vec<serde_json::Value> = all
            .iter()
            .map(|id| {
                serde_json::json!({
                    "id": id,
                    "overall": if passing.contains(id) { "pass" } else { "fail" },
                })
            })
            .collect();
        serde_json::json!({
            "schema_version": 1,
            "platform": platform,
            "host": host,
            "pass1_support": true,
            "structural_fail": null,
            "results": results,
        })
        .to_string()
    }

    fn write_inputs(dir: &str, files: &[(&str, String)]) -> Vec<PathBuf> {
        let base = std::env::temp_dir().join(format!("ti_mg_{dir}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        files
            .iter()
            .map(|(name, content)| {
                let p = base.join(name);
                std::fs::write(&p, content).unwrap();
                p
            })
            .collect()
    }

    #[test]
    fn and_gate_intersects_passing_sets() {
        let all = ["ascii_0021", "box_2502", "block_2588", "braille_2801"];
        let inputs = write_inputs(
            "and",
            &[
                (
                    "linux.json",
                    verdicts_json("linux/xterm", "xterm", &all[..3], &all),
                ),
                (
                    "macos.json",
                    verdicts_json("macos/terminal-app", "terminal-app", &all[1..], &all),
                ),
                (
                    "windows.json",
                    verdicts_json("windows/wt", "wt", &[all[0], all[2], all[3]], &all),
                ),
            ],
        );
        let survivors = generate_from_inputs(&inputs).unwrap();
        let ids: HashSet<String> = survivors.into_iter().map(|e| e.id).collect();
        // Only block_2588 passes on ALL three platforms.
        assert_eq!(ids, HashSet::from(["block_2588".to_string()]));
    }

    #[test]
    fn unsanctioned_host_is_rejected() {
        let inputs = write_inputs(
            "host",
            &[(
                "win.json",
                verdicts_json("windows/wt", "gnome-terminal", &[], &["a"]),
            )],
        );
        assert!(generate_from_inputs(&inputs).is_err());
    }

    #[test]
    fn degraded_pass1_never_passes_aggregation() {
        let raw = r#"{
            "platform": "linux/xterm", "host": "xterm",
            "pass1_support": false, "structural_fail": null,
            "results": [ {"id": "ascii_0021", "overall": "pass"} ]
        }"#;
        let inputs = write_inputs("degraded", &[("l.json", raw.to_string())]);
        assert!(generate_from_inputs(&inputs).is_err());
    }

    #[test]
    fn structural_failure_fails_everything() {
        let raw = r#"{
            "platform": "macos/terminal-app", "host": "terminal-app",
            "pass1_support": true, "structural_fail": Some,
            "results": [ {"id": "ascii_0021", "overall": "pass"} ]
        }"#
        .replace("Some", "\"page 0: bands mismatch\"");
        let inputs = write_inputs("structfail", &[("m.json", raw)]);
        assert!(generate_from_inputs(&inputs).is_err());
    }

    #[test]
    fn universe_skew_is_detected() {
        let a = verdicts_json("linux/xterm", "xterm", &["ascii_0021"], &["ascii_0021"]);
        let b = verdicts_json(
            "windows/wt",
            "wt",
            &["ascii_0021"],
            &["ascii_0021", "box_2502"],
        );
        let inputs = write_inputs("skew", &[("a.json", a), ("b.json", b)]);
        assert!(generate_from_inputs(&inputs).is_err());
    }

    #[test]
    fn validate_host_accepts_conhost_headless_target() {
        assert!(validate_host("windows/conhost-run", "conhost").is_ok());
        assert!(validate_host("windows/wt", "wt").is_ok());
        assert!(validate_host("unknown/x", "whatever").is_ok()); // no policy
    }
}
