//! Pass-1 correlation: match DSR payloads to candidates by **payload row**,
//! never by arrival order.
//!
//! Correlation contract (plan §4.3): each candidate row fires exactly one
//! fire-and-forget `\x1b[6n` right after its frame is printed, so a valid
//! response for candidate `(page, page_row)` must report the same physical
//! viewport row (normalized via `saturating_sub(1)`) and a column equal to
//! `EXPECTED_END_COL_1BASED` when — and only when — the glyph advanced exactly
//! one cell.

use crate::dsr::{CursorReport, ReportLog};
use crate::render::{FOOTER_REPLY_COL_1BASED, FOOTER_ROW_1BASED};
use crate::sidecar::{Pass1Report, Pass1Result, Pass1Status, RowEntry};

/// Outcome of the end-of-run drain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DrainStats {
    /// Reports observed in total (including footer sentinels).
    pub reports: usize,
    /// Candidates that found a correlating payload.
    pub correlated: usize,
    /// Total candidates.
    pub candidates: usize,
}

impl DrainStats {
    /// Circuit-breaker predicate: fewer than half the rows answered ⇒ the
    /// transport is unusable and the run must fail fast.
    #[must_use]
    pub fn below_half(&self) -> bool {
        self.candidates > 0 && self.correlated * 2 < self.candidates
    }
}

/// Wait until the footer sentinel reply for `page_start_index` arrives or the
/// deadline expires.
///
/// `from` indexes into the log so only reports emitted during this page's
/// window are considered; nothing is ever discarded from the log.
pub fn wait_footer(
    log: &std::sync::Mutex<ReportLog>,
    from: usize,
    deadline: std::time::Instant,
) -> Result<(), crate::FailFast> {
    let want = CursorReport::new(u32::from(FOOTER_ROW_1BASED), FOOTER_REPLY_COL_1BASED);
    loop {
        {
            let log = log.lock().expect("reader thread panicked");
            if log.find_from(from, want).is_some() {
                return Ok(());
            }
        }
        if std::time::Instant::now() >= deadline {
            return Err(crate::FailFast::FooterHandshakeTimeout);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// Correlate every candidate with the accumulated report log.
///
/// Footer replies (`row 25`) are excluded from candidate data by construction:
/// they normalize to page_row 24 which no candidate occupies.
#[must_use]
pub fn correlate(log: &ReportLog, sidecar: &crate::sidecar::Sidecar) -> Pass1Report {
    let mut results = Vec::new();
    let mut correlated = 0usize;
    let mut total = 0usize;

    for entry in &sidecar.rows {
        let RowEntry::Candidate {
            id,
            codepoint,
            page,
            page_row,
            expected_end_col,
            ..
        } = entry
        else {
            continue;
        };
        total += 1;
        // First report whose normalized payload row matches this candidate's
        // page-relative row. Duplicate/stray same-row reports are ignored
        // beyond the first hit.
        let hit = log
            .reports()
            .iter()
            .find(|r| r.page_row0() == u32::from(*page_row));

        let status = match hit {
            None => Pass1Status::Inconclusive,
            Some(r) if r.col_1based == u32::from(*expected_end_col) => Pass1Status::Pass,
            // Column mismatch: the emulator expanded the glyph past one cell.
            Some(_) => Pass1Status::Fail,
        };
        if status == Pass1Status::Pass {
            correlated += 1;
        }
        results.push(Pass1Result {
            id: id.clone(),
            codepoint: *codepoint,
            page: *page,
            page_row: *page_row,
            expected_end_col: *expected_end_col,
            observed_col: hit.map(|r| u16::try_from(r.col_1based).unwrap_or(u16::MAX)),
            status,
        });
    }

    Pass1Report {
        pass1_support: true,
        pass1_method: crate::sidecar::PASS1_METHOD,
        results,
        stats: DrainStats {
            reports: log.reports().len(),
            correlated,
            candidates: total,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sidecar::{Colors, Sentinels, Sidecar};
    use icon_catalog::Block;

    fn sidecar_with(rows: Vec<RowEntry>) -> Sidecar {
        Sidecar {
            schema_version: 1,
            platform: "test".into(),
            host: "xterm".into(),
            pass1_support: true,
            pass1_method: "batched-dsr",
            sentinels: Sentinels {
                boundary: '|',
                left: 'A',
                right: 'B',
            },
            colors: Colors {
                fg: "#c0c0c0",
                bg: "#000000",
            },
            page_rows: 24,
            pages: 1,
            rows,
        }
    }

    fn cand(id: &str, page: u32, page_row: u16) -> RowEntry {
        RowEntry::Candidate {
            id: id.into(),
            codepoint: 0x2502,
            block: Block::BoxDrawing,
            fallback: "|",
            page,
            page_row,
            expected_end_col: 8,
        }
    }

    #[test]
    fn pass_when_payload_matches_expected_column() {
        let sc = sidecar_with(vec![cand("box_2502", 0, 3)]);
        let mut log = ReportLog::new();
        log.feed(b"\x1b[4;8R"); // physical row 4 → page_row 3, col 8 ✓
        let report = correlate(&log, &sc);
        assert_eq!(report.results[0].status, Pass1Status::Pass);
        assert_eq!(report.results[0].observed_col, Some(8));
    }

    #[test]
    fn fail_on_width_expansion() {
        let sc = sidecar_with(vec![cand("wide", 0, 3)]);
        let mut log = ReportLog::new();
        log.feed(b"\x1b[4;9R"); // one cell too far: 2-cell glyph
        assert_eq!(correlate(&log, &sc).results[0].status, Pass1Status::Fail);
    }

    #[test]
    fn inconclusive_when_response_missing() {
        let sc = sidecar_with(vec![cand("box_2502", 0, 3), cand("box_2500", 0, 4)]);
        let mut log = ReportLog::new();
        log.feed(b"\x1b[5;8R"); // answers row 4 only
        let report = correlate(&log, &sc);
        assert_eq!(report.results[0].status, Pass1Status::Inconclusive);
        assert_eq!(report.results[1].status, Pass1Status::Pass);
    }

    #[test]
    fn stray_da_report_cannot_desynchronize() {
        let sc = sidecar_with(vec![cand("box_2502", 0, 0)]);
        let mut log = ReportLog::new();
        log.feed(b"\x1b[?1;2c\x1b[1;8R\x1b[6;3R");
        assert_eq!(correlate(&log, &sc).results[0].status, Pass1Status::Pass);
    }

    #[test]
    fn dropped_middle_response_does_not_shift_correlation() {
        // Responses arrive out of order relative to emission; payload-row
        // matching is immune to drops of any single report.
        let sc = sidecar_with(vec![cand("a", 0, 1), cand("b", 0, 2), cand("c", 0, 3)]);
        let mut log = ReportLog::new();
        log.feed(b"\x1b[2;8R"); // a ✓
        log.feed(b"\x1b[4;8R"); // c ✓ (b's report dropped entirely)
        let report = correlate(&log, &sc);
        assert_eq!(report.results[0].status, Pass1Status::Pass);
        assert_eq!(report.results[1].status, Pass1Status::Inconclusive);
        assert_eq!(report.results[2].status, Pass1Status::Pass);
    }

    #[test]
    fn footer_reply_never_correlates_to_candidates() {
        let sc = sidecar_with(vec![cand("box_2502", 0, 23)]);
        let mut log = ReportLog::new();
        // Candidate never answered; only the footer reply exists.
        log.feed(b"\x1b[25;1R");
        assert_eq!(
            correlate(&log, &sc).results[0].status,
            Pass1Status::Inconclusive
        );
    }

    #[test]
    fn circuit_breaker_thresholds() {
        let stats = |correlated, candidates| DrainStats {
            reports: 0,
            correlated,
            candidates,
        };
        assert!(!stats(10, 20).below_half());
        assert!(stats(9, 20).below_half());
        assert!(!stats(0, 0).below_half());
    }
}
