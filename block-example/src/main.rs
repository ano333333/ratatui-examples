use crossterm::event::{self, Event};
use ratatui::{Frame, style::{Color, Style}, text::Line, widgets::{Block, BorderType, Borders}};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let b = Block::default()
        .title(Line::from("     Left Title").left_aligned())
        .title(Line::from("Middle Title").centered())
        .title(Line::from("Right Title     ").right_aligned())
        .border_style(Style::default().fg(Color::Magenta))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);
    frame.render_widget(b, frame.area());
}

