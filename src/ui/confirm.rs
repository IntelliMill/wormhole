use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::config::storage;
use crate::i18n;
use crate::theme::Theme;
use crate::ui::icons;

/// Render a generic yes/no confirmation dialog.
pub fn render_confirm(
    frame: &mut ratatui::Frame,
    area: Rect,
    message: &str,
    theme: &Theme,
) {
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {} {}", icons::ERROR, message),
            Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("confirm.yes")),
            Style::default().fg(theme.muted),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.error))
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

/// Render the delete-group confirmation with options to remove hosts or move them to root.
pub fn render_delete_group(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
) {
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {} {}", icons::ERROR, i18n::tr("confirm.delete_group")),
            Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("confirm.delete_group_all")),
            Style::default().fg(theme.accent),
        )),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("confirm.delete_group_move")),
            Style::default().fg(theme.muted),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  [n/Esc] 取消",
            Style::default().fg(theme.muted),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.error))
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

/// Render the warning dialog shown when skipping master password setup.
pub fn render_skip_password(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
) {
    frame.render_widget(Clear, area);

    let path_str = storage::plain_passwords_path().display().to_string();

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {} {}", icons::ERROR, i18n::tr("confirm.skip_password.title")),
            Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("confirm.skip_password.warning")),
            Style::default().fg(theme.fg),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("confirm.skip_password.path")),
            Style::default().fg(theme.muted),
        )),
        Line::from(Span::styled(
            format!("  {}", path_str),
            Style::default().fg(theme.accent),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("confirm.yes")),
            Style::default().fg(theme.muted),
        )),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(theme.error))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let pad_block = Block::default()
        .padding(Padding::horizontal(3))
        .style(Style::default().bg(theme.bg));
    let paragraph = Paragraph::new(lines).block(pad_block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, inner);
}
