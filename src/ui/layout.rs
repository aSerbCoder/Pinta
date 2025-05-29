use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Table},
};

use crate::app::App;

use super::{
    border::draw_border_retur_layout,
    directories::{self, draw_directories},
};

/// Creates the layout of the program
///
/// # Functionality
/// Takes in the application state [`App`] and a [`ratatui::Frame`]
/// Creates divs and main border for the program
pub fn draw_layout(app: &mut App, frame: &mut Frame) {
    let layout_area = draw_border_retur_layout(frame);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(layout_area);

    directories::draw_directories(app, frame, layout[0]);
}
