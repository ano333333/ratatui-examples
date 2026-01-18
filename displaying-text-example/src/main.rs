use crossterm::event::{self, Event};
use ratatui::{Frame, layout::Alignment, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, Paragraph}};

fn main() {
    let mut terminal = ratatui::init();
    let mut mode = 0;
    loop {
        terminal.draw(match mode {
            0 => draw_span,
            1 => draw_line,
            _ => draw_text,
        }).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            if mode < 2 {
                mode += 1;
            } else {
                break;
            }
        }
    }
    ratatui::restore();
}

fn draw_span(frame: &mut Frame) {
    let span = Span::styled("This is text that will be yellow", Style::default().fg(Color::Yellow));
    frame.render_widget(span, frame.area());
}

fn draw_line(frame: &mut Frame) {
    let line = Line::from(vec![
        Span::styled("hello", Style::default().fg(Color::Red)),
        Span::styled(" ", Style::default()),
        Span::styled("world", Style::default().fg(Color::Red).bold()),
    ]).alignment(Alignment::Center);
    frame.render_widget(line, frame.area());
}

fn draw_text(frame: &mut Frame) {
    let line_c = Line::from(vec![
        Span::styled("hello", Style::default().fg(Color::Red)),
        Span::styled(" ", Style::default()),
        Span::styled("world", Style::default().fg(Color::Red).bold()),
    ]).centered();
    let line_l = Line::from("hello world left").left_aligned();
    let line_r = Line::from("hello world right").right_aligned();
    let text = Text::from(vec![
        line_l,
        line_c,
        line_r,
    ]).centered();
    frame.render_widget(Paragraph::new(text).block(Block::bordered()), frame.area());
}

