use app::app::App;
use color_eyre::Result;
mod app;
mod commands;
mod paths;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
