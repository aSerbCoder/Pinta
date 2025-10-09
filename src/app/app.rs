use color_eyre::Result;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    widgets::ScrollbarState,
};

use crate::{
    paths::directories::{get_current_directory_contents, get_current_directory_name},
    ui::draw,
};

#[derive(Default)]
pub struct App {
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub selected_line: usize,
    pub total_lines: usize,
    pub visible_height: usize,
    pub current_directory: PathBuf,
    pub current_directory_contents: Vec<PathBuf>,
    exit: bool,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        self.initalize_state();

        loop {
            terminal.draw(|frame| draw(&mut self, frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.key_handler(key);
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

    fn key_handler(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }

            KeyCode::Char('j') | KeyCode::Down => {
                if self.total_lines > 0 {
                    let prev_line = self.selected_line;
                    self.selected_line = (self.selected_line + 1) % self.total_lines;

                    if self.selected_line == 0 && prev_line == self.total_lines - 1 {
                        self.vertical_scroll = 0;
                    } else if self.selected_line >= self.vertical_scroll + self.visible_height {
                        // normal scroll down
                        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                    }
                }
            }

            KeyCode::Char('k') | KeyCode::Up => {
                if self.total_lines > 0 {
                    let prev_line = self.selected_line;
                    if self.selected_line == 0 {
                        // wrap to bottom
                        self.selected_line = self.total_lines - 1;
                        self.vertical_scroll = self
                            .total_lines
                            .saturating_sub(self.visible_height)
                            .min(self.total_lines.saturating_sub(1));
                    } else {
                        self.selected_line -= 1;
                        if self.selected_line < self.vertical_scroll {
                            self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                        }
                    }

                    if prev_line == 0 && self.selected_line == self.total_lines - 1 {
                        self.vertical_scroll = self
                            .total_lines
                            .saturating_sub(self.visible_height)
                            .min(self.total_lines.saturating_sub(1));
                    }
                }
            }

            _ => {}
        }
    }

    fn initalize_state(&mut self) {
        self.current_directory = get_current_directory_name();
        self.current_directory_contents = get_current_directory_contents()
    }
}
