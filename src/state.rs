pub mod state {
    use crossterm::event::{KeyCode, KeyEvent};

    pub struct State {}

    impl State {
        pub fn key_press_esc(event: KeyEvent) -> bool {
            event.code == KeyCode::Esc
        }
    }
}
