use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};

use crate::app::app::App;

pub fn draw(app: &mut App, frame: &mut Frame, popup_area: Rect) {
    frame.render_widget(Clear, popup_area);

    let help_items = vec![
        ("q", "Quit"),
        ("j / k", "Move down / up"),
        ("h / l", "Back / enter dir"),
        ("a", "Toggle hidden"),
        ("Enter", "Open in tmux"),
        ("Shift + H", "Help"),
    ];

    let mut lines: Vec<Line> = Vec::new();

    for (i, (key, desc)) in help_items.iter().enumerate() {
        let line = if i == app.help_selected_line {
            Line::from(vec![
                Span::styled(
                    format!("{:<10}", key),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(desc.to_string(), Style::default().fg(Color::White)),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{:<10}", key), Style::default().fg(Color::Cyan)),
                Span::raw(desc.to_string()),
            ])
        };
        lines.push(line);
    }

    app.help_total_lines = lines.len();
    app.help_visible_height = popup_area.height.saturating_sub(2) as usize;

    let max_scroll = app.help_total_lines.saturating_sub(app.help_visible_height);
    app.help_scroll = app.help_scroll.min(max_scroll);

    app.help_scroll_state = app
        .help_scroll_state
        .content_length(app.help_total_lines)
        .viewport_content_length(app.help_visible_height)
        .position(app.help_scroll);

    let paragraph = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Help ")
                .title_alignment(Alignment::Center),
        )
        .scroll((app.help_scroll as u16, 0))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, popup_area);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        popup_area,
        &mut app.help_scroll_state,
    );
}
