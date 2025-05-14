pub mod state {
    use crossterm::event::{KeyCode, KeyEvent};

    use crate::{directory::directory::Directory, ui::ui::Interface};

    pub struct State {
        directory: Directory,
        interface: Interface,
    }

    impl State {
        pub fn new() -> State {
            State {
                directory: Directory::new(),
                interface: Interface::new(),
            }
        }

        pub fn init_state(&mut self) {
            self.interface.write_cur_directory(self.directory.path());
        }

        pub fn key_press_esc(event: KeyEvent) -> bool {
            event.code == KeyCode::Esc
        }
    }
}
