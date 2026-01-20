use crossterm::event::{self, Event};
use derive_setters::Setters;
use lipsum::lipsum;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

fn main() {
    let mut terminal = ratatui::init();
    let mut with_clear = false;
    loop {
        terminal
            .draw(if with_clear {
                draw_with_clear
            } else {
                draw_without_clear
            })
            .expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            if !with_clear {
                with_clear = true;
            } else {
                break;
            }
        }
    }
    ratatui::restore();
}

#[derive(Debug, Default, Setters)]
struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

fn draw_without_clear(frame: &mut Frame) {
    let area = frame.area();
    let background_text = Paragraph::new(lipsum(1000))
        .wrap(Wrap { trim: true })
        .style(
            Style::new()
                .light_blue()
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        );
    frame.render_widget(background_text, area);

    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 3,
        width: area.width / 2,
        height: area.height / 3,
    };
    let popup = Popup::default()
        .content("Hello world!")
        .style(Style::new().yellow())
        .title("Without Clear")
        .title_style(Style::new().white().bold())
        .border_style(Style::new().red());
    frame.render_widget(popup, popup_area);
}

fn draw_with_clear(frame: &mut Frame) {
    let area = frame.area();
    let background_text = Paragraph::new(lipsum(1000))
        .wrap(Wrap { trim: true })
        .style(
            Style::new()
                .light_blue()
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        );
    frame.render_widget(background_text, area);

    let popup_area = Rect {
        x: area.width / 4,
        y: area.height / 3,
        width: area.width / 2,
        height: area.height / 3,
    };
    let popup = Popup::default()
        .content("Hello world!")
        .style(Style::new().yellow())
        .title("With Clear")
        .title_style(Style::new().white().bold())
        .border_style(Style::new().red());
    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup, popup_area);
}
