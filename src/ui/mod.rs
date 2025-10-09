use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::app::app::App;
mod directories;
mod title;

pub fn draw(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(area);

    title::draw(app, frame, chunks[0]);
    directories::draw(app, frame, chunks[1]);
}
