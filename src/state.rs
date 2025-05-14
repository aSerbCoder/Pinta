pub mod state {

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
            self.interface
                .write_directory_contents(self.directory.dir_contents.as_ref());
        }
    }
}
