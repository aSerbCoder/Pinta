pub mod terminal {

    pub struct Terminal;

    impl Terminal {
        fn init() {
            Terminal::clear_terminal();
            crossterm::terminal::enable_raw_mode().unwrap();
        } // Terminal::init()

        fn clear_terminal() {
            use crossterm::{execute, terminal::Clear, terminal::ClearType};
            use std::io::stdout;

            let mut stdout = stdout();

            // clear screen
            execute!(stdout, Clear(ClearType::All)).unwrap();
        } // Terminal::clear_terminal()

        pub fn new() -> Self {
            Terminal::init();
            Terminal {}
        } // Terminal::new()

        pub fn close() {
            use crossterm::execute;
            use crossterm::style::Color;
            use crossterm::style::Print;
            use crossterm::style::SetBackgroundColor;
            use std::io::stdout;

            let mut stdout = stdout();

            execute!(stdout, Print("\r\n")).unwrap();
            execute!(stdout, SetBackgroundColor(Color::Rgb { r: 0, g: 0, b: 0 })).unwrap();

            Terminal::clear_terminal();

            crossterm::terminal::disable_raw_mode().unwrap();
        } // Terminal::close()
    } // Terminal{}
} // terminal
