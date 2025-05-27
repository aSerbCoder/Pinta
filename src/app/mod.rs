pub mod handler;
use ratatui::{DefaultTerminal, Frame};
use std::io;

/// Stores Pinta's current state.
///
/// # Fields
/// - `exit`: bool for exiting Pinta
pub struct App {
    pub exit: bool,
} // App{}

impl App {
    /// Creates a new instance of [`App`].
    ///
    /// This method initializes the App struct with default settings.
    /// See the [`App`] struct for more information about its fields.
    pub fn new() -> Self {
        App { exit: false }
    } // App::new()

    /// Runs the loop for [`App`].
    ///
    /// Until `self.exit`, the terminal will draw and handle events
    ///
    /// # Return
    /// This function returns an [`std::io::Result`] depending on the loop's functionality
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
