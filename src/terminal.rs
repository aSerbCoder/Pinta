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

        pub fn new() -> Terminal {
            Terminal::init();
            Terminal {}
        } // Terminal::new()

        pub fn close() {
            Terminal::clear_terminal();
            crossterm::terminal::disable_raw_mode().unwrap();
        } // Terminal::close()
    } // Terminal{}
} // terminal
