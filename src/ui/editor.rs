use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::config::model::{AuthType, Group, Host};
use crate::i18n;
use crate::theme::Theme;

// ── field types ──────────────────────────────────────────

/// Input type for an editor field.
#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Password,
    Select { options: Vec<(String, String)> },
    ColorSelect { options: Vec<(&'static str, (u8, u8, u8))> },
}

/// A single editable field in the host editor form.
#[derive(Debug, Clone)]
pub struct EditorField {
    pub key: FieldKey,
    pub label: String,
    pub value: String,
    pub field_type: FieldType,
}

/// Semantic identifier for each editable host attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldKey {
    DisplayName,
    Address,
    Port,
    Username,
    AuthType,
    Password,
    KeyPath,
    Group,
    Color,
    Notes,
}

// ── predefined options ──────────────────────────────────

/// Auth type choices for the editor select field.
pub fn auth_options() -> Vec<(String, String)> {
    vec![
        ("key".into(), i18n::tr("editor.auth.key")),
        ("password".into(), i18n::tr("editor.auth.password")),
        ("interactive".into(), i18n::tr("editor.auth.interactive")),
    ]
}

/// Color swatch choices for the editor color selector.
pub fn color_options() -> Vec<(&'static str, (u8, u8, u8))> {
    vec![
        ("none", (128, 128, 128)),
        ("red", (243, 139, 168)),
        ("green", (166, 227, 161)),
        ("blue", (137, 180, 250)),
        ("yellow", (249, 226, 175)),
        ("purple", (203, 166, 247)),
        ("cyan", (148, 226, 213)),
        ("orange", (250, 179, 135)),
    ]
}

/// Build group selector options: first entry is "no group", then each existing group.
/// Key = group id or "none", label = display name.
pub fn group_options(groups: &[Group]) -> Vec<(String, String)> {
    let mut opts = vec![("none".into(), i18n::tr("editor.group.none"))];
    for g in groups {
        opts.push((g.id.clone(), g.name.clone()));
    }
    opts
}

// ── editor state ─────────────────────────────────────────

/// Mutable state for the host editor form.
#[derive(Debug, Clone)]
pub struct EditorState {
    data: HostData,
    groups: Vec<Group>,
    fields: Vec<EditorField>,
    pub current_field: usize,
    pub is_new: bool,
}

#[derive(Debug, Clone)]
struct HostData {
    display_name: String,
    address: String,
    port: String,
    username: String,
    auth_type: String,
    password: String,
    key_path: String,
    group_id: String, // "none" means no group
    color: String,
    notes: String,
    host_id: String,
}

const BOX_INNER_W: usize = 34;

impl EditorState {
    pub fn new(host: Host, groups: Vec<Group>, saved_password: String) -> Self {
        let data = HostData {
            display_name: host.display_name.clone(),
            address: host.address.clone(),
            port: host.port.to_string(),
            username: host.username.clone(),
            auth_type: match host.auth_type {
                AuthType::Password => "password".into(),
                AuthType::Key => "key".into(),
                AuthType::Interactive => "interactive".into(),
            },
            password: saved_password,
            key_path: host.key_path.clone().unwrap_or_default(),
            group_id: host.group_id.as_deref().unwrap_or("none").to_string(),
            color: host.color.as_deref().unwrap_or("none").to_string(),
            notes: host.notes.clone().unwrap_or_default(),
            host_id: host.id.clone(),
        };
        let fields = build_fields(&data, &groups);
        Self { data, groups, fields, current_field: 0, is_new: false }
    }

    pub fn new_empty(groups: Vec<Group>) -> Self {
        let mut s = Self::new(Host::new(String::new(), String::new(), String::new()), groups, String::new());
        s.is_new = true;
        s
    }

    /// Return the host display name being edited.
    pub fn host_display_name(&self) -> &str { &self.data.display_name }

    /// Get the string value of the currently focused field.
    pub fn current_value(&self) -> &str {
        &self.fields[self.current_field].value
    }

    /// Update the value of the currently focused field.
    pub fn set_current_value(&mut self, val: String) {
        let key = self.fields[self.current_field].key.clone();
        self.data.set(&key, &val);
        self.fields[self.current_field].value = val;
    }

    /// Advance or retreat the selected option in a Select/ColorSelect field.
    pub fn cycle_option(&mut self, forward: bool) {
        let is_auth = self.fields[self.current_field].key == FieldKey::AuthType;
        let new_val = {
            let field = &self.fields[self.current_field];
            match &field.field_type {
                FieldType::Select { options } => cycle_in(options, &field.value, forward),
                FieldType::ColorSelect { options } => {
                    let keys: Vec<(String, String)> =
                        options.iter().map(|(k, _)| (k.to_string(), k.to_string())).collect();
                    cycle_in(&keys, &field.value, forward)
                }
                _ => return,
            }
        };
        self.set_current_value(new_val);
        if is_auth { self.rebuild_fields(); }
    }

    /// Whether the current field uses option cycling instead of free text input.
    pub fn is_select_field(&self) -> bool {
        matches!(
            self.fields[self.current_field].field_type,
            FieldType::Select { .. } | FieldType::ColorSelect { .. }
        )
    }

    /// Move focus to the next field.
    pub fn next_field(&mut self) {
        if self.current_field < self.fields.len() - 1 { self.current_field += 1; }
    }
    /// Move focus to the previous field.
    pub fn prev_field(&mut self) {
        if self.current_field > 0 { self.current_field -= 1; }
    }

    fn rebuild_fields(&mut self) {
        let cursor_key = self.fields.get(self.current_field).map(|f| f.key.clone());
        self.fields = build_fields(&self.data, &self.groups);
        self.current_field = if let Some(ref key) = cursor_key {
            self.fields.iter().position(|f| f.key == *key).unwrap_or(0)
        } else { 0 };
    }

    /// Consume the editor state and produce a persisted [`Host`].
    pub fn into_host(self) -> Host {
        let d = self.data;
        let mut host = Host::new(d.display_name, d.address, d.username);
        host.id = d.host_id;
        host.port = d.port.parse().unwrap_or(22);
        host.auth_type = match d.auth_type.as_str() {
            "password" => AuthType::Password,
            "key" => AuthType::Key,
            _ => AuthType::Interactive,
        };
        host.key_path = if d.key_path.is_empty() { None } else { Some(d.key_path) };
        host.group_id = if d.group_id == "none" || d.group_id.is_empty() { None } else { Some(d.group_id) };
        host.color = if d.color == "none" || d.color.is_empty() { None } else { Some(d.color) };
        host.notes = if d.notes.is_empty() { None } else { Some(d.notes) };
        host.password = if d.password.is_empty() { None } else { Some(d.password) };
        host
    }
}

impl HostData {
    fn set(&mut self, key: &FieldKey, val: &str) {
        match key {
            FieldKey::DisplayName => self.display_name = val.to_string(),
            FieldKey::Address => self.address = val.to_string(),
            FieldKey::Port => self.port = val.to_string(),
            FieldKey::Username => self.username = val.to_string(),
            FieldKey::AuthType => self.auth_type = val.to_string(),
            FieldKey::Password => self.password = val.to_string(),
            FieldKey::KeyPath => self.key_path = val.to_string(),
            FieldKey::Group => self.group_id = val.to_string(),
            FieldKey::Color => self.color = val.to_string(),
            FieldKey::Notes => self.notes = val.to_string(),
        }
    }
}

fn cycle_in(options: &[(String, String)], current: &str, forward: bool) -> String {
    let pos = options.iter().position(|(k, _)| k == current).unwrap_or(0);
    let len = options.len();
    let new = if forward { (pos + 1) % len } else { (pos + len - 1) % len };
    options[new].0.clone()
}

fn build_fields(d: &HostData, groups: &[Group]) -> Vec<EditorField> {
    let mut fields = vec![
        EditorField { key: FieldKey::DisplayName, label: i18n::tr("editor.field.display_name"), value: d.display_name.clone(), field_type: FieldType::Text },
        EditorField { key: FieldKey::Address,     label: i18n::tr("editor.field.address"),      value: d.address.clone(),      field_type: FieldType::Text },
        EditorField { key: FieldKey::Port,        label: i18n::tr("editor.field.port"),          value: d.port.clone(),         field_type: FieldType::Text },
        EditorField { key: FieldKey::Username,    label: i18n::tr("editor.field.username"),      value: d.username.clone(),     field_type: FieldType::Text },
        EditorField { key: FieldKey::AuthType,    label: i18n::tr("editor.field.auth_type"),     value: d.auth_type.clone(),    field_type: FieldType::Select { options: auth_options() } },
    ];

    match d.auth_type.as_str() {
        "password" => fields.push(EditorField { key: FieldKey::Password, label: i18n::tr("editor.field.password"), value: d.password.clone(), field_type: FieldType::Password }),
        "key" => fields.push(EditorField { key: FieldKey::KeyPath, label: i18n::tr("editor.field.key_path"), value: d.key_path.clone(), field_type: FieldType::Text }),
        _ => {}
    }

    fields.extend([
        EditorField { key: FieldKey::Group, label: i18n::tr("editor.field.group"), value: d.group_id.clone(), field_type: FieldType::Select { options: group_options(groups) } },
        EditorField { key: FieldKey::Color, label: i18n::tr("editor.field.color"), value: d.color.clone(),    field_type: FieldType::ColorSelect { options: color_options() } },
        EditorField { key: FieldKey::Notes, label: i18n::tr("editor.field.notes"), value: d.notes.clone(),    field_type: FieldType::Text },
    ]);
    fields
}

// ── rendering helpers ────────────────────────────────────

use unicode_width::UnicodeWidthStr;

fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

fn text_box_line(val: &str, is_selected: bool, is_password: bool, theme: &Theme) -> Line<'static> {
    let display = if is_password { "•".repeat(val.chars().count()) } else { val.to_string() };
    let cursor = if is_selected { "│" } else { " " };
    let cursor_w = 1;
    let content_w = 1 + display_width(&display) + cursor_w;
    let padding = BOX_INNER_W.saturating_sub(content_w);
    let padding_str: String = " ".repeat(padding);

    Line::from(vec![
        Span::styled("  │", Style::default().fg(theme.border)),
        Span::styled(format!(" {}", display), Style::default().fg(if is_selected { theme.fg } else { theme.detail_value })),
        Span::styled(cursor, Style::default().fg(theme.accent)),
        Span::styled(padding_str, Style::default()),
        Span::styled("│", Style::default().fg(theme.border)),
    ])
}

fn box_top(theme: &Theme) -> Line<'static> {
    let bar: String = "─".repeat(BOX_INNER_W);
    Line::from(vec![
        Span::styled("  ┌", Style::default().fg(theme.border)),
        Span::styled(bar, Style::default().fg(theme.border)),
        Span::styled("┐", Style::default().fg(theme.border)),
    ])
}

fn box_bottom(theme: &Theme) -> Line<'static> {
    let bar: String = "─".repeat(BOX_INNER_W);
    Line::from(vec![
        Span::styled("  └", Style::default().fg(theme.border)),
        Span::styled(bar, Style::default().fg(theme.border)),
        Span::styled("┘", Style::default().fg(theme.border)),
    ])
}

// ── main render ──────────────────────────────────────────

/// Lines consumed by each field type (label + input + spacing).
fn field_height(field: &EditorField) -> usize {
    match field.field_type {
        FieldType::Text | FieldType::Password => 5, // label + box_top + box_line + box_bottom + blank
        FieldType::Select { .. } | FieldType::ColorSelect { .. } => 3, // label + widget + blank
    }
}

pub fn render_editor(
    frame: &mut ratatui::Frame,
    area: Rect,
    state: &EditorState,
    theme: &Theme,
) {
    frame.render_widget(Clear, area);

    let title_key = if state.is_new { "editor.title_new" } else { "editor.title" };
    let title_text = if state.is_new || state.host_display_name().is_empty() {
        i18n::tr(title_key)
    } else {
        format!("{} {}", i18n::tr(title_key), state.host_display_name())
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut field_line_offsets: Vec<usize> = Vec::new();
    let mut line = 3usize; // header: blank + title + blank

    for field in &state.fields {
        field_line_offsets.push(line);
        line += field_height(field);
    }
    // Recalculate: we want the label line of the current field
    let target_line = field_line_offsets.get(state.current_field).copied().unwrap_or(0);

    // Calculate scroll to keep current field visible
    let visible_height = inner.height as usize;
    let total_lines = line + 1; // +1 for save hint
    let scroll = if total_lines > visible_height {
        // Scroll so target_line is visible, prefer showing it near the top
        let max_scroll = total_lines.saturating_sub(visible_height);
        if target_line >= max_scroll {
            max_scroll
        } else {
            target_line
        }
    } else {
        0
    };

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", title_text),
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (i, field) in state.fields.iter().enumerate() {
        let sel = i == state.current_field;

        lines.push(Line::from(Span::styled(
            format!("  {}", field.label),
            Style::default()
                .fg(if sel { theme.accent } else { theme.detail_key })
                .add_modifier(if sel { Modifier::BOLD } else { Modifier::empty() }),
        )));

        match &field.field_type {
            FieldType::Text | FieldType::Password => {
                lines.push(box_top(theme));
                lines.push(text_box_line(&field.value, sel, matches!(field.field_type, FieldType::Password), theme));
                lines.push(box_bottom(theme));
            }
            FieldType::Select { options } => {
                let label = options.iter().find(|(k, _)| k == &field.value).map(|(_, l)| l.clone()).unwrap_or_default();
                let la = if sel { "◂ " } else { "  " };
                let ra = if sel { " ▸" } else { "  " };
                let hint = if sel { format!("  {}", i18n::tr("editor.select_hint")) } else { String::new() };
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(la, Style::default().fg(theme.muted)),
                    Span::styled(format!(" {} ", label), Style::default().fg(if sel { theme.accent } else { theme.detail_value }).add_modifier(if sel { Modifier::BOLD } else { Modifier::empty() })),
                    Span::styled(ra, Style::default().fg(theme.muted)),
                    Span::styled(hint, Style::default().fg(theme.muted)),
                ]));
            }
            FieldType::ColorSelect { options } => {
                let mut spans = vec![Span::styled("  ", Style::default())];
                for (name, (r, g, b)) in options.iter() {
                    let is_cur = *name == field.value;
                    let color = ratatui::style::Color::Rgb(*r, *g, *b);
                    if sel && is_cur {
                        spans.push(Span::styled("▐", Style::default().fg(theme.accent)));
                    }
                    spans.push(Span::styled("██", Style::default().fg(color)));
                    if sel && is_cur {
                        spans.push(Span::styled("▌", Style::default().fg(theme.accent)));
                    }
                    spans.push(Span::styled(" ", Style::default()));
                }
                if sel {
                    let cur = options.iter().find(|(n, _)| *n == field.value).map(|(n, _)| n.to_string()).unwrap_or_default();
                    spans.push(Span::styled(format!(" {}", cur), Style::default().fg(theme.muted)));
                }
                lines.push(Line::from(spans));
            }
        }
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        format!("  {}", i18n::tr("editor.save")),
        Style::default().fg(theme.muted),
    )));

    let pad_block = Block::default()
        .padding(ratatui::widgets::Padding::horizontal(3))
        .style(Style::default().bg(theme.bg));
    let paragraph = Paragraph::new(lines)
        .block(pad_block)
        .wrap(Wrap { trim: true })
        .scroll((scroll as u16, 0));
    frame.render_widget(paragraph, inner);
}
