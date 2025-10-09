use crate::app::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation},
};
use std::time::Duration;

pub fn draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let highlight_active = app.searched_string.len() > 1 && !app.search_matches.is_empty();
    let query = app.searched_string.to_lowercase();

    let _ = app
        .search_match_index
        .and_then(|si| app.search_matches.get(si).copied());

    let mut lines: Vec<Line> = Vec::new();

    for (index, path) in app.current_directory_contents.iter().enumerate() {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<invalid>")
            .to_string();

        let spans: Vec<Span> = if highlight_active && app.search_matches.contains(&index) {
            let lower_name = file_name.to_lowercase();
            let mut built = Vec::new();
            let mut start = 0;
            let mut remaining = lower_name.as_str();

            let match_style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);

            while let Some(pos) = remaining.find(&query) {
                let abs_pos = start + pos;
                let abs_end = abs_pos + query.len();

                if abs_pos > start {
                    built.push(Span::raw(file_name[start..abs_pos].to_string()));
                }

                built.push(Span::styled(
                    file_name[abs_pos..abs_end].to_string(),
                    match_style,
                ));

                start = abs_end;
                remaining = &lower_name[start..];
            }

            if start < file_name.len() {
                built.push(Span::raw(file_name[start..].to_string()));
            }

            built
        } else {
            vec![Span::raw(file_name.clone())]
        };

        let mut spans = spans;

        if index == app.directories_selected_line {
            for s in spans.iter_mut() {
                s.style = s.style.patch(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
            }
        }

        lines.push(Line::from(spans));
    }

    app.directories_total_lines = lines.len();
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

    let paragraph = Paragraph::new(Text::from(lines))
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

    let show_popup = app.searching
        || app
            .last_search_update
            .map(|t| t.elapsed() < Duration::from_secs(2))
            .unwrap_or(false);

    if show_popup && !app.searched_string.is_empty() {
        let total_matches = app.search_matches.len();
        let current_index = app.search_match_index.map(|i| i + 1).unwrap_or(0);

        let status_text = if app.searching {
            format!(
                "/{}  ({} match{})",
                app.searched_string,
                total_matches,
                if total_matches == 1 { "" } else { "es" }
            )
        } else if total_matches > 0 {
            format!(
                "/{}  [{}/{}]",
                app.searched_string, current_index, total_matches
            )
        } else {
            format!("/{ }  (no matches)", app.searched_string)
        };

        let popup_width = (status_text.len() as u16 + 4).min(area.width.saturating_sub(4));
        let popup_area = Rect {
            x: area.x + 2,
            y: area.y + area.height.saturating_sub(3),
            width: popup_width,
            height: 3,
        };

        let popup = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(" Search "),
            );

        frame.render_widget(popup, popup_area);
    }
}
