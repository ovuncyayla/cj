use std::error;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};

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

#[allow(dead_code)]
#[derive(Debug, PartialEq, Default)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Default)]
pub struct SectionBlock<'a> {
    pub rect: Rect,
    pub block: Block<'a>,
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    pub running: bool,
    pub input_mode: InputMode,
    pub query: String,

    pub query_section: SectionBlock<'a>,
    pub input_section: SectionBlock<'a>,
    pub output_section: SectionBlock<'a>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            running: true,
            query: String::from(""),
            query_section: Default::default(),
            input_section: Default::default(),
            output_section: Default::default(),
            input_mode: InputMode::Normal,
        }
    }
}

impl App<'_> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
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
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(frame.size());

        let input = Paragraph::new(self.query.as_ref())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::RAPID_BLINK),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));

        self.query_section.block = Block::default()
            .title("Query")
            .borders(Borders::ALL)
            .style(Style::default().add_modifier(Modifier::RAPID_BLINK));
        self.query_section.rect = l[0];
        frame.render_widget(input, self.query_section.rect);

        match self.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => frame.set_cursor(
                self.query_section.rect.x + self.query.width() as u16 + 1,
                self.query_section.rect.y + 1,
            ),
        }
        let chunks = layout(frame.size());

        self.input_section.block = Block::default().title("JSON").borders(Borders::ALL);
        self.input_section.rect = chunks[0];
        frame.render_widget(self.input_section.block.clone(), self.input_section.rect);

        self.output_section.block = Block::default().title("Output").borders(Borders::ALL);
        self.output_section.rect = chunks[1];
        frame.render_widget(self.output_section.block.clone(), self.output_section.rect);
    }
}

fn layout(size: tui::layout::Rect) -> Vec<tui::layout::Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(size);

    let l = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout[1]);
    l
}
