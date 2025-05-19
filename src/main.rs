use app::app::App;
use std::io;

mod commands;
mod directory;

mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
