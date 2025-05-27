use crate::app::App;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::io;

/// Handles key events of [`App`].
fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        crossterm::event::KeyCode::Char('q') => app.exit(),
        _ => {}
    }
}

/// Handles events of [`App`].
///
/// Checks for key events with the [`handle_key_event()`] function.
///
/// # Return
/// This function returns an [`std::io::Result<()>`] depending on the [`crossterm::event::read()`]
/// function.
pub(super) fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event);
        }
        _ => {}
    }
    Ok(())
} // App::handle_events()
