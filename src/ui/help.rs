use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::i18n;
use crate::theme::Theme;

/// Render the keyboard shortcuts help overlay.
pub fn render_help(frame: &mut ratatui::Frame, area: Rect, theme: &Theme) {
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("help.title")),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        help_line("j / ↓", &i18n::tr("help.move_down"), theme),
        help_line("k / ↑", &i18n::tr("help.move_up"), theme),
        help_line("h / l", &i18n::tr("help.switch_panel"), theme),
        help_line("Enter", &i18n::tr("help.connect"), theme),
        help_line("e", &i18n::tr("help.edit"), theme),
        help_line("d", &i18n::tr("help.delete"), theme),
        help_line("n", &i18n::tr("help.new_host"), theme),
        help_line("g", &i18n::tr("help.new_group"), theme),
        help_line("/", &i18n::tr("help.search"), theme),
        help_line("Esc", &i18n::tr("help.escape"), theme),
        help_line("q", &i18n::tr("help.quit"), theme),
        help_line("?", &i18n::tr("help.help"), theme),
        help_line("t", &i18n::tr("help.theme"), theme),
        help_line("L", &i18n::tr("help.lang"), theme),
        help_line("Shift+J/K", &i18n::tr("help.reorder"), theme),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("help.close")),
            Style::default().fg(theme.muted),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(theme.accent))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let pad_block = Block::default()
        .padding(Padding::horizontal(3))
        .style(Style::default().bg(theme.bg));
    let paragraph = Paragraph::new(lines).block(pad_block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, inner);
}

fn help_line(key: &str, desc: &str, theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {:-12}", key),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(desc.to_string(), Style::default().fg(theme.fg)),
    ])
}
