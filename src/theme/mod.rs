use ratatui::style::Color;

/// Color palette used across all UI widgets.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub border: Color,
    pub error: Color,
    pub muted: Color,
    pub highlight: Color,
    pub group_active: Color,
    pub host_normal: Color,
    pub detail_key: Color,
    pub detail_value: Color,
}

pub mod presets;

/// Look up a built-in theme by name, falling back to Catppuccin Mocha.
pub fn get_theme(name: &str) -> Theme {
    match name {
        "catppuccin_mocha" => presets::catppuccin_mocha(),
        "tokyo_night" => presets::tokyo_night(),
        "dracula" => presets::dracula(),
        "gruvbox_dark" => presets::gruvbox_dark(),
        _ => presets::catppuccin_mocha(),
    }
}

/// Return the list of all available theme names.
pub fn theme_names() -> Vec<&'static str> {
    vec![
        "catppuccin_mocha",
        "tokyo_night",
        "dracula",
        "gruvbox_dark",
    ]
}
