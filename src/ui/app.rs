use jsonpath_rust::JsonPathFinder;
use serde_json::{to_string_pretty, Value};
use std::error;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};

use tui::text::{Span, Spans, Text};
use tui::widgets::{List, ListItem, Wrap};
use unicode_width::UnicodeWidthStr;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::extract_values;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub input_mode: InputMode,
    pub query: String,
    pub json: String,
    pub result: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            query: String::from(""),
            input_mode: InputMode::Normal,
            json: String::from(""),
            result: String::from(""),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(content: String) -> Self {
        App {
            json: content,
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let l = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
            .split(frame.size());

        let input = Paragraph::new(self.query.as_ref())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::RAPID_BLINK),
            })
            .block(Block::default().borders(Borders::ALL).title("Query"));

        match self.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => {
                frame.set_cursor(l[0].x + self.query.width() as u16 + 1, l[0].y + 1)
            }
        }
        frame.render_widget(input, l[0]);

        let l = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(l[1]);

        frame.render_widget(
            self.prepare_input(&self.json)
                .block(Block::default().title("JSON").borders(Borders::ALL)),
            l[0],
        );

        frame.render_widget(
            self.prepare_input(&self.result)
                .block(Block::default().title("Output").borders(Borders::ALL)),
            l[1],
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
