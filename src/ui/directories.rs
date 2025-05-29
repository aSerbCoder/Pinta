use std::path::Path;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn draw_directories(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        area,
    );
}
