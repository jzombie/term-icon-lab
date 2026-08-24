//! matrix-harness: renders the candidate matrix inside a real terminal and
//! runs Pass 1 (PTY/CPR) assertions.
//!
//! Exit codes: `0` success, `2` fail-fast (handshake timeout, footer
//! handshake timeout, ack timeout, circuit breaker, undersized terminal),
//! `1` internal error. Clap usage errors also exit 2.

mod dsr;
mod pass1;
mod render;
mod sidecar;
mod sync;

use std::fmt;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::Context;
use clap::Parser;
use icon_catalog::candidates;

use crate::dsr::ReportLog;
use crate::sidecar::{Colors, Pass1Report, RowEntry, Sentinels, Sidecar};
use crate::sync::SyncChannel;

/// Minimum usable viewport enforced before rendering begins.
const MIN_COLS: u16 = 16;
const MIN_ROWS: u16 = 30;
/// Deadline for the footer handshake on every page. Generous on purpose: a
/// loaded CI runner can pause the emulator between page consumption and CPR
/// reply, and a mid-run timeout discards the whole verification run.
const FOOTER_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
/// End-of-run drain deadline for straggler DSR responses.
const DRAIN_TIMEOUT: Duration = Duration::from_secs(5);
/// Quiescence window that ends the drain early.
const DRAIN_QUIESCENCE: Duration = Duration::from_millis(300);

/// Fail-fast conditions; every variant terminates the process with code 2.
#[derive(Debug)]
pub enum FailFast {
    HandshakeTimeout,
    FooterHandshakeTimeout,
    Ack(sync::SyncError),
    CircuitBreaker,
    TerminalTooSmall { rows: u16, cols: u16 },
}

impl fmt::Display for FailFast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HandshakeTimeout => write!(f, "startup CPR handshake timed out"),
            Self::FooterHandshakeTimeout => write!(f, "page-end footer handshake timed out"),
            Self::Ack(e) => write!(f, "orchestrator sync failed: {e}"),
            Self::CircuitBreaker => write!(
                f,
                "circuit breaker: fewer than half of candidate rows received DSR replies"
            ),
            Self::TerminalTooSmall { rows, cols } => {
                write!(
                    f,
                    "terminal too small: {cols}x{rows}, need >= {MIN_COLS}x{MIN_ROWS}"
                )
            }
        }
    }
}

impl std::error::Error for FailFast {}

#[derive(Parser, Debug)]
#[command(
    name = "matrix-harness",
    about = "Render the term-icon candidate matrix and run Pass 1 (PTY/CPR) assertions."
)]
struct Cli {
    /// Orchestrator-created directory for .ready/.ack marker files.
    #[arg(long, required_unless_present = "self_check")]
    sync_dir: Option<PathBuf>,

    /// Directory for sidecar.json / pass1.json artifacts.
    #[arg(long, default_value = ".")]
    out_dir: PathBuf,

    /// Platform label recorded in the sidecar (e.g. linux/xterm).
    #[arg(long, default_value = "unknown")]
    platform: String,

    /// Host emulator enum string (wt | conhost | xterm | gnome-terminal | terminal-app).
    #[arg(long, default_value = "unknown")]
    host: String,

    /// Candidates per page (the control row occupies row 0; max 23).
    #[arg(long, default_value_t = render::MAX_CANDIDATES_PER_PAGE)]
    page_candidates: usize,

    /// Skip all terminal interaction; emit the sidecar only.
    #[arg(long)]
    self_check: bool,

    /// Startup handshake deadline in milliseconds.
    #[arg(long, default_value_t = 2000)]
    handshake_timeout_ms: u64,
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("matrix-harness: {err:#}");
            std::process::ExitCode::from(2)
        }
    }
}

fn paginate(
    catalog: &[icon_catalog::Candidate],
    per_page: usize,
) -> anyhow::Result<Vec<&[icon_catalog::Candidate]>> {
    if per_page == 0 || per_page > render::MAX_CANDIDATES_PER_PAGE {
        anyhow::bail!(
            "--page-candidates must be between 1 and {}",
            render::MAX_CANDIDATES_PER_PAGE
        );
    }
    Ok(catalog.chunks(per_page).collect())
}

fn build_rows(pages: &[&[icon_catalog::Candidate]], platform: &str, host: &str) -> Sidecar {
    let mut rows = Vec::new();
    for (page, chunk) in pages.iter().enumerate() {
        rows.push(RowEntry::Control {
            page: page as u32,
            page_row: 0,
        });
        for (ri, c) in chunk.iter().enumerate() {
            rows.push(RowEntry::Candidate {
                id: c.id.clone(),
                codepoint: c.codepoint,
                block: c.block,
                fallback: c.fallback,
                page: page as u32,
                page_row: (ri + 1) as u16,
                expected_end_col: render::EXPECTED_END_COL_1BASED,
            });
        }
    }
    Sidecar {
        schema_version: sidecar::SCHEMA_VERSION,
        platform: platform.to_owned(),
        host: host.to_owned(),
        pass1_support: true,
        pass1_method: sidecar::PASS1_METHOD,
        sentinels: Sentinels {
            boundary: '|',
            left: 'A',
            right: 'B',
        },
        colors: Colors {
            fg: "#c0c0c0",
            bg: "#000000",
        },
        page_rows: render::PAGE_ROWS,
        pages: pages.len() as u32,
        rows,
    }
}

fn run(cli: &Cli) -> Result<(), anyhow::Error> {
    let catalog = candidates();
    let pages = paginate(catalog, cli.page_candidates)?;
    let sidecar = build_rows(&pages, &cli.platform, &cli.host);

    if cli.self_check {
        return self_check(cli, &sidecar);
    }

    let sync_dir = cli.sync_dir.clone().context("--sync-dir is required")?;
    let sync = SyncChannel::new(sync_dir);
    if !sync.dir().is_dir() {
        anyhow::bail!("--sync-dir {} does not exist", sync.dir().display());
    }

    // One shared report log between the reader thread and control logic.
    let log: Arc<Mutex<ReportLog>> = Arc::new(Mutex::new(ReportLog::new()));
    spawn_reader(log.clone());

    let mut out = std::io::stdout();
    crossterm::terminal::enable_raw_mode().context("enable raw mode")?;

    let session = drive_matrix(
        &mut out,
        &log,
        &sync,
        &sidecar,
        &pages,
        cli.handshake_timeout_ms,
    );

    // Best-effort restore regardless of outcome.
    let _ = out.write_all(&render::end_stream());
    let _ = out.flush();
    let _ = crossterm::terminal::disable_raw_mode();

    match session {
        Ok(report) => {
            write_artifacts(cli, &sidecar, Some(&report))?;
            println!(
                "pass1: {}/{} correlated, {} reports",
                report.stats.correlated, report.stats.candidates, report.stats.reports
            );
            Ok(())
        }
        Err(fast) => {
            eprintln!("FAIL-FAST: {fast}");
            write_diagnostic_sidecar(cli, &sidecar)?;
            Err(anyhow::anyhow!("{fast}"))
        }
    }
}

/// The full terminal session: startup handshake → paged render + handshakes →
/// final drain → correlation. Returns the Pass-1 report on success.
fn drive_matrix(
    out: &mut impl Write,
    log: &Arc<Mutex<ReportLog>>,
    sync: &SyncChannel,
    sidecar: &Sidecar,
    pages: &[&[icon_catalog::Candidate]],
    handshake_timeout_ms: u64,
) -> Result<Pass1Report, FailFast> {
    // Size guard before anything is drawn.
    let (cols, rows) =
        crossterm::terminal::size().map_err(|_| FailFast::TerminalTooSmall { rows: 0, cols: 0 })?;
    if cols < MIN_COLS || rows < MIN_ROWS {
        return Err(FailFast::TerminalTooSmall { rows, cols });
    }

    out.write_all(&render::begin_stream()).map_err(io_fail)?;

    // Startup handshake: single blocking probe with a hard deadline.
    let start_len = current_len(log);
    out.write_all(b"\x1b[1;1H\x1b[6n").map_err(io_fail)?;
    out.flush().map_err(io_fail)?;
    wait_for_growth(
        log,
        start_len,
        Instant::now() + Duration::from_millis(handshake_timeout_ms),
    )
    .ok_or(FailFast::HandshakeTimeout)?;

    // Paged render with footer handshakes and file-sync capture points.
    let mut windows: Vec<pass1::PageWindow> = Vec::with_capacity(pages.len());
    for (page, chunk) in pages.iter().enumerate() {
        let page = page as u32;
        let mut bytes = render::clear_page();
        bytes.extend(render::control_row(0).into_bytes());
        for (ri, c) in chunk.iter().enumerate() {
            bytes.extend(render::candidate_row((ri + 1) as u16, c.glyph()).into_bytes());
        }

        // Snapshot the log length BEFORE emitting this page's queries so both
        // the footer handshake and Pass-1 correlation are scoped to exactly
        // this page's window.
        let from = current_len(log);
        bytes.extend(render::footer_probe().into_bytes());
        windows.push(pass1::PageWindow { page, start: from });

        out.write_all(&bytes).map_err(io_fail)?;
        out.flush().map_err(io_fail)?;

        // Block until the emulator proves it consumed the whole page.
        pass1::wait_footer(log, from, Instant::now() + FOOTER_HANDSHAKE_TIMEOUT)
            .map_err(|_| FailFast::FooterHandshakeTimeout)?;

        // Frame is consumed by the emulator: release it to the orchestrator.
        sync.write_ready(page).map_err(FailFast::Ack)?;
        sync.wait_ack(page, sync::ACK_TIMEOUT)
            .map_err(FailFast::Ack)?;
        sync.cleanup(page);
    }

    // Final drain: collect stragglers until quiescent or deadline.
    drain(log);

    let report = pass1::correlate(
        &log.lock().expect("reader thread panicked"),
        sidecar,
        &windows,
    );
    if report.stats.below_half() {
        return Err(FailFast::CircuitBreaker);
    }
    Ok(report)
}

fn io_fail(e: std::io::Error) -> FailFast {
    // stdout failures under a live PTY are fatal; reuse the ack wrapper path.
    FailFast::Ack(sync::SyncError::Io {
        path: PathBuf::from("<stdout>"),
        source: e,
    })
}

fn current_len(log: &Mutex<ReportLog>) -> usize {
    log.lock().expect("reader thread panicked").reports().len()
}

fn wait_for_growth(log: &Mutex<ReportLog>, baseline: usize, deadline: Instant) -> Option<usize> {
    loop {
        let len = current_len(log);
        if len > baseline {
            return Some(len);
        }
        if Instant::now() >= deadline {
            return None;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn drain(log: &Arc<Mutex<ReportLog>>) {
    let deadline = Instant::now() + DRAIN_TIMEOUT;
    loop {
        let before = current_len(log);
        std::thread::sleep(DRAIN_QUIESCENCE);
        let after = current_len(log);
        if after == before || Instant::now() >= deadline {
            return;
        }
    }
}

/// Reader thread: raw-byte stdin pump feeding the shared [`ReportLog`].
///
/// No line-oriented primitives exist here by contract — DSR payloads carry no
/// newline terminator.
fn spawn_reader(log: Arc<Mutex<ReportLog>>) {
    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();
        let mut buf = [0u8; 4096];
        loop {
            match stdin.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if let Ok(mut l) = log.lock() {
                        l.feed(&buf[..n]);
                    }
                }
            }
        }
    });
}

fn write_artifacts(
    cli: &Cli,
    sidecar: &Sidecar,
    report: Option<&sidecar::Pass1Report>,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(&cli.out_dir)?;
    let sc_path = cli.out_dir.join("sidecar.json");
    std::fs::write(&sc_path, serde_json::to_vec_pretty(sidecar)?)
        .with_context(|| format!("write {}", sc_path.display()))?;
    if let Some(report) = report {
        let p1_path = cli.out_dir.join("pass1.json");
        std::fs::write(&p1_path, serde_json::to_vec_pretty(report)?)
            .with_context(|| format!("write {}", p1_path.display()))?;
    }
    Ok(())
}

fn write_diagnostic_sidecar(cli: &Cli, sidecar: &Sidecar) -> anyhow::Result<()> {
    let mut diag = sidecar.clone();
    diag.pass1_support = false;
    write_artifacts(cli, &diag, None)
}

fn self_check(cli: &Cli, sidecar: &Sidecar) -> Result<(), anyhow::Error> {
    write_artifacts(cli, sidecar, None)?;
    println!(
        "self-check: {} candidates across {} pages (no terminal interaction)",
        sidecar
            .rows
            .iter()
            .filter(|r| matches!(r, RowEntry::Candidate { .. }))
            .count(),
        sidecar.pages
    );
    Ok(())
}
