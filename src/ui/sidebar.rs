use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};

use crate::config::model::Config;
use crate::ui::colors::parse_color;
use crate::ui::icons;
use crate::theme::Theme;

/// Tracks which group IDs are currently expanded in the sidebar tree.
#[derive(Debug, Clone)]
pub struct ExpandedGroups(pub Vec<String>);

impl ExpandedGroups {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn toggle(&mut self, group_id: &str) {
        if let Some(pos) = self.0.iter().position(|id| id == group_id) {
            self.0.remove(pos);
        } else {
            self.0.push(group_id.to_string());
        }
    }

    pub fn is_expanded(&self, group_id: &str) -> bool {
        self.0.iter().any(|id| id == group_id)
    }
}

/// A flattened sidebar entry pointing to either a group or a host in the config.
#[derive(Debug, Clone)]
pub enum SidebarItem {
    Group { index: usize },
    Host { index: usize, group_index: Option<usize> },
}

/// Build the ordered list of sidebar items respecting sort order and group expansion.
pub fn build_sidebar_items(config: &Config, expanded: &ExpandedGroups) -> Vec<SidebarItem> {
    let valid_group_ids: Vec<&str> = config.groups.iter().map(|g| g.id.as_str()).collect();

    // Level-0 items: groups + ungrouped hosts, sorted by sort_order
    let mut level0: Vec<SidebarItem> = Vec::new();

    for (gi, _group) in config.groups.iter().enumerate() {
        level0.push(SidebarItem::Group { index: gi });
    }

    for (hi, host) in config.hosts.iter().enumerate() {
        let ungrouped = match host.group_id.as_deref() {
            None => true,
            Some(gid) => !valid_group_ids.contains(&gid),
        };
        if ungrouped {
            level0.push(SidebarItem::Host { index: hi, group_index: None });
        }
    }

    level0.sort_by(|a, b| {
        let oa = match a {
            SidebarItem::Group { index } => config.groups[*index].sort_order,
            SidebarItem::Host { index, .. } => config.hosts[*index].sort_order,
        };
        let ob = match b {
            SidebarItem::Group { index } => config.groups[*index].sort_order,
            SidebarItem::Host { index, .. } => config.hosts[*index].sort_order,
        };
        oa.cmp(&ob)
    });

    // Build final list with expanded group hosts
    let mut items = Vec::new();
    for item in &level0 {
        match item {
            SidebarItem::Group { index: gi } => {
                items.push(SidebarItem::Group { index: *gi });
                let group = &config.groups[*gi];
                if expanded.is_expanded(&group.id) {
                    let mut group_hosts: Vec<usize> = config
                        .hosts
                        .iter()
                        .enumerate()
                        .filter(|(_, h)| h.group_id.as_deref() == Some(&group.id))
                        .map(|(hi, _)| hi)
                        .collect();
                    group_hosts.sort_by_key(|hi| config.hosts[*hi].sort_order);

                    for hi in group_hosts {
                        items.push(SidebarItem::Host {
                            index: hi,
                            group_index: Some(*gi),
                        });
                    }
                }
            }
            SidebarItem::Host { index, .. } => {
                items.push(SidebarItem::Host {
                    index: *index,
                    group_index: None,
                });
            }
        }
    }

    items
}

/// Check whether two sidebar items are siblings at the same nesting level.
pub fn is_same_level(a: &SidebarItem, b: &SidebarItem) -> bool {
    match (a, b) {
        (SidebarItem::Group { .. }, SidebarItem::Group { .. }) => true,
        (SidebarItem::Group { .. }, SidebarItem::Host { group_index: None, .. }) => true,
        (SidebarItem::Host { group_index: None, .. }, SidebarItem::Group { .. }) => true,
        (
            SidebarItem::Host { group_index: None, .. },
            SidebarItem::Host { group_index: None, .. },
        ) => true,
        (
            SidebarItem::Host { group_index: Some(ga), .. },
            SidebarItem::Host { group_index: Some(gb), .. },
        ) => ga == gb,
        _ => false,
    }
}

/// Aggregate of data needed to render the sidebar in a single pass.
pub struct SidebarContext<'a> {
    pub config: &'a Config,
    pub theme: &'a Theme,
    pub expanded: &'a ExpandedGroups,
    pub items: &'a [SidebarItem],
    pub selected: Option<usize>,
    pub focused: bool,
    pub search_query: &'a str,
    pub search_active: bool,
}

/// Render the search bar and host/group list into the given area.
pub fn render_sidebar(
    frame: &mut ratatui::Frame,
    area: Rect,
    ctx: &SidebarContext,
) -> ListState {
    let config = ctx.config;
    let theme = ctx.theme;
    let expanded = ctx.expanded;
    let items = ctx.items;
    let selected = ctx.selected;
    let focused = ctx.focused;
    // Split area: search bar (4 lines: outer top + box top + content + box bottom) + list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(area);

    let search_area = chunks[0];
    let list_area = chunks[1];

    // Render search bar
    render_search_bar(frame, search_area, theme, ctx.search_query, ctx.search_active);

    // Render list
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| match item {
            SidebarItem::Group { index } => {
                let group = &config.groups[*index];
                let arrow = if expanded.is_expanded(&group.id) { "\u{F024B} " } else { "\u{F07B} " };
                let line = Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(arrow, Style::default().fg(theme.group_active)),
                    Span::styled(
                        group.name.clone(),
                        Style::default()
                            .fg(theme.group_active)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                ListItem::new(line)
            }
            SidebarItem::Host { index, group_index } => {
                let host = &config.hosts[*index];
                let indent = match group_index {
                    Some(_) => "    ",
                    None => "  ",
                };
                let dot_color = host
                    .color
                    .as_deref()
                    .and_then(parse_color)
                    .map(|(r, g, b)| ratatui::style::Color::Rgb(r, g, b))
                    .unwrap_or(ratatui::style::Color::Rgb(186, 194, 222));
                let line = Line::from(vec![
                    Span::styled(indent, Style::default()),
                    Span::styled("\u{F048B} ", Style::default().fg(dot_color)),
                    Span::styled(
                        host.display_name.clone(),
                        Style::default().fg(theme.host_normal),
                    ),
                ]);
                ListItem::new(line)
            }
        })
        .collect();

    let border_style = if focused {
        Style::default().fg(theme.accent)
    } else {
        Style::default().fg(theme.border)
    };

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .style(Style::default().bg(theme.bg)),
        )
        .highlight_style(
            Style::default()
                .bg(theme.highlight)
                .fg(theme.fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    let mut state = ListState::default();
    state.select(selected);
    frame.render_stateful_widget(list, list_area, &mut state);

    state
}

fn render_search_bar(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
    query: &str,
    active: bool,
) {
    let border_color = if active { theme.accent } else { theme.border };
    let bg = Style::default().bg(theme.bg);

    // Outer frame: right + top borders (aligns with sidebar)
    let outer = Block::default()
        .borders(Borders::RIGHT | Borders::TOP)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .style(bg);
    let outer_inner = outer.inner(area);
    frame.render_widget(outer, area);

    // Hand-drawn inner search box: ┌─┐  │ │  └─┘
    let w = outer_inner.width as usize;
    let border_style = Style::default().fg(border_color).bg(theme.bg);

    let top_line = Line::from(vec![
        Span::styled(format!("╭{}╮", "─".repeat(w.saturating_sub(2))), border_style),
    ]);
    let bot_line = Line::from(vec![
        Span::styled(format!("╰{}╯", "─".repeat(w.saturating_sub(2))), border_style),
    ]);

    let content_line = {
        let content_style = if active {
            Style::default().fg(theme.fg).bg(theme.bg)
        } else {
            Style::default().fg(theme.muted).bg(theme.bg)
        };
        let icon_style = if active {
            Style::default().fg(theme.accent).bg(theme.bg)
        } else {
            Style::default().fg(theme.muted).bg(theme.bg)
        };

        let inner_w = w.saturating_sub(2); // inside ╭─╮
        if active {
            let text = format!("{} {}", icons::SEARCH, query);
            let text_w = unicode_width::UnicodeWidthStr::width(text.as_str());
            let cursor_w = 1;
            let pad = inner_w.saturating_sub(text_w + cursor_w);
            Line::from(vec![
                Span::styled("│", border_style),
                Span::styled(format!("{} ", icons::SEARCH), icon_style),
                Span::styled(query.to_string(), content_style),
                Span::styled("│", Style::default().fg(theme.accent).bg(theme.bg)),
                Span::styled(" ".repeat(pad), bg),
                Span::styled("│", border_style),
            ])
        } else if query.is_empty() {
            let text = format!("{} search...", icons::SEARCH);
            let text_w = unicode_width::UnicodeWidthStr::width(text.as_str());
            let pad = inner_w.saturating_sub(text_w);
            Line::from(vec![
                Span::styled("│", border_style),
                Span::styled(format!("{} search...", icons::SEARCH), content_style),
                Span::styled(" ".repeat(pad), bg),
                Span::styled("│", border_style),
            ])
        } else {
            let text_w = unicode_width::UnicodeWidthStr::width(query);
            let icon_w = unicode_width::UnicodeWidthStr::width(format!("{} ", icons::SEARCH).as_str());
            let pad = inner_w.saturating_sub(icon_w + text_w);
            Line::from(vec![
                Span::styled("│", border_style),
                Span::styled(format!("{} ", icons::SEARCH), icon_style),
                Span::styled(query.to_string(), content_style),
                Span::styled(" ".repeat(pad), bg),
                Span::styled("│", border_style),
            ])
        }
    };

    let paragraph = Paragraph::new(vec![top_line, content_line, bot_line]);
    frame.render_widget(paragraph, outer_inner);
}
