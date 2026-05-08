use super::*;
use crate::config::model::Host;
use crate::ssh::command;
use crate::ui::confirm;
use crate::ui::lang_selector;
use crate::ui::sidebar;

impl App {
    pub(super) fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        let t = theme::get_theme(&self.config.settings.theme);
        terminal.draw(|frame| {
            let size = frame.area();
            match self.mode {
                Mode::PasswordInput
                | Mode::SetMasterPassword
                | Mode::SetMasterPasswordConfirm => {
                    let pw_area = centered_rect(size, 60, 55);
                    self.draw_password_screen(frame, pw_area, &t);
                }
                Mode::Editing => {
                    self.draw_main(frame, size, &t);
                    if let Some(ref editor) = self.editor_state {
                        let editor_area = centered_rect(size, 55, 80);
                        editor::render_editor(frame, editor_area, editor, &t);
                    }
                }
                Mode::Help => {
                    self.draw_main(frame, size, &t);
                    let help_area = centered_rect(size, 50, 70);
                    help::render_help(frame, help_area, &t);
                }
                Mode::ThemeSelector => {
                    self.draw_main(frame, size, &t);
                    let theme_area = centered_rect(size, 40, 50);
                    theme_selector::render_theme_selector(
                        frame,
                        theme_area,
                        &t,
                        self.theme_selector_index,
                    );
                }
                Mode::LangSelector => {
                    self.draw_main(frame, size, &t);
                    let lang_area = centered_rect(size, 40, 40);
                    lang_selector::render_lang_selector(
                        frame,
                        lang_area,
                        &t,
                        self.lang_selector_index,
                    );
                }
                Mode::ConfirmDelete => {
                    self.draw_main(frame, size, &t);
                    let confirm_area = centered_rect(size, 50, 30);
                    confirm::render_confirm(
                        frame,
                        confirm_area,
                        &self.confirm_message,
                        &t,
                    );
                }
                Mode::ConfirmDeleteGroup => {
                    self.draw_main(frame, size, &t);
                    let confirm_area = centered_rect(size, 50, 28);
                    confirm::render_delete_group(frame, confirm_area, &t);
                }
                Mode::ConfirmReset => {
                    let confirm_area = centered_rect(size, 50, 25);
                    confirm::render_confirm(
                        frame,
                        confirm_area,
                        &i18n::tr("confirm.reset"),
                        &t,
                    );
                }
                Mode::ConfirmSkipPassword => {
                    let pw_area = centered_rect(size, 60, 55);
                    self.draw_password_screen(frame, pw_area, &t);
                    let confirm_area = centered_rect(size, 50, 30);
                    confirm::render_skip_password(frame, confirm_area, &t);
                }
                Mode::CreateGroup => {
                    self.draw_main(frame, size, &t);
                    let group_area = centered_rect(size, 40, 25);
                    self.draw_create_group(frame, group_area, &t);
                }
                _ => {
                    self.draw_main(frame, size, &t);
                }
            }
        })?;
        Ok(())
    }

    fn draw_main(&self, frame: &mut ratatui::Frame, area: Rect, t: &theme::Theme) {
        let (title_area, sidebar_area, detail_area, search_area) =
            ui_layout::main_layout(area);

        let host_count = self.config.hosts.len();

        let title_content = Paragraph::new(Line::from(vec![
            Span::styled(
                " Wormhole ",
                Style::default()
                    .fg(t.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}台主机", host_count),
                Style::default().fg(t.muted),
            ),
        ]))
        .style(Style::default().bg(t.bg))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(t.border)),
        );
        frame.render_widget(title_content, title_area);

        let sidebar_ctx = SidebarContext {
            config: &self.config,
            theme: t,
            expanded: &self.expanded_groups,
            items: &self.sidebar_items,
            selected: self.selected_index,
            focused: self.focus == Focus::Sidebar,
            search_query: &self.search_query,
            search_active: self.mode == Mode::Search,
        };
        sidebar::render_sidebar(frame, sidebar_area, &sidebar_ctx);

        let selection = self.get_detail_selection();
        detail::render_detail(frame, detail_area, selection, t, self.focus == Focus::Detail, self.detail_tab);

        let hint = Paragraph::new(Line::from(Span::styled(
            i18n::tr("bottom_bar.hint"),
            Style::default().fg(t.muted),
        )))
        .style(Style::default().bg(t.bg));
        frame.render_widget(hint, search_area);
    }

    fn draw_password_screen(
        &self,
        frame: &mut ratatui::Frame,
        area: Rect,
        t: &theme::Theme,
    ) {
        let sublabel = match self.mode {
            Mode::PasswordInput => i18n::tr("password.enter"),
            Mode::SetMasterPassword => i18n::tr("password.set_hint"),
            Mode::SetMasterPasswordConfirm => i18n::tr("password.confirm_hint"),
            _ => return,
        };

        let pw_dots = "•".repeat(self.password_input.len());
        let pw_inner = 30usize;
        let pw_cursor = if self.password_input.is_empty() { "│" } else { "" };
        let dots_width = unicode_width::UnicodeWidthStr::width(pw_dots.as_str());
        let cursor_width = if self.password_input.is_empty() { 1 } else { 0 };
        let pw_content_w = 1 + dots_width + cursor_width;
        let pw_pad = pw_inner.saturating_sub(pw_content_w);
        let pw_fill: String = " ".repeat(pw_pad);
        let bar: String = "─".repeat(pw_inner);

        let banner_style = Style::default().fg(t.accent).add_modifier(Modifier::BOLD);

        let mut lines = vec![
            Line::from(""),
        ];
        for bl in BANNER_LINES {
            lines.push(Line::from(Span::styled(
                format!(" {}", bl),
                banner_style,
            )));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  {}", sublabel),
            Style::default().fg(t.muted),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  ┌", Style::default().fg(t.border)),
            Span::styled(bar.clone(), Style::default().fg(t.border)),
            Span::styled("┐", Style::default().fg(t.border)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  │", Style::default().fg(t.border)),
            Span::styled(format!(" {}", pw_dots), Style::default().fg(t.fg)),
            Span::styled(pw_cursor, Style::default().fg(t.accent)),
            Span::styled(pw_fill, Style::default()),
            Span::styled("│", Style::default().fg(t.border)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  └", Style::default().fg(t.border)),
            Span::styled(bar, Style::default().fg(t.border)),
            Span::styled("┘", Style::default().fg(t.border)),
        ]));

        if !self.password_error.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  {} {}", icons::ERROR, self.password_error),
                Style::default().fg(t.error),
            )));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  {}", i18n::tr("password.confirm_btn")),
            Style::default().fg(t.muted),
        )));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(t.border))
            .style(Style::default().bg(t.bg));
        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }

    fn draw_create_group(
        &self,
        frame: &mut ratatui::Frame,
        area: Rect,
        t: &theme::Theme,
    ) {
        frame.render_widget(Clear, area);

        let input_inner = 30usize;
        let cursor = if self.group_name_input.is_empty() { "│" } else { "" };
        let input_w = unicode_width::UnicodeWidthStr::width(self.group_name_input.as_str());
        let cursor_w = if self.group_name_input.is_empty() { 1 } else { 0 };
        let content_w = 1 + input_w + cursor_w;
        let pad = input_inner.saturating_sub(content_w);
        let fill: String = " ".repeat(pad);
        let bar: String = "─".repeat(input_inner);

        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  {}", i18n::tr("group.create_title")),
                Style::default().fg(t.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("  {}", i18n::tr("group.name_hint")),
                Style::default().fg(t.muted),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ┌", Style::default().fg(t.border)),
                Span::styled(bar.clone(), Style::default().fg(t.border)),
                Span::styled("┐", Style::default().fg(t.border)),
            ]),
            Line::from(vec![
                Span::styled("  │", Style::default().fg(t.border)),
                Span::styled(format!(" {}", self.group_name_input), Style::default().fg(t.fg)),
                Span::styled(cursor, Style::default().fg(t.accent)),
                Span::styled(fill, Style::default()),
                Span::styled("│", Style::default().fg(t.border)),
            ]),
            Line::from(vec![
                Span::styled("  └", Style::default().fg(t.border)),
                Span::styled(bar, Style::default().fg(t.border)),
                Span::styled("┘", Style::default().fg(t.border)),
            ]),
        ];

        if !self.password_error.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  {} {}", icons::ERROR, self.password_error),
                Style::default().fg(t.error),
            )));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  [Enter] 确认   [Esc] 取消",
            Style::default().fg(t.muted),
        )));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(t.accent))
            .style(Style::default().bg(t.bg));
        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }

    pub(super) fn get_detail_selection(&self) -> DetailSelection<'_> {
        if let Some(idx) = self.selected_index {
            match self.sidebar_items.get(idx) {
                Some(SidebarItem::Host { index, .. }) => {
                    if let Some(host) = self.config.hosts.get(*index) {
                        return DetailSelection::Host(host);
                    }
                }
                Some(SidebarItem::Group { index }) => {
                    if let Some(group) = self.config.groups.get(*index) {
                        let hosts: Vec<&Host> = self
                            .config
                            .hosts
                            .iter()
                            .filter(|h| h.group_id.as_deref() == Some(&group.id))
                            .collect();
                        return DetailSelection::Group(group, hosts);
                    }
                }
                None => {}
            }
        }
        DetailSelection::None
    }

    pub(super) fn vault_salt_bytes(&self) -> Vec<u8> {
        match self.config.vault_salt {
            Some(ref s) => hex::decode(s).unwrap_or_default(),
            None => b"wormhole-salt".to_vec(),
        }
    }

    pub(super) fn clamp_selection(&mut self) {
        if self.sidebar_items.is_empty() {
            self.selected_index = None;
        } else if self.selected_index.is_none() {
            self.selected_index = Some(0);
        } else {
            self.selected_index = Some(self.selected_index.unwrap_or(0).min(self.sidebar_items.len() - 1));
        }
    }

    pub(super) fn rebuild_sidebar(&mut self) {
        self.sidebar_items =
            sidebar::build_sidebar_items(&self.config, &self.expanded_groups);
    }

    pub(super) fn apply_search(&mut self) {
        if self.search_query.is_empty() {
            self.rebuild_sidebar();
            return;
        }
        let query = self.search_query.to_lowercase();
        let mut matching_group_ids: Vec<String> = Vec::new();

        // Find groups whose name matches
        for group in &self.config.groups {
            if group.name.to_lowercase().contains(&query) {
                matching_group_ids.push(group.id.clone());
            }
        }

        // Build with all groups expanded so hosts inside matched groups show up
        let mut expanded = self.expanded_groups.clone();
        for gid in &matching_group_ids {
            if !expanded.is_expanded(gid) {
                expanded.toggle(gid);
            }
        }
        let all_items = sidebar::build_sidebar_items(&self.config, &expanded);

        let mut filtered = Vec::new();
        for item in &all_items {
            match item {
                SidebarItem::Host { index, group_index } => {
                    if let Some(host) = self.config.hosts.get(*index) {
                        let host_matches = host.display_name.to_lowercase().contains(&query)
                            || host.address.to_lowercase().contains(&query)
                            || host.username.to_lowercase().contains(&query)
                            || host.notes.as_deref().unwrap_or("").to_lowercase().contains(&query);
                        // Show host if it matches directly, or if its group matches
                        let group_matches = group_index
                            .and_then(|gi| self.config.groups.get(gi))
                            .is_some_and(|g| matching_group_ids.contains(&g.id));
                        if host_matches || group_matches {
                            filtered.push(item.clone());
                        }
                    }
                }
                SidebarItem::Group { index } => {
                    if let Some(group) = self.config.groups.get(*index)
                        && matching_group_ids.contains(&group.id) {
                            filtered.push(item.clone());
                        }
                }
            }
        }
        self.sidebar_items = filtered;
        if !self.sidebar_items.is_empty() {
            self.selected_index = Some(0);
        } else {
            self.selected_index = None;
        }
    }

    pub(super) fn connect_host(&self, host_idx: usize) {
        if let Some(host) = self.config.hosts.get(host_idx) {
            let password = self.passwords.get(&host.id).map(|s| s.as_str());
            let _ = command::exec_ssh(host, password);
        }
    }

    pub(super) fn get_selected_host(&self) -> Option<&Host> {
        if let Some(idx) = self.selected_index
            && let Some(SidebarItem::Host { index, .. }) = self.sidebar_items.get(idx) {
                return self.config.hosts.get(*index);
            }
        None
    }
}
