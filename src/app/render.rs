pub mod renders {

    use crate::app::app::App;

    use ratatui::{
        buffer::Buffer,
        layout::Rect,
        style::Stylize,
        symbols::border,
        text::{Line, Text},
        widgets::{Block, Paragraph, Widget},
    };

    impl Widget for &App {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let title = Line::from("  Pinta  ".bold());
            let instructions = Line::from(vec![
                " Up ".into(),
                "<K>".blue().bold(),
                " Down ".into(),
                "<J>".blue().bold(),
                " Back ".into(),
                "<H>".blue().bold(),
                " Forward ".into(),
                "<L>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]);
            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered())
                .border_set(border::THICK);

            let counter_text = Text::from("The ship to your coding destination");

            Paragraph::new(counter_text)
                .centered()
                .block(block)
                .render(area, buf);
        }
    }
}
