use color_eyre::Result;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEvent},
        terminal,
    },
    widgets::ScrollbarState,
};

use crate::{
    commands::tmux::{TmuxSession, enter_in_tmux, list_tmux_sessions, reenter_tmux_session},
    paths::directories::{get_current_directory_contents, get_current_directory_name},
    ui::draw,
};

#[derive(Default)]
pub struct App {
    pub show_help: bool,
    pub selected_tab: usize, // 0 or 1

    pub show_hidden: bool,

    pub search_matches: Vec<usize>,
    pub search_match_index: Option<usize>,

    pub searched_string: String,
    pub searching: bool,

    pub last_search_update: Option<Instant>,
    pub help_selected_tab: usize,

    // help scroller
    pub help_selected_line: usize,
    pub help_scroll: usize,
    pub help_total_lines: usize,
    pub help_visible_height: usize,
    pub help_scroll_state: ScrollbarState,

    // Tab 0 scroller
    pub directories_selected_line: usize,
    pub directories_scroll: usize,
    pub directories_total_lines: usize,
    pub directories_visible_height: usize,
    pub directories_scroll_state: ScrollbarState,

    // Tab 1 scroller
    pub tmux_selected_line: usize,
    pub tmux_scroll: usize,
    pub tmux_total_lines: usize,
    pub tmux_visible_height: usize,
    pub tmux_scroll_state: ScrollbarState,

    pub current_directory: PathBuf,
    pub current_directory_contents: Vec<PathBuf>,
    pub tmux_sessions: Vec<TmuxSession>,
    exit: bool,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        self.tmux_sessions = list_tmux_sessions();

        self.initalize_state();

        loop {
            terminal.draw(|frame| draw(&mut self, frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if self.show_help {
                        self.key_handler_help(key);
                    } else if self.searching {
                        self.key_handler_searching(key);
                    } else {
                        match self.selected_tab {
                            0 => self.key_handler_directories(key),
                            1 => self.key_handler_tmux(key),
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            if self.exit {
                return Ok(());
            }
        }
    }

    fn key_handler_directories(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Char('H') => {
                self.show_help = !self.show_help;
            }

            KeyCode::Char('2') => self.selected_tab = 1,

            KeyCode::Char('/') => {
                self.searching = true;
                self.searched_string.clear();
                self.search_matches.clear();
                self.search_match_index = None;
                self.last_search_update = Some(Instant::now());
                self.update_search_results();
            }

            KeyCode::Char('j') | KeyCode::Down => {
                if self.directories_total_lines > 0 {
                    let prev_line = self.directories_selected_line;
                    self.directories_selected_line =
                        (self.directories_selected_line + 1) % self.directories_total_lines;

                    if self.directories_selected_line == 0
                        && prev_line == self.directories_total_lines - 1
                    {
                        self.directories_scroll = 0;
                    } else if self.directories_selected_line
                        >= self.directories_scroll + self.directories_visible_height
                    {
                        self.directories_scroll = self.directories_scroll.saturating_add(1);
                    }
                }
            }

            KeyCode::Char('k') | KeyCode::Up => {
                if self.directories_total_lines > 0 {
                    let prev_line = self.directories_selected_line;
                    if self.directories_selected_line == 0 {
                        self.directories_selected_line = self.directories_total_lines - 1;
                        self.directories_scroll = self
                            .directories_total_lines
                            .saturating_sub(self.directories_visible_height)
                            .min(self.directories_total_lines.saturating_sub(1));
                    } else {
                        self.directories_selected_line -= 1;
                        if self.directories_selected_line < self.directories_scroll {
                            self.directories_scroll = self.directories_scroll.saturating_sub(1);
                        }
                    }

                    if prev_line == 0
                        && self.directories_selected_line == self.directories_total_lines - 1
                    {
                        self.directories_scroll = self
                            .directories_total_lines
                            .saturating_sub(self.directories_visible_height)
                            .min(self.directories_total_lines.saturating_sub(1));
                    }
                }
            }

            KeyCode::Char('n') => {
                if !self.search_matches.is_empty() {
                    self.jump_to_next_match();
                }
            }

            KeyCode::Char('A') => {
                self.show_hidden = !self.show_hidden;
                self.search_matches = Vec::new();
                self.search_match_index = None;
                self.searched_string = String::new();
                self.current_directory_contents = get_current_directory_contents(
                    self.current_directory.as_path(),
                    self.show_hidden,
                );
                self.directories_selected_line = 0;
                self.directories_scroll = 0;
            }

            KeyCode::Char('N') => {
                if !self.search_matches.is_empty() {
                    self.jump_to_prev_match();
                }
            }

            KeyCode::Char('h') | KeyCode::Left => {
                if let Some(parent) = self.current_directory.parent() {
                    self.search_matches = Vec::new();
                    self.search_match_index = None;
                    self.searched_string = String::new();
                    self.current_directory = parent.to_path_buf();
                    self.current_directory_contents = get_current_directory_contents(
                        self.current_directory.as_path(),
                        self.show_hidden,
                    );
                    self.directories_selected_line = 0;
                    self.directories_scroll = 0;
                }
            }

            KeyCode::Char('l') | KeyCode::Right => {
                let selected_path =
                    &self.current_directory_contents[self.directories_selected_line];
                if selected_path.is_dir() {
                    self.search_matches = Vec::new();
                    self.search_match_index = None;
                    self.searched_string = String::new();

                    self.current_directory
                        .push(selected_path.file_name().unwrap());
                    self.current_directory_contents =
                        get_current_directory_contents(&self.current_directory, self.show_hidden);
                    self.directories_selected_line = 0;
                    self.directories_scroll = 0;
                }
            }

            KeyCode::Char('t') => {
                terminal::disable_raw_mode().expect("Could not disable raw mode");
                crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)
                    .expect("Could not leave alternate screen");

                let selected_path = &self.current_directory;

                if selected_path.is_dir() {
                    enter_in_tmux(selected_path);
                }

                self.exit = true;
            }

            _ => {}
        }
    }

    fn key_handler_tmux(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Char('H') => {
                self.show_help = !self.show_help;
            }

            KeyCode::Char('1') => self.selected_tab = 0,

            KeyCode::Char('j') | KeyCode::Down => {
                if !self.tmux_sessions.is_empty() {
                    self.tmux_selected_line =
                        (self.tmux_selected_line + 1) % self.tmux_sessions.len();

                    let mut line_index = 0;
                    for (i, session) in self.tmux_sessions.iter().enumerate() {
                        if i == self.tmux_selected_line {
                            break;
                        }
                        line_index += 1 + session.windows.len() + 1;
                    }

                    let session_height =
                        1 + self.tmux_sessions[self.tmux_selected_line].windows.len() + 1;

                    if line_index < self.tmux_scroll {
                        self.tmux_scroll = line_index;
                    } else if line_index + session_height
                        > self.tmux_scroll + self.tmux_visible_height
                    {
                        self.tmux_scroll = line_index + session_height - self.tmux_visible_height;
                    }
                }
            }

            KeyCode::Char('k') | KeyCode::Up => {
                if !self.tmux_sessions.is_empty() {
                    if self.tmux_selected_line == 0 {
                        self.tmux_selected_line = self.tmux_sessions.len() - 1;
                    } else {
                        self.tmux_selected_line -= 1;
                    }

                    let mut line_index = 0;
                    for (i, session) in self.tmux_sessions.iter().enumerate() {
                        if i == self.tmux_selected_line {
                            break;
                        }
                        line_index += 1 + session.windows.len() + 1;
                    }

                    let session_height =
                        1 + self.tmux_sessions[self.tmux_selected_line].windows.len() + 1;

                    if line_index < self.tmux_scroll {
                        self.tmux_scroll = line_index;
                    } else if line_index + session_height
                        > self.tmux_scroll + self.tmux_visible_height
                    {
                        self.tmux_scroll = line_index + session_height - self.tmux_visible_height;
                    }
                }
            }

            KeyCode::Char('t') => {
                if !self.tmux_sessions.is_empty() {
                    terminal::disable_raw_mode().expect("Could not disable raw mode");
                    crossterm::execute!(
                        std::io::stdout(),
                        crossterm::terminal::LeaveAlternateScreen
                    )
                    .expect("Could not leave alternate screen");

                    if let Some(session) = self.tmux_sessions.get(self.tmux_selected_line) {
                        reenter_tmux_session(&session.name);
                    }

                    self.exit = true;
                }
            }

            _ => {}
        }
    }

    fn key_handler_help(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('H') => {
                self.show_help = !self.show_help;
                return;
            }

            KeyCode::Char('j') | KeyCode::Down => {
                if self.help_total_lines > 0 {
                    let prev_line = self.help_selected_line;
                    self.help_selected_line = (self.help_selected_line + 1) % self.help_total_lines;

                    if self.help_selected_line == 0 && prev_line == self.help_total_lines - 1 {
                        self.help_scroll = 0;
                    } else if self.help_selected_line >= self.help_scroll + self.help_visible_height
                    {
                        self.help_scroll = self.help_scroll.saturating_add(1);
                    }
                }
            }

            KeyCode::Char('k') | KeyCode::Up => {
                if self.help_total_lines > 0 {
                    let prev_line = self.help_selected_line;
                    if self.help_selected_line == 0 {
                        self.help_selected_line = self.help_total_lines - 1;
                        self.help_scroll = self
                            .help_total_lines
                            .saturating_sub(self.help_visible_height)
                            .min(self.help_total_lines.saturating_sub(1));
                    } else {
                        self.help_selected_line -= 1;
                        if self.help_selected_line < self.help_scroll {
                            self.help_scroll = self.help_scroll.saturating_sub(1);
                        }
                    }

                    if prev_line == 0 && self.help_selected_line == self.help_total_lines - 1 {
                        self.help_scroll = self
                            .help_total_lines
                            .saturating_sub(self.help_visible_height)
                            .min(self.help_total_lines.saturating_sub(1));
                    }
                }
            }

            KeyCode::Char('1') => self.help_selected_tab = 0,
            KeyCode::Char('2') => self.help_selected_tab = 1,
            KeyCode::Char('3') => self.help_selected_tab = 2,
            KeyCode::Char('4') => self.help_selected_tab = 3,

            KeyCode::Char('h') => {
                if self.help_selected_tab == 0 {
                    self.help_selected_tab = 3;
                } else {
                    self.help_selected_tab -= 1;
                }
            }
            KeyCode::Char('l') => {
                if self.help_selected_tab == 3 {
                    self.help_selected_tab = 0;
                } else {
                    self.help_selected_tab += 1;
                }
            }

            _ => {}
        }
    }

    fn key_handler_searching(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.searching = false;
                self.searched_string.clear();
                self.search_matches.clear();
                self.search_match_index = None;
            }

            KeyCode::Char(c) => {
                if !c.is_control() && !c.is_whitespace() {
                    self.searched_string.push(c);

                    self.last_search_update = Some(Instant::now());
                    self.update_search_results();
                }
            }

            KeyCode::Backspace => {
                self.searched_string.pop();
                self.last_search_update = Some(Instant::now());
                self.update_search_results();
            }

            KeyCode::Enter => {
                self.last_search_update = Some(Instant::now());
                self.searching = false;
            }

            _ => {}
        }
    }

    fn update_search_results(&mut self) {
        self.search_matches.clear();
        self.search_match_index = None;

        if self.searched_string.is_empty() {
            return;
        }

        let query = self.searched_string.to_lowercase();

        for (i, path) in self.current_directory_contents.iter().enumerate() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.to_lowercase().contains(&query) {
                    self.search_matches.push(i);
                }
            }
        }

        if !self.search_matches.is_empty() {
            self.search_match_index = Some(0);
            self.directories_selected_line = self.search_matches[0];
            self.ensure_selected_visible();
        }
    }

    fn initalize_state(&mut self) {
        self.current_directory = get_current_directory_name();
        self.current_directory_contents =
            get_current_directory_contents(self.current_directory.as_path(), self.show_hidden);
    }

    fn jump_to_next_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }

        let next_index = match self.search_match_index {
            Some(i) => (i + 1) % self.search_matches.len(),
            None => 0,
        };

        self.search_match_index = Some(next_index);
        self.directories_selected_line = self.search_matches[next_index];
        self.ensure_selected_visible();
    }

    fn jump_to_prev_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }

        let prev_index = match self.search_match_index {
            Some(0) | None => self.search_matches.len() - 1,
            Some(i) => i - 1,
        };

        self.search_match_index = Some(prev_index);
        self.directories_selected_line = self.search_matches[prev_index];
        self.ensure_selected_visible();
    }

    fn ensure_selected_visible(&mut self) {
        if self.directories_selected_line < self.directories_scroll {
            self.directories_scroll = self.directories_selected_line;
        } else if self.directories_selected_line
            >= self.directories_scroll + self.directories_visible_height
        {
            self.directories_scroll = self
                .directories_selected_line
                .saturating_sub(self.directories_visible_height - 1);
        }
    }
}
