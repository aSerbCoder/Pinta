pub mod events {
    use std::time::Duration;

    use crossterm::event::{Event, KeyCode, poll, read};

    use crate::terminal::terminal::Terminal;

    pub fn running() {
        use crate::state::state::State;
        let mut state = State::new();
        let mut terminal = Terminal::new();
        loop {
            if poll(Duration::from_millis(100)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => match event.code {
                        KeyCode::Esc => break,
                        _ => {}
                    },
                    Event::Resize(nc, nr) => {
                        terminal.columns = nc;
                        terminal.rows = nr;
                    }
                    _ => {}
                }
            }

            terminal.update();
            state.init_state();
        }
        Terminal::close();
    }
}
