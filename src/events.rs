pub mod events {
    use crossterm::event::{Event, read};

    pub fn check_events() {
        use crate::state::state::State;
        loop {
            match read().unwrap() {
                Event::Key(event) => {
                    if State::key_press_esc(event) {
                        break;
                    }
                }
                Event::FocusLost => {}
                Event::FocusGained => {}
                Event::Mouse(event) => {}
                Event::Paste(data) => {}
                Event::Resize(width, height) => {}
            }
        }
    }
}
