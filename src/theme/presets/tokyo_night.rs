use ratatui::style::Color;
use crate::theme::Theme;

pub fn tokyo_night() -> Theme {
    Theme {
        bg: Color::Rgb(26, 27, 38),
        fg: Color::Rgb(192, 202, 245),
        accent: Color::Rgb(122, 162, 247),
        border: Color::Rgb(69, 71, 90),
        error: Color::Rgb(247, 118, 142),
        muted: Color::Rgb(88, 91, 112),
        highlight: Color::Rgb(69, 71, 90),
        group_active: Color::Rgb(187, 154, 247),
        host_normal: Color::Rgb(169, 177, 214),
        detail_key: Color::Rgb(187, 154, 247),
        detail_value: Color::Rgb(192, 202, 245),
    }
}
