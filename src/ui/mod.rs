use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Row, Table},
};

use crate::app::app::App;

mod directories;
mod help;
mod title;
mod tmux;

pub fn draw(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .title(" Pinta ")
        .title_alignment(Alignment::Center)
        .title_bottom(" <H> Help ")
        .border_style(Style::default().fg(Color::White));

    frame.render_widget(outer_block, area);

    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let chunks = Layout::vertical([
        Constraint::Min(inner_area.height.saturating_sub(1)),
        Constraint::Length(1),
    ])
    .split(inner_area);

    let content_area = chunks[0];

    let inner_chunks = Layout::vertical([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(content_area);

    if app.show_help {
        let popup_width = area.width.min(50);
        let popup_height = area.height.min(10);

        let popup_area = Rect {
            x: (area.width.saturating_sub(popup_width)) / 2,
            y: (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };
        help::draw(app, frame, popup_area);
    }

    directories::draw(app, frame, inner_chunks[0]);
    tmux::draw(app, frame, inner_chunks[1]);
}
