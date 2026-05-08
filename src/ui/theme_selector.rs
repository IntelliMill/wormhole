use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::i18n;
use crate::theme::Theme;

/// Render the theme selection overlay with live preview.
pub fn render_theme_selector(
    frame: &mut ratatui::Frame,
    area: Rect,
    theme: &Theme,
    selected: usize,
) {
    frame.render_widget(Clear, area);

    let names = crate::theme::theme_names();

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("theme.title")),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (i, name) in names.iter().enumerate() {
        let is_sel = i == selected;
        let prefix = if is_sel { "  ▸ " } else { "    " };
        let th = crate::theme::get_theme(name);

        lines.push(Line::from(vec![
            Span::styled(
                format!("{}{} ", prefix, name),
                Style::default()
                    .fg(if is_sel { theme.accent } else { theme.fg })
                    .add_modifier(if is_sel { Modifier::BOLD } else { Modifier::empty() }),
            ),
            Span::styled("●●●", Style::default().fg(th.accent)),
        ]));
    }

    lines.extend([
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", i18n::tr("theme.hint")),
            Style::default().fg(theme.muted),
        )),
    ]);

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
