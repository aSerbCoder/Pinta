pub mod render;
pub mod app {
    use std::io;

    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::{
        DefaultTerminal, Frame,
        layout::{Constraint, Direction, Layout},
        style::{Color, Style, Stylize},
        text::{Line, Span, Text},
        widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    };

    use crate::directory::directory::Directory;

    pub struct App {
        pub exit: bool,
        pub directory: Directory,
        pub vertical_scroll: usize,
        pub vertical_scroll_state: ScrollbarState,
        pub selected: usize,
        pub show_hidden: bool,
    } // App{}

    impl App {
        pub fn new() -> Self {
            App {
                exit: false,
                directory: Directory::new(),
                vertical_scroll: 0,
                vertical_scroll_state: ScrollbarState::default(),
                selected: 0,
                show_hidden: false,
            }
        }

        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
            while !self.exit {
                terminal.draw(|frame| self.draw(frame))?;
                self.handle_events()?;
            }
            Ok(())
        }

        fn list_dir_contents(&self, frame: &mut Frame) {
            let full_area = frame.area();

            // Step 1: Vertical layout — more space for the middle chunk (scroll area)
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(15), // Less space above
                    Constraint::Percentage(70), // More space for scrollable area
                    Constraint::Percentage(15), // Less space below
                ])
                .split(full_area);

            let middle_area = vertical_chunks[1];

            // Step 2: Horizontal layout — make the scrollable area wider
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20), // Narrow left margin
                    Constraint::Percentage(60), // Wider scroll area
                    Constraint::Percentage(20), // Narrow right margin
                ])
                .split(middle_area);

            let content_area = horizontal_chunks[1];
            let cur_dir = self.directory.cur_dir.to_string_lossy();

            let title = Line::from(vec![
                Span::raw(" "),                                         // Leading space
                Span::styled(cur_dir, Style::default().bold().white()), // Styled path
                Span::raw(" "),                                         // Trailing space
            ]);

            // Block with THICK border and blue background
            let block = Block::bordered()
                .border_type(BorderType::Thick)
                .style(Style::default().bg(Color::DarkGray))
                .title(title.bold().white());
            // Build content
            let dir_contents: Vec<Line> = self
                .directory
                .dir_contents
                .iter()
                .enumerate()
                .filter_map(|(i, entry)| {
                    let entry_str = entry.file_name().to_string_lossy().to_string();

                    // If show_hidden is false and the entry is a hidden file, skip it
                    if !self.show_hidden && entry_str.starts_with('.') {
                        return None; // Skip this entry
                    }

                    // Create the line for this entry
                    let line = if i == self.selected {
                        // Selected line: different background color
                        Line::from(Span::styled(
                            entry_str,
                            Style::default().bg(Color::White).fg(Color::Green).bold(),
                        ))
                    } else {
                        // Normal line
                        Line::from(entry_str)
                    };

                    Some(line) // Return the line wrapped in Some
                })
                .collect();

            // Paragraph widget
            let paragraph = Paragraph::new(Text::from(dir_contents))
                .block(block)
                .scroll((self.vertical_scroll as u16, 0))
                .style(Style::default().bg(Color::DarkGray).fg(Color::White));

            // Render paragraph
            frame.render_widget(paragraph, content_area);

            // Scrollbar
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓")),
                content_area,
                &mut self.vertical_scroll_state.clone(),
            );
        }

        fn draw(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.area());
            self.list_dir_contents(frame);
        }

        fn handle_events(&mut self) -> io::Result<()> {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
            Ok(())
        }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('j') => {
                    if self.selected < self.directory.dir_contents.len() - 1 {
                        self.selected += 1;
                    } else {
                        self.selected = 0;
                    }
                }
                KeyCode::Char('k') => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = self.directory.dir_contents.len() - 1;
                    }
                }
                KeyCode::Char('h') => {
                    self.selected = 0;
                    self.directory.prev_directory();
                }
                KeyCode::Char('a') => {
                    self.show_hidden = !self.show_hidden;
                }
                KeyCode::Char('l') => {
                    let selected = self.selected;
                    self.selected = 0;
                    self.directory
                        .next_directory(self.directory.dir_contents[selected].path().as_ref());
                }
                _ => {}
            }
        }

        fn exit(&mut self) {
            self.exit = true;
        }
    }
}
