use crossterm::event::{self, Event};
use ratatui::{Frame, style::{Color, Modifier, Style}, text::{Line, Span, Text}};

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
    let span = Span::styled(
        "Hello, Ratatui!",
        Style::default().fg(Color::Red).bg(Color::Yellow),
    );
    let bold_span = Span::styled(
        "This is bold",
        Style::default().add_modifier(Modifier::BOLD),
    );
    let italic_span = Span::styled(
        "This is italic",
        Style::default().add_modifier(Modifier::ITALIC),
    );
    let bold_italic_text = Span::styled(
        "This is bold and italic",
        Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC),
    );
    let mixed_line = Line::from(vec![
        Span::styled("This is mixed ", Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC)),
        Span::styled("styling", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::from("!"),
    ]);
    let text = Text::from(vec![
        span.into(), bold_span.into(), italic_span.into(), bold_italic_text.into(),
        mixed_line,
    ]);
    frame.render_widget(text, frame.area());
}

