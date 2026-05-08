use ratatui::style::Color;
use crate::theme::Theme;

pub fn catppuccin_mocha() -> Theme {
    Theme {
        bg: Color::Rgb(30, 30, 46),
        fg: Color::Rgb(205, 214, 244),
        accent: Color::Rgb(137, 180, 250),
        border: Color::Rgb(88, 91, 112),
        error: Color::Rgb(243, 139, 168),
        muted: Color::Rgb(108, 112, 134),
        highlight: Color::Rgb(88, 91, 112),
        group_active: Color::Rgb(203, 166, 247),
        host_normal: Color::Rgb(186, 194, 222),
        detail_key: Color::Rgb(180, 190, 254),
        detail_value: Color::Rgb(205, 214, 244),
    }
}
