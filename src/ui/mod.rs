use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation},
};

use crate::app::app::App;

pub fn draw(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(area);

    let area = chunks[1];

    let text: Vec<Line> = (0..35)
        .map(|i| {
            if i == app.selected_line {
                Line::from(Span::styled(
                    format!("Hello {i}"),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    format!("Hello {i}"),
                    Style::default().fg(Color::Gray),
                ))
            }
        })
        .collect();

    app.total_lines = text.len();

    // record visible height for use in scroll logic
    app.visible_height = area.height.saturating_sub(2) as usize;

    let max_scroll = app.total_lines.saturating_sub(app.visible_height);
    app.vertical_scroll = app.vertical_scroll.min(max_scroll);

    app.vertical_scroll_state = app
        .vertical_scroll_state
        .content_length(app.total_lines)
        .viewport_content_length(app.visible_height)
        .position(app.vertical_scroll);

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Vertical scrollbar with arrows"),
        )
        .scroll((app.vertical_scroll as u16, 0));

    frame.render_widget(paragraph, area);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.vertical_scroll_state,
    );
}
