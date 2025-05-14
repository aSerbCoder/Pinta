mod directory;
mod events;
mod state;
mod terminal;
mod ui;
use events::events::check_events;

fn main() {
    check_events();
}
