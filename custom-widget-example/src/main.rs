use crossterm::event::{self, Event};
use ratatui::{Frame, buffer::Buffer, layout::Rect, style::{Color, Style}, widgets::Widget};

pub struct MyWidget {
    col: u16,
    content: &'static str,
    color: Color,
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top() + self.col, &self.content, Style::default().fg(self.color));
    }
}

pub struct MyWidgetWithState {
    col: u16,
    is_activated: bool,
    counter: u32,
}

// WidgetをStatefulにする
// reference: https://github.com/ratatui/ratatui/discussions/1881
impl MyWidgetWithState {
    pub fn new(col: u16) -> Self {
        MyWidgetWithState {
            col,
            is_activated: false,
            counter: 0,
        }
    }
}

impl Widget for &MyWidgetWithState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color = if self.is_activated { Color::Yellow } else { Color::Gray };
       buf.set_string(area.left(), area.top() + self.col, format!("counter: {0}", self.counter), color);
    }
}

fn main() {
    let mut terminal = ratatui::init();
    let mut widget_with_state = MyWidgetWithState::new(10);
    loop {
        terminal.draw(|frame| { draw(frame, &widget_with_state)}).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            if widget_with_state.counter < 5 {
                widget_with_state.counter += 1;
                widget_with_state.is_activated = !widget_with_state.is_activated;
            } else {
                break;
            }
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame, widget: &MyWidgetWithState) {
    let widget1  = MyWidget { col: 0, content: "Hello, World!", color: Color::White };
    let widget2 = MyWidget { col: 5, content: "Hello, ratatui!", color: Color::Yellow };
    frame.render_widget(widget1, frame.area());
    frame.render_widget(widget2, frame.area());
    frame.render_widget(widget, frame.area());
}

