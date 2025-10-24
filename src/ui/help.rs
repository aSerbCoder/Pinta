use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, Tabs,
    },
};

use crate::app::app::App;

pub fn draw(app: &mut App, frame: &mut Frame, popup_area: Rect) {
    frame.render_widget(Clear, popup_area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(popup_area);

    let tab_titles = vec![
        Span::styled("Help", Style::default().fg(Color::Cyan)),
        Span::styled("Directories", Style::default().fg(Color::Cyan)),
        Span::styled("Tmux", Style::default().fg(Color::Cyan)),
        Span::styled("Search", Style::default().fg(Color::Cyan)),
    ];

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Help Categories "),
        )
        .select(app.help_selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::styled("|", Style::default().fg(Color::DarkGray)));

    frame.render_widget(tabs, layout[0]);

    let help_items = match app.help_selected_tab {
        0 => vec![
            ("H", "Toggle help menu on/off"),
            ("← / →", "Switch between help categories"),
            ("1 - 4", "Jump directly to a help tab"),
            ("Esc / q", "Exit help"),
            ("↑ / ↓", "Scroll help items"),
        ],
        1 => vec![
            ("h / l", "Go back / enter directory"),
            ("j / k", "Move down / up"),
            ("A", "Toggle hidden"),
            ("t", "Open current dir in tmux"),
            ("/", "Start search"),
            ("n / N", "Next / previous match"),
        ],
        2 => vec![
            ("j / k", "Move between sessions"),
            ("t", "Attach or re-enter selected session"),
        ],
        3 => vec![
            ("Typing", "Enter search text"),
            ("Backspace", "Delete last character"),
            ("n / N", "Next / previous match"),
            ("Enter", "Finish search"),
            ("Esc", "Exit search mode"),
        ],
        _ => vec![],
    };

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
    app.help_visible_height = layout[1].height.saturating_sub(2) as usize;
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
                .title(format!(
                    " {} Help ",
                    match app.help_selected_tab {
                        0 => "General",
                        1 => "Directories",
                        2 => "Tmux",
                        3 => "Search",
                        _ => "",
                    }
                ))
                .title_alignment(Alignment::Center),
        )
        .scroll((app.help_scroll as u16, 0))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, layout[1]);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        layout[1],
        &mut app.help_scroll_state,
    );
}
