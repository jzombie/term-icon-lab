//! JSON mirrors of the harness sidecar and Pass-1 report.

use icon_catalog::Block;
use serde::Deserialize;

/// Sidecar written by `matrix-harness` (see plan §4).
///
/// Fields not consumed locally are retained so the struct mirrors the wire
/// schema one-to-one (serde skips unknown keys; we keep named ones explicit).
#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct Sidecar {
    pub schema_version: u32,
    pub platform: String,
    pub host: String,
    pub pass1_support: bool,
    #[allow(dead_code)]
    pub pass1_method: String,
    #[serde(default)]
    pub page_rows: u16,
    pub pages: u32,
    pub rows: Vec<RowEntry>,
}

impl Sidecar {
    /// Rows of one page ordered by `page_row`.
    #[must_use]
    pub fn rows_for_page(&self, page: u32) -> Vec<&RowEntry> {
        let mut rows: Vec<&RowEntry> = self.rows.iter().filter(|r| r.page() == page).collect();
        rows.sort_by_key(|r| r.page_row());
        rows
    }
}

/// One rendered line of the matrix.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RowEntry {
    Control {
        page: u32,
        page_row: u16,
    },
    Candidate {
        id: String,
        codepoint: u32,
        block: Block,
        page: u32,
        page_row: u16,
        #[allow(dead_code)]
        expected_end_col: u16,
    },
}

impl RowEntry {
    #[must_use]
    pub fn page(&self) -> u32 {
        match self {
            RowEntry::Control { page, .. } | RowEntry::Candidate { page, .. } => *page,
        }
    }

    #[must_use]
    pub fn page_row(&self) -> u16 {
        match self {
            RowEntry::Control { page_row, .. } | RowEntry::Candidate { page_row, .. } => *page_row,
        }
    }
}

/// Pass-1 report (`pass1.json`).
#[derive(Clone, Debug, Deserialize)]
pub struct Pass1Report {
    pub pass1_support: bool,
    pub results: Vec<Pass1Result>,
}

/// Per-candidate Pass-1 outcome.
#[derive(Clone, Debug, Deserialize)]
pub struct Pass1Result {
    pub id: String,
    #[serde(rename = "codepoint")]
    pub _codepoint: u32,
    pub status: Pass1Status,
}

/// Pass-1 status strings as serialized by the harness.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pass1Status {
    Pass,
    Fail,
    Inconclusive,
}
