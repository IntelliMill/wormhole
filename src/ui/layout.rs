use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Split the terminal area into title bar, sidebar, detail panel, and bottom hint bar.
pub fn main_layout(area: Rect) -> (Rect, Rect, Rect, Rect) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // title bar
            Constraint::Min(0),    // main content
            Constraint::Length(1),  // search bar
        ])
        .split(area);

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35), // sidebar
            Constraint::Percentage(65), // detail
        ])
        .split(outer[1]);

    (outer[0], content[0], content[1], outer[2])
}
