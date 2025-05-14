mod events;
mod state;
mod terminal;
use events::events::check_events;
use terminal::terminal::*;

fn main() {
    Terminal::new();
    check_events();
    Terminal::close();
}
