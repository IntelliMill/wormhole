// Nerd Font v3 icon constants.
// Requires a Nerd Font (v3+) to display correctly.

pub const SEARCH: &str = "\u{F0349}"; // nf-md-magnify
pub const ERROR: &str = "\u{F0028}";  // nf-md-alert_circle

// Unicode regional indicator flags (not Nerd Font — standard emoji).
// Each flag is two Regional Indicator Symbol letters: U+1F1E6..U+1F1FF.
pub const FLAG_ZH: &str = "\u{1F1E8}\u{1F1F3}"; // 🇨🇳
pub const FLAG_EN: &str = "\u{1F1EC}\u{1F1E7}"; // 🇬🇧

/// Return the flag emoji for a supported language.
pub fn lang_flag(lang: crate::i18n::Lang) -> &'static str {
    match lang {
        crate::i18n::Lang::Zh => FLAG_ZH,
        crate::i18n::Lang::En => FLAG_EN,
    }
}
