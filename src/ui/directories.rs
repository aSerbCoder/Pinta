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
            if index == app.directories_selected_line {
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
                    Style::default(),
                ))
            }
        })
        .collect();

    app.directories_total_lines = text.len();

    app.directories_visible_height = area.height.saturating_sub(2) as usize;

    let max_scroll = app
        .directories_total_lines
        .saturating_sub(app.directories_visible_height);
    app.directories_scroll = app.directories_scroll.min(max_scroll);

    app.directories_scroll_state = app
        .directories_scroll_state
        .content_length(app.directories_total_lines)
        .viewport_content_length(app.directories_visible_height)
        .position(app.directories_scroll);

    let border_style = if app.selected_tab == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" {} ", app.current_directory.to_string_lossy())),
        )
        .scroll((app.directories_scroll as u16, 0));

    frame.render_widget(paragraph, area);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.directories_scroll_state,
    );
}
