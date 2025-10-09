use color_eyre::Result;
use std::time::{Duration, Instant};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::ui::draw;

#[derive(Default)]
pub struct App {
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub selected_line: usize,
    pub total_lines: usize,
    pub visible_height: usize, // <-- add this
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| draw(&mut self, frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),

                        KeyCode::Char('j') | KeyCode::Down => {
                            if self.total_lines > 0 {
                                let prev_line = self.selected_line;
                                self.selected_line = (self.selected_line + 1) % self.total_lines;

                                // if wrapped to top (e.g., 34 -> 0)
                                if self.selected_line == 0 && prev_line == self.total_lines - 1 {
                                    self.vertical_scroll = 0;
                                } else if self.selected_line
                                    >= self.vertical_scroll + self.visible_height
                                {
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
                                        self.vertical_scroll =
                                            self.vertical_scroll.saturating_sub(1);
                                    }
                                }

                                // if wrapped from top → bottom, force scroll to bottom
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
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }
}
