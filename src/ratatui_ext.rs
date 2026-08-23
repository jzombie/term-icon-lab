//! Ratatui buffer integration for [`SafeIcon`].
//!
//! Enabled by the `ratatui` feature (the crate's only feature flag). All
//! writes are single-cell and bounds-checked: clipping against the buffer edge
//! is a no-op, never a panic.

use crate::SafeIcon;
use ratatui::buffer::Buffer;
use ratatui::layout::Position;
use ratatui::style::Style;

/// Write verified 1x1 icons into a ratatui [`Buffer`].
pub trait SetSafeIcon {
    /// Write `icon` at `(x, y)` with `style` and return the x coordinate of
    /// the next free column.
    ///
    /// The cursor always advances by exactly [`SafeIcon::CELL_WIDTH`] (== 1)
    /// on success. If the target position lies outside the buffer area the
    /// call is a no-op and `x` is returned unchanged — rendering near a
    /// clipped window edge never panics.
    fn set_safe_icon(&mut self, x: u16, y: u16, icon: &SafeIcon, style: Style) -> u16;
}

impl SetSafeIcon for Buffer {
    fn set_safe_icon(&mut self, x: u16, y: u16, icon: &SafeIcon, style: Style) -> u16 {
        if x < self.area.left()
            || x >= self.area.right()
            || y < self.area.top()
            || y >= self.area.bottom()
        {
            return x;
        }
        match self.cell_mut(Position::new(x, y)) {
            Some(cell) => {
                cell.set_symbol(icon.glyph).set_style(style);
                x + SafeIcon::CELL_WIDTH
            }
            None => x,
        }
    }
}

#[cfg(all(test, feature = "ratatui"))]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    fn buf(w: u16, h: u16) -> Buffer {
        Buffer::empty(Rect::new(0, 0, w, h))
    }

    #[test]
    fn writes_glyph_and_advances_one() {
        let mut b = buf(4, 1);
        let next = b.set_safe_icon(0, 0, &SafeIcon::new("│", "|"), Style::default());
        assert_eq!(next, 1);
        assert_eq!(b[(0, 0)].symbol(), "│");
        assert_eq!(b[(1, 0)].symbol(), " ");
    }

    #[test]
    fn clips_right_edge_without_panic() {
        let mut b = buf(2, 1);
        let next = b.set_safe_icon(2, 0, &SafeIcon::new("█", "#"), Style::default());
        assert_eq!(next, 2); // x == area.right(): no mutation, no advance
        // Reading back must also avoid panicking: Option-based access.
        assert!(b.cell(Position::new(2, 0)).is_none());
    }

    #[test]
    fn clips_bottom_edge_without_panic() {
        let mut b = buf(2, 1);
        let next = b.set_safe_icon(0, 1, &SafeIcon::new("⠁", "*"), Style::default());
        assert_eq!(next, 0);
    }

    #[test]
    fn applies_style() {
        use ratatui::style::{Color, Modifier};
        let mut b = buf(2, 1);
        b.set_safe_icon(
            0,
            0,
            &SafeIcon::new("─", "-"),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        );
        let cell = &b[(0, 0)];
        assert_eq!(cell.symbol(), "─");
        assert_eq!(cell.fg, Color::Red);
    }
}
