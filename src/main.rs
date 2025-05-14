mod directory;
mod events;
mod state;
mod terminal;
mod ui;
use events::events::check_events;
use state::state::State;
use terminal::terminal::*;

fn main() {
    Terminal::new();
    let mut state = State::new();
    state.init_state();

    check_events();
    Terminal::close();
}
