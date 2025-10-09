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
    pub selected_tab: usize, // 0 or 1

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
                    match self.selected_tab {
                        0 => self.key_handler_directories(key),
                        1 => self.key_handler_tmux(key),
                        _ => {}
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

            KeyCode::Char('2') => self.selected_tab = 1,

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

            KeyCode::Char('h') | KeyCode::Left => {
                // reset a few things
                if let Some(parent) = self.current_directory.parent() {
                    self.current_directory = parent.to_path_buf();
                    self.current_directory_contents =
                        get_current_directory_contents(self.current_directory.as_path());
                    self.directories_selected_line = 0;
                    self.directories_scroll = 0;
                }
            }

            KeyCode::Char('l') | KeyCode::Right => {
                let selected_path =
                    &self.current_directory_contents[self.directories_selected_line];
                if selected_path.is_dir() {
                    self.current_directory
                        .push(selected_path.file_name().unwrap());
                    self.current_directory_contents =
                        get_current_directory_contents(&self.current_directory);
                    self.directories_selected_line = 0;
                    self.directories_scroll = 0;
                }
            }

            KeyCode::Enter => {
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

            KeyCode::Enter => {
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

    fn initalize_state(&mut self) {
        self.current_directory = get_current_directory_name();
        self.current_directory_contents =
            get_current_directory_contents(self.current_directory.as_path());
    }
}
