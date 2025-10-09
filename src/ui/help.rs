use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
};

use crate::app::app::App;
pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let paragraph = Line::from("Press h for help").centered();

    frame.render_widget(paragraph, area);
}
