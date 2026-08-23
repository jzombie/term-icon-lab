//! Sidecar and Pass-1 report schemas (all coordinates 0-based page-relative).

use icon_catalog::Block;
use serde::Serialize;

/// Schema version of the sidecar contract consumed by `pixel-assert`.
pub const SCHEMA_VERSION: u32 = 1;
/// Pass-1 transport identifier recorded in every sidecar.
pub const PASS1_METHOD: &str = "batched-dsr";

/// Sentinel configuration of the anchor frame.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Sentinels {
    pub boundary: char,
    pub left: char,
    pub right: char,
}

/// Explicit SGR colors pinned by the harness before rendering.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Colors {
    /// SGR 37 → light gray.
    pub fg: &'static str,
    /// SGR 40 → black.
    pub bg: &'static str,
}

/// One rendered line of the matrix.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RowEntry {
    /// Reference row `| A B |` used by Pass 2 to calibrate cell geometry and
    /// to harvest the clean sentinel-B crop for the bleed check.
    Control {
        page: u32,
        /// 0-based page-relative row.
        page_row: u16,
    },
    /// A candidate under test.
    Candidate {
        id: String,
        codepoint: u32,
        block: Block,
        fallback: &'static str,
        page: u32,
        /// 0-based page-relative row.
        page_row: u16,
        /// Cursor column (1-based) expected after printing the frame when —
        /// and only when — the glyph renders exactly one cell wide.
        expected_end_col: u16,
    },
}

/// Sidecar describing the full render: what was printed where, and whether
/// Pass 1 ran at all. Consumed together with the per-page screenshots by
/// `pixel-assert`.
#[derive(Clone, Debug, Serialize)]
pub struct Sidecar {
    pub schema_version: u32,
    /// Free-form platform label from `--platform` (e.g. `linux/xterm`).
    pub platform: String,
    /// Host emulator enum string (`wt | conhost | xterm | gnome-terminal |
    /// terminal-app`). Never emitted into generated code; used only for
    /// in-memory validation downstream.
    pub host: String,
    pub pass1_support: bool,
    pub pass1_method: &'static str,
    pub sentinels: Sentinels,
    pub colors: Colors,
    /// Rows per page including the control row.
    pub page_rows: u16,
    pub pages: u32,
    pub rows: Vec<RowEntry>,
}

/// Outcome of a single candidate's PTY-level assertion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Pass1Status {
    /// Cursor landed on the same row at `expected_end_col`: glyph advanced by
    /// exactly one cell.
    Pass,
    /// Cursor landed further right or wrapped: the emulator expanded the glyph
    /// beyond one cell.
    Fail,
    /// No correlatable response arrived for this row (dropped report, timeout).
    Inconclusive,
}

/// Per-candidate Pass-1 result.
#[derive(Clone, Debug, Serialize)]
pub struct Pass1Result {
    pub id: String,
    pub codepoint: u32,
    pub page: u32,
    pub page_row: u16,
    pub expected_end_col: u16,
    /// Column observed in the correlated DSR payload, if any.
    pub observed_col: Option<u16>,
    pub status: Pass1Status,
}

/// Full Pass-1 report (`pass1.json`).
#[derive(Clone, Debug, Serialize)]
pub struct Pass1Report {
    pub pass1_support: bool,
    pub pass1_method: &'static str,
    pub results: Vec<Pass1Result>,
    /// Aggregate drain statistics for diagnostics.
    pub stats: crate::pass1::DrainStats,
}
