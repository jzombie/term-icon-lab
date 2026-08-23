//! Pure page-render byte builders.
//!
//! Everything the harness writes to the terminal is produced here as bytes so
//! it can be unit-tested without a TTY: CUP parameter conversion, page clears,
//! sentinel frames, and probe placement all live in this module.
//!
//! Coordinate contract (see plan §4):
//! * internal rows are 0-based and **page-relative** (`0..PAGE_ROWS-1`);
//! * ANSI CUP is 1-based and treats parameter `0` as `1`, so emission adds 1
//!   exactly once — `cup(page_row) == \x1b[page_row+1;1H`;
//! * physical row [`FOOTER_ROW_1BASED`] (25) is reserved for the page-end
//!   handshake probe and never carries candidate ink.

/// Candidate rows per page (physical rows 1..=24).
pub const PAGE_ROWS: u16 = 24;
/// Physical 1-based row reserved for the footer handshake probe.
pub const FOOTER_ROW_1BASED: u16 = 25;
/// Sentinel characters of the anchor frame.
pub const SENTINEL_LEFT: char = 'A';
pub const SENTINEL_RIGHT: char = 'B';
pub const BOUNDARY: char = '|';

// Frame layout (0-based columns inside the printed line):
// col 0 = '|', col 2 = 'A', col 3 = icon, col 4 = 'B', col 6 = '|'.
// Consumed by tests and by downstream schema documentation.
#[allow(dead_code)]
pub const COL_A0: u16 = 2;
#[allow(dead_code)]
pub const COL_ICON0: u16 = 3;
#[allow(dead_code)]
pub const COL_B0: u16 = 4;
/// Cursor column (1-based) after printing a 7-char frame with a 1-cell icon.
pub const EXPECTED_END_COL_1BASED: u16 = 8;

/// Enter alternate screen, hide the cursor, pin explicit SGR colors
/// (light-gray on black) so pixel analysis sees known fg/bg values.
#[must_use]
pub fn begin_stream() -> Vec<u8> {
    b"\x1b[?1049h\x1b[?25l\x1b[0;37;40m".to_vec()
}

/// Restore SGR/cursor state and leave the alternate screen.
#[must_use]
pub fn end_stream() -> Vec<u8> {
    b"\x1b[0m\x1b[?25h\x1b[?1049l".to_vec()
}

/// Clear the screen and home the cursor. Emitted at the start of every page so
/// residue from page N can never contaminate page N+1's capture.
#[must_use]
pub fn clear_page() -> Vec<u8> {
    b"\x1b[2J\x1b[H".to_vec()
}

/// CUP to the given 0-based page-relative row, column 1.
///
/// ECMA-48 parameters are 1-based and `0` aliases to `1`; converting here is
/// the single +1 boundary so rows 0 and 1 land on distinct physical lines.
#[must_use]
pub fn cup(page_row0: u16) -> String {
    format!("\x1b[{};1H", u32::from(page_row0) + 1)
}

/// The control/reference row `| A B |` (icon slot holds a space).
#[must_use]
pub fn control_row(page_row0: u16) -> String {
    format!(
        "{}{BOUNDARY} {SENTINEL_LEFT} {SENTINEL_RIGHT} {BOUNDARY}",
        cup(page_row0)
    )
}

/// A candidate row `| A<glyph>B |` followed by its fire-and-forget CPR query.
#[must_use]
pub fn candidate_row(page_row0: u16, glyph: char) -> String {
    format!(
        "{}{BOUNDARY} {SENTINEL_LEFT}{glyph}{SENTINEL_RIGHT} {BOUNDARY}\x1b[6n",
        cup(page_row0)
    )
}

/// Move to the reserved footer row and fire the handshake probe. Its reply is
/// expected to be exactly `\x1b[{FOOTER_ROW_1BASED};1R`.
#[must_use]
pub fn footer_probe() -> String {
    format!("\x1b[{FOOTER_ROW_1BASED};1H\x1b[6n")
}

/// Expected reply payload of the footer handshake (column component; the row
/// component is [`FOOTER_ROW_1BASED`]).
pub const FOOTER_REPLY_COL_1BASED: u32 = 1;

/// Maximum candidates that fit on one page (control row occupies row 0).
pub const MAX_CANDIDATES_PER_PAGE: usize = PAGE_ROWS as usize - 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cup_rows_zero_and_one_are_distinct() {
        // ECMA-48 aliases parameter 0 to 1; our conversion must not emit 0.
        assert_eq!(cup(0), "\x1b[1;1H");
        assert_eq!(cup(1), "\x1b[2;1H");
        assert_ne!(cup(0), cup(1));
        for r in 0..PAGE_ROWS - 1 {
            assert_ne!(cup(r), cup(r + 1));
        }
    }

    #[test]
    fn pages_start_with_clear() {
        let mut stream = begin_stream();
        stream.extend(clear_page());
        let s = String::from_utf8(stream).unwrap();
        assert!(s.starts_with("\x1b[?1049h"));
        assert!(s.contains("\x1b[2J\x1b[H"));
    }

    #[test]
    fn control_row_layout_matches_contract() {
        let row = control_row(0);
        assert!(row.starts_with("\x1b[1;1H"));
        let text: String = row["\x1b[1;1H".len()..].to_string();
        let cols: Vec<char> = text.chars().collect();
        assert_eq!(cols[0], '|');
        assert_eq!(cols[usize::from(COL_A0)], 'A');
        assert_eq!(cols[usize::from(COL_ICON0)], ' '); // blank reference slot
        assert_eq!(cols[usize::from(COL_B0)], 'B');
        assert_eq!(cols[6], '|');
        assert_eq!(text.chars().count(), 7);
    }

    #[test]
    fn candidate_row_embeds_glyph_and_query() {
        let row = candidate_row(5, '\u{2502}');
        assert!(row.starts_with("\x1b[6;1H"));
        assert!(row.contains("| A\u{2502}B |"));
        assert!(row.ends_with("\x1b[6n"));
    }

    #[test]
    fn footer_probe_targets_reserved_row() {
        let p = footer_probe();
        assert_eq!(p, "\x1b[25;1H\x1b[6n");
    }

    #[test]
    fn end_stream_restores_terminal() {
        let s = String::from_utf8(end_stream()).unwrap();
        assert!(s.contains("\x1b[0m"));
        assert!(s.contains("\x1b[?25h"));
        assert!(s.contains("\x1b[?1049l"));
    }
}
