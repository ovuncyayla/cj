use jsonpath_rust::JsonPathFinder;
use serde_json::{to_string_pretty, Value};
use std::error;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};

use tui::text::{Span, Spans, Text};
use tui::widgets::{List, ListItem, Widget, Wrap};
use unicode_width::UnicodeWidthStr;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Drawable<T: Widget>(T, Rect);

#[derive(Debug, Default)]
pub struct Component {
    pub buffer: String,
    pub shape: Rect,
    pub cursor_pos: (u16, u16),
}

pub struct AppLayout(Rect, Rect, Rect);

fn make_app_layout(frame: Rect) -> AppLayout {
    let top = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
        .split(frame);

    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(top[1]);

    AppLayout(top[0], bottom[0], bottom[1])
}

fn draw_query_component<B: Backend>(frame: &mut Frame<'_, B>, app: &mut App, area: Rect) {
    let input = Paragraph::new(app.query.clone())
        .block(Block::default().borders(Borders::ALL).title("Query"))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::RAPID_BLINK),
        });
    frame.render_widget(input, area);

    app.input_cursor_offset = app.query.width() as u16 + 1;
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => frame.set_cursor(area.x + app.input_cursor_offset, area.y + 1),
    };
}

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}

/// Application.
#[derive(Default)]
pub struct App {
    pub running: bool,
    pub input_mode: InputMode,
    pub query: String,
    pub json: String,
    pub result: String,
    pub input_cursor_offset: u16,
    pub input_block_pos: (u16, u16),
}

impl App {
    pub fn new(content: String) -> Self {
        App {
            json: content,
            running: true,
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let layout = make_app_layout(frame.size());

        draw_query_component(frame, self, layout.0);

        frame.render_widget(
            self.prepare_input(&self.json)
                .block(Block::default().title("JSON").borders(Borders::ALL)),
            layout.1,
        );

        frame.render_widget(
            self.prepare_input(&self.result)
                .block(Block::default().title("Output").borders(Borders::ALL)),
            layout.2,
        );
    }

    pub fn prepare_input(&self, content: &str) -> List {
        let lines: Vec<ListItem> = content
            .lines()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(vec![
                    Span::styled(format!("{} ", i), Style::default().fg(Color::Yellow)),
                    Span::raw(m.to_string()),
                ])];
                ListItem::new(content)
            })
            .collect();
        List::new(lines)
    }

    pub fn eval_query(&mut self) {
        let result = JsonPathFinder::from_str(&self.json, &self.query);
        if result.is_err() {
            // TODO Make query input RED in case of err
            return;
        }
        self.result = to_string_pretty(&result.unwrap().find()).unwrap();
    }
}
