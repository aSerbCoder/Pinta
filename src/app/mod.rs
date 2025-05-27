pub mod handler;
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub struct App {
    pub exit: bool,
} // App{}

impl App {
    pub fn new() -> Self {
        App { exit: false }
    } // App::new()

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self::handler::handle_events(self)?;
        }
        Ok(())
    } // App::run()

    fn draw(&self, _frame: &mut Frame) {
        todo!()
    }

    fn exit(&mut self) {
        self.exit = true;
    } // App::exit()
}
