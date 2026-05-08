use ratatui::style::Color;
use crate::theme::Theme;

pub fn gruvbox_dark() -> Theme {
    Theme {
        bg: Color::Rgb(40, 40, 40),
        fg: Color::Rgb(235, 219, 178),
        accent: Color::Rgb(214, 188, 90),
        border: Color::Rgb(80, 73, 69),
        error: Color::Rgb(204, 36, 29),
        muted: Color::Rgb(146, 131, 116),
        highlight: Color::Rgb(80, 73, 69),
        group_active: Color::Rgb(214, 188, 90),
        host_normal: Color::Rgb(213, 196, 161),
        detail_key: Color::Rgb(214, 188, 90),
        detail_value: Color::Rgb(235, 219, 178),
    }
}
