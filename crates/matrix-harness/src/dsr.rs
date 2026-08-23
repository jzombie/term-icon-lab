//! Byte-level DSR (Device Status Report) parsing.
//!
//! Terminal emulators answer `\x1b[6n` with raw cursor-position reports
//! (`\x1b[{row};{col}R`) that carry **no newline terminator** and may be
//! interleaved with unrelated reports (e.g. primary DA replies
//! `\x1b[?1;2c`). Line-oriented stdin primitives would deadlock or corrupt
//! the stream, so all parsing operates on a raw byte buffer through a minimal
//! CSI state machine.
//!
//! Coordinates inside a [`CursorReport`] are the terminal's 1-based payload
//! values; normalization to 0-based page-relative rows happens once, in
//! [`CursorReport::page_row0`].

/// A parsed cursor position report.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorReport {
    /// 1-based physical viewport row, exactly as reported by the emulator.
    pub row_1based: u32,
    /// 1-based column, exactly as reported by the emulator.
    pub col_1based: u32,
}

impl CursorReport {
    /// Normalize the 1-based payload row to a 0-based page-relative row.
    ///
    /// Single conversion point for the entire pipeline: everything internal is
    /// 0-based, everything on the wire (DSR payloads, CUP parameters) is
    /// 1-based.
    #[must_use]
    pub fn page_row0(&self) -> u32 {
        self.row_1based.saturating_sub(1)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Ground,
    Esc,
    CsiPrivate,
    CsiParams,
    CsiIntermediate,
}

/// Incremental parser over the raw stdin byte stream.
///
/// The parser never discards bytes it has not classified: unknown sequences
/// are consumed atomically (CSI semantics) and dropped only when they cannot
/// be a cursor report, which makes stray reports (DA replies, mode reports)
/// inert to correlation.
#[derive(Default)]
pub struct DsrParser {
    state: State,
    params: Vec<u8>,
}

impl DsrParser {
    /// Feed raw bytes, returning every completed cursor report.
    pub fn feed(&mut self, bytes: &[u8]) -> Vec<CursorReport> {
        let mut out = Vec::new();
        for &b in bytes {
            match self.state {
                State::Ground => {
                    if b == 0x1B {
                        self.state = State::Esc;
                    }
                }
                State::Esc => match b {
                    b'[' => {
                        self.state = State::CsiParams;
                        self.params.clear();
                    }
                    0x1B => {}
                    _ => self.state = State::Ground,
                },
                State::CsiParams => {
                    if b.is_ascii_digit() || b == b';' {
                        self.params.push(b);
                    } else if (0x3C..=0x3F).contains(&b) {
                        // Private parameter marker (`?`, `<`, `=`, `>`):
                        // sequences like DA replies can never be CPRs.
                        self.state = State::CsiPrivate;
                    } else if (0x20..=0x2F).contains(&b) {
                        self.state = State::CsiIntermediate;
                    } else if (0x40..=0x7E).contains(&b) {
                        if b == b'R'
                            && let Some(report) = parse_params(&self.params)
                        {
                            out.push(report);
                        }
                        self.state = State::Ground;
                    } else {
                        // Control byte inside CSI (e.g. ESC): restart.
                        self.state = if b == 0x1B { State::Esc } else { State::Ground };
                    }
                }
                State::CsiPrivate => {
                    if (0x40..=0x7E).contains(&b) {
                        self.state = State::Ground;
                    }
                }
                State::CsiIntermediate => {
                    if (0x40..=0x7E).contains(&b) {
                        self.state = State::Ground;
                    }
                }
            }
        }
        out
    }
}

fn parse_params(params: &[u8]) -> Option<CursorReport> {
    let text = std::str::from_utf8(params).ok()?;
    let (row, col) = text.split_once(';')?;
    let row: u32 = row.parse().ok()?;
    let col: u32 = col.parse().ok()?;
    Some(CursorReport {
        row_1based: row,
        col_1based: col,
    })
}

/// Shared inbox between the stdin reader thread and the control logic.
///
/// Bytes flow in continuously from t=0 (outside the frame render loop);
/// consumers inspect the accumulated report list without ever blocking the
/// render path.
#[derive(Default)]
pub struct ReportLog {
    parser: DsrParser,
    reports: Vec<CursorReport>,
}

impl ReportLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed freshly read bytes into the log.
    pub fn feed(&mut self, bytes: &[u8]) {
        self.reports.extend(self.parser.feed(bytes));
    }

    /// All decoded cursor reports so far, in arrival order.
    pub fn reports(&self) -> &[CursorReport] {
        &self.reports
    }

    /// Search for a specific report among entries at index >= `from`.
    pub fn find_from(&self, from: usize, want: CursorReport) -> Option<usize> {
        self.reports[from..]
            .iter()
            .position(|r| *r == want)
            .map(|i| from + i)
    }
}

impl CursorReport {
    /// Convenience constructor for tests and sentinels.
    pub const fn new(row_1based: u32, col_1based: u32) -> Self {
        Self {
            row_1based,
            col_1based,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed_all(bytes: &[u8]) -> Vec<CursorReport> {
        let mut p = DsrParser::default();
        p.feed(bytes)
    }

    #[test]
    fn parses_simple_report() {
        assert_eq!(
            feed_all(b"\x1b[10;5R"),
            vec![CursorReport {
                row_1based: 10,
                col_1based: 5
            }]
        );
    }

    #[test]
    fn parses_concatenated_reports() {
        let out = feed_all(b"\x1b[1;8R\x1b[2;8R\x1b[24;9R");
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].row_1based, 1);
        assert_eq!(out[2].col_1based, 9);
    }

    #[test]
    fn handles_split_feeds_byte_by_byte() {
        let mut p = DsrParser::default();
        let mut all = Vec::new();
        for b in b"\x1b[7;3R" {
            all.extend(p.feed(std::slice::from_ref(b)));
        }
        assert_eq!(
            all,
            vec![CursorReport {
                row_1based: 7,
                col_1based: 3
            }]
        );
    }

    #[test]
    fn ignores_stray_da_reply() {
        // Primary DA reply carries a private marker and final `c`.
        assert!(feed_all(b"\x1b[?1;2c").is_empty());
    }

    #[test]
    fn ignores_mode_and_other_csi_sequences() {
        assert!(feed_all(b"\x1b[?1049h\x1b[2J\x1b[H\x1b[0m").is_empty());
    }

    #[test]
    fn garbage_between_reports_is_inert() {
        let out = feed_all(b"hello \x1b world \x1b[12;4R trailing");
        assert_eq!(out.len(), 1);
        assert_eq!(
            out[0],
            CursorReport {
                row_1based: 12,
                col_1based: 4
            }
        );
    }

    #[test]
    fn footer_sentinel_distinguished_from_row_report() {
        // Row-24 candidate reply followed by the dedicated footer sentinel.
        let mut log = ReportLog::new();
        log.feed(b"\x1b[24;8R");
        assert!(log.find_from(0, CursorReport::new(25, 1)).is_none());
        log.feed(b"\x1b[25;1R");
        assert!(log.find_from(0, CursorReport::new(25, 1)).is_some());
        // Both responses preserved — nothing was discarded.
        assert_eq!(log.reports().len(), 2);
        assert_eq!(log.reports()[0].page_row0(), 23);
    }

    #[test]
    fn normalization_is_single_point() {
        assert_eq!(CursorReport::new(1, 8).page_row0(), 0);
        assert_eq!(CursorReport::new(25, 1).page_row0(), 24);
    }
}
