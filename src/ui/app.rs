use std::error;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};

use tui::text::Text;
use tui::widgets::Wrap;
use unicode_width::UnicodeWidthStr;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            query: String::from(""),
            input_mode: InputMode::Normal,
            json: String::from(""),
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
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples
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
            self.prepare_input()
                .block(Block::default().title("JSON").borders(Borders::ALL)),
            l[0],
        );

        frame.render_widget(Block::default().title("Output").borders(Borders::ALL), l[1]);
    }

    pub fn prepare_input(&self) -> Paragraph {
        Paragraph::new(Text::from(self.json.as_ref())).wrap(Wrap { trim: false })
    }
}
