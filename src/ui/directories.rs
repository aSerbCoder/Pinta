use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation},
};

use crate::app::app::App;

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let text: Vec<Line> = app
        .current_directory_contents
        .iter()
        .enumerate()
        .map(|(index, path)| {
            if index == app.selected_line {
                Line::from(Span::styled(
                    format!(
                        "{}",
                        path.file_name()
                            .expect(&format!(
                                "Could not get path name of path: {}",
                                path.to_string_lossy()
                            ))
                            .to_string_lossy()
                    ),
                    Style::default(),
                ))
            } else {
                Line::from(Span::styled(
                    format!(
                        "{}",
                        path.file_name()
                            .expect(&format!(
                                "Could not get path name of path: {}",
                                path.to_string_lossy()
                            ))
                            .to_string_lossy()
                    ),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            }
        })
        .collect();

    app.total_lines = text.len();

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
                .title(format!(" {} ", app.current_directory.to_string_lossy())),
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
