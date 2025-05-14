mod directory;
mod events;
mod state;
mod terminal;
mod ui;
use directory::directory::Directory;
use events::events::check_events;
use terminal::terminal::*;
use ui::ui::Interface;

fn main() {
    Terminal::new();
    let directory = Directory::new();
    let interface = Interface::new();
    interface.write_cur_directory(directory.cur_dir);

    check_events();
    Terminal::close();
}
