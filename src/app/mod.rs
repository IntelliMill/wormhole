mod drawing;
mod handlers;

use std::collections::HashMap;
use std::io;

use crossterm::event::{self, Event};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Terminal;

use crate::config::model::Config;
use crate::config::storage;
use crate::i18n;
use crate::theme;
use crate::ui::detail::{self, DetailSelection, DetailTab};
use crate::ui::editor::{self, EditorState};
use crate::ui::help;
use crate::ui::icons;
use crate::ui::layout as ui_layout;
use crate::ui::sidebar::{self, ExpandedGroups, SidebarContext, SidebarItem};
use crate::ui::theme_selector;

const BANNER_LINES: &[&str] = &[
    " _       __                     __          __   ",
    "| |     / /___  _________ ___  / /_  ____  / /__ ",
    r#"| | /| / / __ \/ ___/ __ `__ \/ __ \/ __ \/ / _ \"#,
    "| |/ |/ / /_/ / /  / / / / / / / / / /_/ / /  __/",
    r#"|__/|__/\____/_/  /_/ /_/ /_/_/ /_/\____/_/\___/"#,
    "                                                 ",
];

#[derive(Debug, Clone, PartialEq)]
enum Focus {
    Sidebar,
    Detail,
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Normal,
    Search,
    Editing,
    ConfirmDelete,
    ConfirmDeleteGroup,
    ConfirmReset,
    CreateGroup,
    ThemeSelector,
    LangSelector,
    Help,
    PasswordInput,
    SetMasterPassword,
    SetMasterPasswordConfirm,
    ConfirmSkipPassword,
}

/// Top-level application state driving the TUI event loop.
pub struct App {
    config: Config,
    passwords: HashMap<String, String>,
    encryption_key: Option<[u8; 32]>,
    mode: Mode,
    focus: Focus,
    sidebar_items: Vec<SidebarItem>,
    expanded_groups: ExpandedGroups,
    selected_index: Option<usize>,
    search_query: String,
    editor_state: Option<EditorState>,
    confirm_message: String,
    password_input: String,
    password_error: String,
    first_password: String,
    group_name_input: String,
    pending_delete_group_index: Option<usize>,
    should_quit: bool,
    should_connect: Option<usize>,
    detail_tab: DetailTab,
    theme_selector_index: usize,
    theme_before_selector: String,
    lang_selector_index: usize,
}

impl App {
    /// Initialize app state, loading persisted config and setting the UI language.
    pub fn new() -> Self {
        let config = storage::load_config().unwrap_or_default();
        let lang = i18n::Lang::from_str(&config.settings.lang);
        i18n::set_lang(lang);

        let expanded = ExpandedGroups::new();
        let items = sidebar::build_sidebar_items(&config, &expanded);
        let saved_theme = config.settings.theme.clone();

        Self {
            config,
            passwords: HashMap::new(),
            encryption_key: None,
            mode: Mode::Normal,
            focus: Focus::Sidebar,
            sidebar_items: items,
            expanded_groups: expanded,
            selected_index: None,
            search_query: String::new(),
            editor_state: None,
            confirm_message: String::new(),
            password_input: String::new(),
            password_error: String::new(),
            first_password: String::new(),
            group_name_input: String::new(),
            pending_delete_group_index: None,
            should_quit: false,
            should_connect: None,
            detail_tab: DetailTab::Overview,
            theme_selector_index: 0,
            theme_before_selector: saved_theme,
            lang_selector_index: 0,
        }
    }

    /// Enter the terminal UI event loop. Restores the terminal on exit or SSH connect.
    pub fn run(&mut self) -> io::Result<()> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        if storage::is_first_run() {
            self.mode = Mode::SetMasterPassword;
        } else if self.config.master_password_hash.is_some() {
            self.mode = Mode::PasswordInput;
        } else {
            // no master password — load plain text passwords
            if let Ok(pwds) = storage::load_plain_passwords() {
                self.passwords = pwds;
            }
            self.clamp_selection();
        }

        loop {
            self.draw(&mut terminal)?;

            if self.should_quit {
                break;
            }

            if let Some(host_idx) = self.should_connect.take() {
                self.restore_terminal(&mut terminal)?;
                self.connect_host(host_idx);
                break;
            }

            if event::poll(std::time::Duration::from_millis(100))?
                && let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                }
        }

        self.restore_terminal(&mut terminal)?;
        Ok(())
    }

    fn restore_terminal(
        &self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::event::DisableMouseCapture,
            crossterm::terminal::LeaveAlternateScreen
        )?;
        crossterm::terminal::disable_raw_mode()?;
        terminal.show_cursor()?;
        Ok(())
    }
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
