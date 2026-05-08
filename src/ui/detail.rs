use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};

use crate::config::model::{AuthType, Group, Host};
use crate::ui::colors::parse_color;
use crate::i18n;
use crate::theme::Theme;

/// Tab in the detail panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailTab {
    Overview,
    Connection,
    Notes,
}

impl DetailTab {
    pub fn items() -> [Self; 3] {
        [Self::Overview, Self::Connection, Self::Notes]
    }

    pub fn next(self) -> Self {
        match self {
            Self::Overview => Self::Connection,
            Self::Connection => Self::Notes,
            Self::Notes => Self::Overview,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Overview => Self::Notes,
            Self::Connection => Self::Overview,
            Self::Notes => Self::Connection,
        }
    }
}

/// What is currently selected in the sidebar: nothing, a host, or a group with its hosts.
pub enum DetailSelection<'a> {
    None,
    Host(&'a Host),
    Group(&'a Group, Vec<&'a Host>),
}

/// Render the right-hand detail panel for the current selection.
pub fn render_detail(
    frame: &mut ratatui::Frame,
    area: Rect,
    selection: DetailSelection,
    theme: &Theme,
    focused: bool,
    active_tab: DetailTab,
) {
    let block = Block::default()
        .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
        .border_style(if focused {
            Style::default().fg(theme.accent)
        } else {
            Style::default().fg(theme.border)
        })
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    match selection {
        DetailSelection::None => {
            render_welcome(frame, inner, theme);
        }
        DetailSelection::Host(host) => {
            let tab_bar_height: u16 = 2;
            let tab_bar = Rect {
                height: tab_bar_height.min(inner.height),
                ..inner
            };
            let content = Rect {
                y: inner.y + tab_bar_height.min(inner.height),
                height: inner.height.saturating_sub(tab_bar_height),
                ..inner
            };

            render_tab_bar(frame, tab_bar, theme, active_tab);
            render_tab_content(frame, content, host, theme, active_tab);
        }
        DetailSelection::Group(group, hosts) => {
            let tab_bar_height: u16 = 2;
            let tab_bar = Rect {
                height: tab_bar_height.min(inner.height),
                ..inner
            };
            let content = Rect {
                y: inner.y + tab_bar_height.min(inner.height),
                height: inner.height.saturating_sub(tab_bar_height),
                ..inner
            };

            render_group_tab_bar(frame, tab_bar, theme, active_tab);
            render_group_tab_content(frame, content, group, &hosts, theme, active_tab);
        }
    }
}

fn render_group_tab_bar(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
    active_tab: DetailTab,
) {
    let tab_labels: Vec<Line> = DetailTab::items()
        .iter()
        .map(|tab| {
            let label = match tab {
                DetailTab::Overview => i18n::tr("detail.tab.overview"),
                DetailTab::Connection => i18n::tr("detail.tab.connection"),
                DetailTab::Notes => i18n::tr("detail.tab.notes"),
            };
            Line::from(Span::styled(
                format!(" {} ", label),
                Style::default().fg(if *tab == active_tab {
                    theme.accent
                } else {
                    theme.muted
                }),
            ))
        })
        .collect();

    let selected = match active_tab {
        DetailTab::Overview => 0,
        DetailTab::Connection => 1,
        DetailTab::Notes => 2,
    };

    let tabs = Tabs::new(tab_labels)
        .select(selected)
        .divider(Span::styled("│", Style::default().fg(theme.border)))
        .style(Style::default().bg(theme.bg));

    frame.render_widget(tabs, area);
}

fn render_group_tab_content(
    frame: &mut ratatui::Frame,
    area: Rect,
    group: &Group,
    hosts: &[&Host],
    theme: &Theme,
    active_tab: DetailTab,
) {
    let lines = match active_tab {
        DetailTab::Overview => build_group_overview(group, hosts, theme),
        _ => vec![Line::from("")],
    };
    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn build_group_overview(group: &Group, hosts: &[&Host], theme: &Theme) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  \u{F024B} ", Style::default().fg(theme.group_active)),
            Span::styled(
                group.name.clone(),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            format!("  {} {}", hosts.len(), i18n::tr("detail.group.host_count")),
            Style::default().fg(theme.muted),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  ──────────────────────────────────────",
            Style::default().fg(theme.border),
        )),
        Line::from(""),
    ];

    if hosts.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  {}", i18n::tr("detail.group.empty")),
            Style::default().fg(theme.muted),
        )));
    } else {
        for host in hosts {
            let dot_color = host
                .color
                .as_deref()
                .and_then(parse_color)
                .map(|(r, g, b)| ratatui::style::Color::Rgb(r, g, b))
                .unwrap_or(ratatui::style::Color::Rgb(186, 194, 222));
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled("\u{F048B} ", Style::default().fg(dot_color)),
                Span::styled(
                    host.display_name.clone(),
                    Style::default().fg(theme.fg),
                ),
                Span::styled(
                    format!("  ({})", host.address),
                    Style::default().fg(theme.muted),
                ),
            ]));
        }
    }

    lines.extend([
        Line::from(""),
        Line::from(Span::styled(
            "  ──────────────────────────────────────",
            Style::default().fg(theme.border),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [n] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.new_host"), Style::default().fg(theme.muted)),
            Span::styled("  [g] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.new_group"), Style::default().fg(theme.muted)),
        ]),
        Line::from(vec![
            Span::styled("  [/] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.search"), Style::default().fg(theme.muted)),
            Span::styled("  [q] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.quit"), Style::default().fg(theme.muted)),
        ]),
    ]);
    lines
}

fn render_tab_bar(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
    active_tab: DetailTab,
) {
    let tab_labels: Vec<Line> = DetailTab::items()
        .iter()
        .map(|tab| {
            let label = match tab {
                DetailTab::Overview => i18n::tr("detail.tab.overview"),
                DetailTab::Connection => i18n::tr("detail.tab.connection"),
                DetailTab::Notes => i18n::tr("detail.tab.notes"),
            };
            Line::from(Span::styled(
                format!(" {} ", label),
                Style::default().fg(if *tab == active_tab {
                    theme.accent
                } else {
                    theme.muted
                }),
            ))
        })
        .collect();

    let selected = match active_tab {
        DetailTab::Overview => 0,
        DetailTab::Connection => 1,
        DetailTab::Notes => 2,
    };

    let tabs = Tabs::new(tab_labels)
        .select(selected)
        .divider(Span::styled("│", Style::default().fg(theme.border)))
        .style(Style::default().bg(theme.bg));

    frame.render_widget(tabs, area);
}

fn render_tab_content(
    frame: &mut ratatui::Frame,
    area: Rect,
    host: &Host,
    theme: &Theme,
    active_tab: DetailTab,
) {
    let lines = match active_tab {
        DetailTab::Overview => build_overview(host, theme),
        DetailTab::Connection => build_connection(host, theme),
        DetailTab::Notes => build_notes(host, theme),
    };

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn build_overview(host: &Host, theme: &Theme) -> Vec<Line<'static>> {
    let dot_color = host
        .color
        .as_deref()
        .and_then(parse_color)
        .map(|(r, g, b)| ratatui::style::Color::Rgb(r, g, b))
        .unwrap_or(theme.accent);

    let default_color = i18n::tr("detail.default");
    let color_display = host.color.as_deref().unwrap_or(&default_color);

    vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ● ", Style::default().fg(dot_color)),
            Span::styled(
                host.display_name.clone(),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            format!("  {} {}", color_display, i18n::tr("detail.color")),
            Style::default().fg(theme.muted),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  ──────────────────────────────────────",
            Style::default().fg(theme.border),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Enter] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.connect"), Style::default().fg(theme.muted)),
            Span::styled("   [e] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.edit"), Style::default().fg(theme.muted)),
            Span::styled("   [d] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.delete"), Style::default().fg(theme.muted)),
        ]),
        Line::from(vec![
            Span::styled("  [n] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.new_host"), Style::default().fg(theme.muted)),
            Span::styled("  [g] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.new_group"), Style::default().fg(theme.muted)),
        ]),
        Line::from(vec![
            Span::styled("  [/] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.search"), Style::default().fg(theme.muted)),
            Span::styled("  [q] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(i18n::tr("detail.action.quit"), Style::default().fg(theme.muted)),
        ]),
    ]
}

fn build_connection(host: &Host, theme: &Theme) -> Vec<Line<'static>> {
    let auth_label = match host.auth_type {
        AuthType::Password => i18n::tr("detail.auth_password"),
        AuthType::Key => i18n::tr("detail.auth_key"),
        AuthType::Interactive => i18n::tr("detail.auth_interactive"),
    };
    let auth_detail = match host.auth_type {
        AuthType::Key => host
            .key_path
            .as_deref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| i18n::tr("detail.not_set")),
        AuthType::Password => "••••••".to_string(),
        AuthType::Interactive => i18n::tr("detail.manual_input"),
    };

    vec![
        Line::from(""),
        Line::from(""),
        detail_line(&i18n::tr("detail.address"), &format!("{}:{}", host.address, host.port), theme),
        detail_line(&i18n::tr("detail.user"), &host.username, theme),
        Line::from(""),
        detail_line(&i18n::tr("detail.auth"), &auth_label, theme),
        detail_line("", &auth_detail, theme),
    ]
}

fn build_notes(host: &Host, theme: &Theme) -> Vec<Line<'static>> {
    let no_notes = i18n::tr("detail.none");
    let notes = host.notes.as_deref().unwrap_or(&no_notes);

    vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", notes),
            Style::default().fg(theme.fg),
        )),
    ]
}

fn render_welcome(frame: &mut ratatui::Frame, area: Rect, theme: &Theme) {
    let lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("detail.welcome")),
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("detail.welcome_hint")),
            Style::default().fg(theme.muted),
        )),
    ];
    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn detail_line(key: &str, value: &str, theme: &Theme) -> Line<'static> {
    if key.is_empty() {
        Line::from(vec![
            Span::styled("           ", Style::default()),
            Span::styled(value.to_string(), Style::default().fg(theme.detail_value)),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                format!("  {:>6}  ", key),
                Style::default().fg(theme.detail_key),
            ),
            Span::styled(value.to_string(), Style::default().fg(theme.detail_value)),
        ])
    }
}
