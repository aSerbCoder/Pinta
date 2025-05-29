use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Padding},
};

pub fn draw_border_retur_layout(frame: &mut Frame) -> Rect {
    let main_area = frame.area();

    let main_border = Block::new()
        .title_top(" Pinta ")
        .title_bottom(Line::from(" Checker "))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title_alignment(ratatui::layout::Alignment::Center)
        .padding(Padding {
            left: 8,
            right: 8,
            top: 2,
            bottom: 2,
        })
        .borders(Borders::ALL);

    let inner = main_border.inner(main_area);
    frame.render_widget(main_border, main_area);

    inner
}
