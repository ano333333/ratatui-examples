use crossterm::event::{self, Event};
use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Wrap};
use ratatui::{Frame, style::Style, widgets::Paragraph};

fn main() {
    let mut terminal = ratatui::init();
    let mut showing_ui = 0;
    loop {
        terminal
            .draw(match showing_ui {
                0 => draw1,
                _ => draw2,
            })
            .expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            if showing_ui == 0 {
                showing_ui = 1;
            } else {
                break;
            }
        }
    }
    ratatui::restore();
}

fn draw1(frame: &mut Frame) {
    let p = Paragraph::new("A veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeery long text that might not fit the container...")
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Title")
                .border_type(BorderType::Rounded)
        );
    frame.render_widget(p, frame.area());
}

fn draw2(frame: &mut Frame) {
    let mut lines = vec![];
    lines.push(Line::from(vec![
        Span::styled("Hello ", Style::default().fg(Color::Yellow)),
        Span::styled("World", Style::default().fg(Color::Blue).bg(Color::White)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Goodbye ", Style::default().fg(Color::Yellow)),
        Span::styled("World", Style::default().fg(Color::Blue).fg(Color::White)),
    ]));
    let text = Text::from(lines);
    let p = Paragraph::new(text);
    frame.render_widget(p, frame.area());
}
