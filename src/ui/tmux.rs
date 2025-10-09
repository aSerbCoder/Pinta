use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation},
};

use crate::app::app::App;

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();
    let mut selected_session_line_index = 0;

    if app.tmux_sessions.is_empty() {
        lines.push(Line::from(Span::styled(
            "No tmux sessions found",
            Style::default().fg(Color::Red),
        )));
    } else {
        let mut current_line_index = 0;

        for (i, session) in app.tmux_sessions.iter().enumerate() {
            if i == app.tmux_selected_line {
                selected_session_line_index = current_line_index;
            }

            let session_line = format!("{} (created at {})", session.name, session.date_created);
            let styled_session_line = if i == app.tmux_selected_line {
                Line::from(Span::styled(
                    session_line,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(session_line)
            };
            lines.push(styled_session_line);
            current_line_index += 1;

            for window in &session.windows {
                let window_line = format!(
                    "  [{}] {}{}",
                    window.index,
                    window.name,
                    if window.active { " (active)" } else { "" }
                );
                lines.push(Line::from(window_line));
                current_line_index += 1;
            }

            lines.push(Line::from(""));
            current_line_index += 1;
        }
    }

    app.tmux_total_lines = lines.len();
    app.tmux_visible_height = area.height.saturating_sub(2) as usize;

    if !app.tmux_sessions.is_empty() {
        if selected_session_line_index < app.tmux_scroll {
            app.tmux_scroll = selected_session_line_index;
        } else if selected_session_line_index >= app.tmux_scroll + app.tmux_visible_height {
            app.tmux_scroll = selected_session_line_index + 1 - app.tmux_visible_height;
        }
    } else {
        app.tmux_scroll = 0;
    }

    let max_scroll = app.tmux_total_lines.saturating_sub(app.tmux_visible_height);
    app.tmux_scroll = app.tmux_scroll.min(max_scroll);

    app.tmux_scroll_state = app
        .tmux_scroll_state
        .content_length(app.tmux_total_lines)
        .viewport_content_length(app.tmux_visible_height)
        .position(app.tmux_scroll);

    let border_style = if app.selected_tab == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let paragraph = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(" Tmux Sessions "),
        )
        .scroll((app.tmux_scroll as u16, 0))
        .style(Style::default());

    frame.render_widget(paragraph, area);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        area,
        &mut app.tmux_scroll_state,
    );
}
