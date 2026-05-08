/// Parse a named color to RGB values.
/// Used by sidebar and detail panels for consistent color rendering.
pub fn parse_color(name: &str) -> Option<(u8, u8, u8)> {
    match name {
        "red" => Some((243, 139, 168)),
        "green" => Some((166, 227, 161)),
        "blue" => Some((137, 180, 250)),
        "yellow" => Some((249, 226, 175)),
        "purple" => Some((203, 166, 247)),
        "cyan" => Some((148, 226, 213)),
        "orange" => Some((250, 179, 135)),
        _ => None,
    }
}
