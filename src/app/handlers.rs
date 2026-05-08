use super::*;
use crossterm::event::{KeyCode, KeyEvent};
use crate::config::crypto;
use crate::config::storage;
use crate::i18n;
use crate::theme;
use crate::ui::editor::EditorState;
use crate::ui::sidebar::{SidebarItem, is_same_level};

impl App {
    pub(super) fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            Mode::PasswordInput => self.handle_password_input(key),
            Mode::SetMasterPassword => self.handle_set_master_password(key),
            Mode::SetMasterPasswordConfirm => {
                self.handle_set_master_password_confirm(key)
            }
            Mode::Normal => self.handle_normal(key),
            Mode::Search => self.handle_search(key),
            Mode::Editing => self.handle_editing(key),
            Mode::ConfirmDelete => self.handle_confirm_delete(key),
            Mode::ConfirmDeleteGroup => self.handle_confirm_delete_group(key),
            Mode::ConfirmReset => self.handle_confirm_reset(key),
            Mode::CreateGroup => self.handle_create_group(key),
            Mode::Help => self.handle_help(key),
            Mode::ThemeSelector => self.handle_theme_selector(key),
            Mode::LangSelector => self.handle_lang_selector(key),
            Mode::ConfirmSkipPassword => self.handle_confirm_skip_password(key),
        }
    }

    fn handle_normal(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('?') => {
                self.mode = Mode::Help;
            }
            KeyCode::Char('/') => {
                self.mode = Mode::Search;
                self.search_query.clear();
            }
            KeyCode::Char('t') => {
                self.enter_theme_selector();
            }
            KeyCode::Char('L') => {
                self.enter_lang_selector();
            }
            KeyCode::Char('n') => {
                let groups = self.config.groups.clone();
                self.editor_state = Some(EditorState::new_empty(groups));
                self.mode = Mode::Editing;
            }
            KeyCode::Char('g') => {
                self.group_name_input.clear();
                self.password_error.clear();
                self.mode = Mode::CreateGroup;
            }
            KeyCode::Char('e') => {
                if let Some(host) = self.get_selected_host() {
                    let groups = self.config.groups.clone();
                    let saved_password = self.passwords.get(&host.id).cloned().unwrap_or_default();
                    let editor = EditorState::new(host.clone(), groups, saved_password);
                    self.editor_state = Some(editor);
                    self.mode = Mode::Editing;
                }
            }
            KeyCode::Char('d') => {
                if self.get_selected_host().is_some() {
                    self.confirm_message = i18n::tr("confirm.delete");
                    self.mode = Mode::ConfirmDelete;
                } else if let Some(idx) = self.selected_index
                    && let Some(SidebarItem::Group { index }) = self.sidebar_items.get(idx).cloned() {
                        self.pending_delete_group_index = Some(index);
                        self.mode = Mode::ConfirmDeleteGroup;
                    }
            }
            KeyCode::Char('h') | KeyCode::Left => {
                self.focus = Focus::Sidebar;
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.focus = Focus::Detail;
            }
            KeyCode::Tab => {
                if self.focus == Focus::Detail {
                    self.detail_tab = self.detail_tab.next();
                }
            }
            KeyCode::BackTab => {
                if self.focus == Focus::Detail {
                    self.detail_tab = self.detail_tab.prev();
                }
            }
            KeyCode::Char('J') | KeyCode::Char('j')
                if key.modifiers.contains(event::KeyModifiers::SHIFT) =>
            {
                self.move_selected_forward();
            }
            KeyCode::Char('K') | KeyCode::Char('k')
                if key.modifiers.contains(event::KeyModifiers::SHIFT) =>
            {
                self.move_selected_backward();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_selection(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_selection(-1);
            }
            KeyCode::Enter => {
                if let Some(idx) = self.selected_index {
                    if let Some(SidebarItem::Host { index, .. }) =
                        self.sidebar_items.get(idx)
                    {
                        self.should_connect = Some(*index);
                    } else if let Some(SidebarItem::Group { index }) =
                        self.sidebar_items.get(idx)
                    {
                        let group = &self.config.groups[*index];
                        self.expanded_groups.toggle(&group.id);
                        self.rebuild_sidebar();
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_search(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.search_query.clear();
                self.rebuild_sidebar();
            }
            KeyCode::Enter => {
                self.mode = Mode::Normal;
                if self.selected_index.is_none() && !self.sidebar_items.is_empty() {
                    self.selected_index = Some(0);
                }
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.apply_search();
            }
            KeyCode::Down => {
                self.move_selection(1);
            }
            KeyCode::Up => {
                self.move_selection(-1);
            }
            KeyCode::Char('j') => {
                self.move_selection(1);
            }
            KeyCode::Char('k') => {
                self.move_selection(-1);
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.apply_search();
            }
            _ => {}
        }
    }

    fn handle_editing(&mut self, key: KeyEvent) {
        if let Some(ref mut editor) = self.editor_state {
            match key.code {
                KeyCode::Esc => {
                    self.editor_state = None;
                    self.mode = Mode::Normal;
                }
                KeyCode::Enter => {
                    self.save_editor();
                }
                KeyCode::Tab => {
                    editor.next_field();
                }
                KeyCode::BackTab => {
                    editor.prev_field();
                }
                KeyCode::Right => {
                    if editor.is_select_field() {
                        editor.cycle_option(true);
                    }
                }
                KeyCode::Left => {
                    if editor.is_select_field() {
                        editor.cycle_option(false);
                    }
                }
                KeyCode::Char(c) => {
                    if !editor.is_select_field() {
                        let val = editor.current_value().to_string() + &c.to_string();
                        editor.set_current_value(val);
                    }
                }
                KeyCode::Backspace => {
                    if !editor.is_select_field() {
                        let mut val = editor.current_value().to_string();
                        val.pop();
                        editor.set_current_value(val);
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_help(&mut self, _key: KeyEvent) {
        self.mode = Mode::Normal;
    }

    fn handle_password_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if let Some(ref hash) = self.config.master_password_hash {
                    match crypto::verify_master_password(&self.password_input, hash) {
                        Ok(true) => {
                            if let Ok(key) =
                                crypto::derive_key(&self.password_input, &self.vault_salt_bytes())
                            {
                                self.encryption_key = Some(key);
                                if let Ok(vault) = storage::load_vault(&key) {
                                    self.passwords = vault;
                                }
                                self.mode = Mode::Normal;
                                self.password_input.clear();
                                self.password_error.clear();
                                self.clamp_selection();
                            }
                        }
                        _ => {
                            self.password_error = i18n::tr("password.wrong");
                            self.password_input.clear();
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('r') => {
                self.mode = Mode::ConfirmReset;
            }
            KeyCode::Char(c) => {
                self.password_input.push(c);
                self.password_error.clear();
            }
            KeyCode::Backspace => {
                self.password_input.pop();
                self.password_error.clear();
            }
            _ => {}
        }
    }

    fn handle_set_master_password(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if self.password_input.len() < 6 {
                    self.password_error = i18n::tr("password.min_length");
                    return;
                }
                self.first_password = self.password_input.clone();
                self.password_input.clear();
                self.mode = Mode::SetMasterPasswordConfirm;
            }
            KeyCode::Esc => {
                self.mode = Mode::ConfirmSkipPassword;
            }
            KeyCode::Char(c) => {
                self.password_input.push(c);
                self.password_error.clear();
            }
            KeyCode::Backspace => {
                self.password_input.pop();
                self.password_error.clear();
            }
            _ => {}
        }
    }

    fn handle_set_master_password_confirm(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if self.password_input == self.first_password {
                    match crypto::hash_master_password(&self.first_password) {
                        Ok(hash) => {
                            self.config.master_password_hash = Some(hash);
                            if self.config.vault_salt.is_none() {
                                self.config.vault_salt = Some(crypto::generate_salt());
                            }
                            if let Ok(key) =
                                crypto::derive_key(&self.first_password, &self.vault_salt_bytes())
                            {
                                self.encryption_key = Some(key);
                                let _ = storage::save_config(&self.config);
                            }
                            self.first_password.clear();
                            self.password_input.clear();
                            self.mode = Mode::Normal;
                        }
                        Err(_) => {
                            self.password_error = i18n::tr("password.encrypt_failed");
                        }
                    }
                } else {
                    self.password_error = i18n::tr("password.mismatch");
                    self.password_input.clear();
                    self.mode = Mode::SetMasterPassword;
                }
            }
            KeyCode::Esc => {
                self.mode = Mode::SetMasterPassword;
                self.password_input.clear();
            }
            KeyCode::Char(c) => {
                self.password_input.push(c);
                self.password_error.clear();
            }
            KeyCode::Backspace => {
                self.password_input.pop();
                self.password_error.clear();
            }
            _ => {}
        }
    }

    fn handle_confirm_delete(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                if let Some(idx) = self.selected_index
                    && let Some(SidebarItem::Host { .. }) = self.sidebar_items.get(idx) {
                        self.delete_selected_host();
                    }
                self.mode = Mode::Normal;
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            _ => {}
        }
    }

    fn handle_confirm_delete_group(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if let Some(gi) = self.pending_delete_group_index.take() {
                    self.delete_group_with_hosts(gi);
                }
                self.mode = Mode::Normal;
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if let Some(gi) = self.pending_delete_group_index.take() {
                    self.delete_group(gi);
                }
                self.mode = Mode::Normal;
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.pending_delete_group_index = None;
                self.mode = Mode::Normal;
            }
            _ => {}
        }
    }

    fn handle_confirm_reset(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                storage::reset_all();
                self.config = Config::default();
                self.passwords.clear();
                self.encryption_key = None;
                self.mode = Mode::SetMasterPassword;
                self.password_input.clear();
                self.password_error.clear();
                self.first_password.clear();
                self.selected_index = None;
                self.rebuild_sidebar();
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.mode = Mode::PasswordInput;
            }
            _ => {}
        }
    }

    fn handle_confirm_skip_password(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.mode = Mode::Normal;
                self.password_input.clear();
                self.password_error.clear();
                let _ = storage::save_config(&self.config);
                self.clamp_selection();
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.mode = Mode::SetMasterPassword;
            }
            _ => {}
        }
    }

    fn handle_theme_selector(&mut self, key: KeyEvent) {
        let names = theme::theme_names();
        match key.code {
            KeyCode::Esc => {
                self.config.settings.theme = self.theme_before_selector.clone();
                self.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                let _ = storage::save_config(&self.config);
                self.mode = Mode::Normal;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.theme_selector_index > 0 {
                    self.theme_selector_index -= 1;
                } else {
                    self.theme_selector_index = names.len() - 1;
                }
                self.config.settings.theme = names[self.theme_selector_index].to_string();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.theme_selector_index = (self.theme_selector_index + 1) % names.len();
                self.config.settings.theme = names[self.theme_selector_index].to_string();
            }
            _ => {}
        }
    }

    fn enter_theme_selector(&mut self) {
        let names = theme::theme_names();
        self.theme_selector_index = names
            .iter()
            .position(|&n| n == self.config.settings.theme)
            .unwrap_or(0);
        self.theme_before_selector = self.config.settings.theme.clone();
        self.mode = Mode::ThemeSelector;
    }

    fn handle_lang_selector(&mut self, key: KeyEvent) {
        let langs = i18n::all_langs();
        match key.code {
            KeyCode::Esc => {
                // revert language
                let code = self.config.settings.lang.clone();
                i18n::set_lang(i18n::Lang::from_str(&code));
                self.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                let _ = storage::save_config(&self.config);
                self.mode = Mode::Normal;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.lang_selector_index > 0 {
                    self.lang_selector_index -= 1;
                } else {
                    self.lang_selector_index = langs.len() - 1;
                }
                let (lang, _) = langs[self.lang_selector_index];
                self.config.settings.lang = i18n::lang_code(lang).to_string();
                i18n::set_lang(lang);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.lang_selector_index = (self.lang_selector_index + 1) % langs.len();
                let (lang, _) = langs[self.lang_selector_index];
                self.config.settings.lang = i18n::lang_code(lang).to_string();
                i18n::set_lang(lang);
            }
            _ => {}
        }
    }

    fn enter_lang_selector(&mut self) {
        let langs = i18n::all_langs();
        let current = i18n::current_lang();
        self.lang_selector_index = langs
            .iter()
            .position(|(l, _)| *l == current)
            .unwrap_or(0);
        self.mode = Mode::LangSelector;
    }

    pub(super) fn move_selection(&mut self, delta: i32) {
        if self.sidebar_items.is_empty() {
            return;
        }
        let current = self.selected_index.unwrap_or(0) as i32;
        let new = (current + delta).clamp(0, self.sidebar_items.len() as i32 - 1);
        self.selected_index = Some(new as usize);
    }

    pub(super) fn swap_items_sort_order(&mut self, idx_a: usize, idx_b: usize) {
        let item_a = self.sidebar_items.get(idx_a).cloned();
        let item_b = self.sidebar_items.get(idx_b).cloned();
        let (Some(a), Some(b)) = (item_a, item_b) else { return };

        self.renumber_sort_orders();

        let order_a = match &a {
            SidebarItem::Group { index } => self.config.groups[*index].sort_order,
            SidebarItem::Host { index, .. } => self.config.hosts[*index].sort_order,
        };
        let order_b = match &b {
            SidebarItem::Group { index } => self.config.groups[*index].sort_order,
            SidebarItem::Host { index, .. } => self.config.hosts[*index].sort_order,
        };

        match &a {
            SidebarItem::Group { index } => self.config.groups[*index].sort_order = order_b,
            SidebarItem::Host { index, .. } => self.config.hosts[*index].sort_order = order_b,
        }
        match &b {
            SidebarItem::Group { index } => self.config.groups[*index].sort_order = order_a,
            SidebarItem::Host { index, .. } => self.config.hosts[*index].sort_order = order_a,
        }

        let _ = storage::save_config(&self.config);
        self.rebuild_sidebar();
    }

    pub(super) fn renumber_sort_orders(&mut self) {
        let mut level0_pos = 0u32;
        let mut group_host_pos: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();

        for item in &self.sidebar_items {
            match item {
                SidebarItem::Group { index } => {
                    self.config.groups[*index].sort_order = level0_pos * 10;
                    level0_pos += 1;
                    group_host_pos.insert(*index, 0);
                }
                SidebarItem::Host { index, group_index: None } => {
                    self.config.hosts[*index].sort_order = level0_pos * 10;
                    level0_pos += 1;
                }
                SidebarItem::Host { index, group_index: Some(gi) } => {
                    let pos = group_host_pos.entry(*gi).or_insert(0);
                    self.config.hosts[*index].sort_order = *pos * 10;
                    *pos += 1;
                }
            }
        }
    }

    pub(super) fn move_selected_forward(&mut self) {
        let Some(idx) = self.selected_index else { return };
        let Some(item) = self.sidebar_items.get(idx) else { return };
        let Some(next) = self.sidebar_items.get(idx + 1) else { return };

        if is_same_level(item, next) {
            self.swap_items_sort_order(idx, idx + 1);
            self.move_selection(1);
        }
    }

    pub(super) fn move_selected_backward(&mut self) {
        let Some(idx) = self.selected_index else { return };
        if idx == 0 { return; }
        let Some(item) = self.sidebar_items.get(idx) else { return };
        let Some(prev) = self.sidebar_items.get(idx - 1) else { return };

        if is_same_level(item, prev) {
            self.swap_items_sort_order(idx, idx - 1);
            self.move_selection(-1);
        }
    }

    pub(super) fn save_editor(&mut self) {
        if let Some(editor) = self.editor_state.take() {
            let host = editor.into_host();
            let host_id = host.id.clone();
            let plain_password = host.password.clone();

            let existing = self.config.hosts.iter().position(|h| h.id == host.id);
            if let Some(idx) = existing {
                self.config.hosts[idx] = host;
            } else {
                self.config.hosts.push(host);
            }

            // save password into vault
            if let Some(ref pwd) = plain_password
                && !pwd.is_empty() {
                    self.passwords.insert(host_id.clone(), pwd.clone());
                    self.save_passwords();
                }

            let _ = storage::save_config(&self.config);
            self.rebuild_sidebar();
            if self.selected_index.is_none() && !self.sidebar_items.is_empty() {
                self.selected_index = Some(0);
            }
            self.focus = Focus::Sidebar;
        }
        self.mode = Mode::Normal;
    }

    pub(super) fn delete_selected_host(&mut self) {
        if let Some(idx) = self.selected_index
            && let Some(SidebarItem::Host { index, .. }) = self.sidebar_items.get(idx) {
                let host_id = self.config.hosts[*index].id.clone();
                self.config.hosts.remove(*index);
                self.passwords.remove(&host_id);
                self.save_passwords();
                let _ = storage::save_config(&self.config);
                self.rebuild_sidebar();
                if self.sidebar_items.is_empty() {
                    self.selected_index = None;
                } else {
                    self.selected_index =
                        Some(idx.min(self.sidebar_items.len() - 1));
                }
            }
    }

    pub(super) fn save_passwords(&mut self) {
        if let Some(ref key) = self.encryption_key {
            let _ = storage::save_vault(&self.passwords, key);
        } else {
            let _ = storage::save_plain_passwords(&self.passwords);
        }
    }

    fn delete_group(&mut self, group_index: usize) {
        if let Some(group) = self.config.groups.get(group_index).cloned() {
            for host in &mut self.config.hosts {
                if host.group_id.as_deref() == Some(&group.id) {
                    host.group_id = None;
                }
            }
            self.config.groups.remove(group_index);
            self.expanded_groups.0.retain(|id| id != &group.id);
            let _ = storage::save_config(&self.config);
            self.rebuild_sidebar();
            self.clamp_selection();
        }
    }

    fn delete_group_with_hosts(&mut self, group_index: usize) {
        if let Some(group) = self.config.groups.get(group_index).cloned() {
            self.config.hosts.retain(|h| h.group_id.as_deref() != Some(&group.id));
            self.config.groups.remove(group_index);
            self.expanded_groups.0.retain(|id| id != &group.id);
            let _ = storage::save_config(&self.config);
            self.rebuild_sidebar();
            self.clamp_selection();
        }
    }

    fn handle_create_group(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if self.group_name_input.trim().is_empty() {
                    self.password_error = i18n::tr("group.name_empty");
                    return;
                }
                let group = crate::config::model::Group::new(self.group_name_input.trim().to_string());
                self.config.groups.push(group);
                let _ = storage::save_config(&self.config);
                self.rebuild_sidebar();
                self.clamp_selection();
                self.group_name_input.clear();
                self.password_error.clear();
                self.mode = Mode::Normal;
            }
            KeyCode::Esc => {
                self.group_name_input.clear();
                self.password_error.clear();
                self.mode = Mode::Normal;
            }
            KeyCode::Char(c) => {
                self.group_name_input.push(c);
                self.password_error.clear();
            }
            KeyCode::Backspace => {
                self.group_name_input.pop();
                self.password_error.clear();
            }
            _ => {}
        }
    }
}
