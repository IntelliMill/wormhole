use ratatui::style::Color;
use crate::theme::Theme;

pub fn dracula() -> Theme {
    Theme {
        bg: Color::Rgb(40, 42, 54),
        fg: Color::Rgb(248, 248, 242),
        accent: Color::Rgb(189, 147, 249),
        border: Color::Rgb(68, 71, 90),
        error: Color::Rgb(255, 85, 85),
        muted: Color::Rgb(98, 114, 164),
        highlight: Color::Rgb(98, 114, 164),
        group_active: Color::Rgb(255, 121, 198),
        host_normal: Color::Rgb(220, 223, 228),
        detail_key: Color::Rgb(255, 121, 198),
        detail_value: Color::Rgb(248, 248, 242),
    }
}
