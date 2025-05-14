pub mod events {
    use crossterm::event::{Event, KeyCode, read};

    use crate::terminal::terminal::Terminal;

    pub fn check_events() {
        use crate::state::state::State;
        let mut state = State::new();
        let mut terminal = Terminal::new();
        loop {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        break;
                    }

                    _ => {}
                },
                Event::FocusLost => {}
                Event::FocusGained => {}
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
                Event::Resize(nc, nr) => {
                    terminal.rows = nr;
                    terminal.columns = nc;
                }
            }

            state.init_state();
        }

        Terminal::close();
    }
}
