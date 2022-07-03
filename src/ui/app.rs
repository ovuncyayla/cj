use std::error;
use tui::layout::Alignment;
use tui::style::{Color, Style};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
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
        let block = Block::default().title("Block").borders(Borders::ALL);
        frame.render_widget(block, l[0]);
        let chunks = layout(frame.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        frame.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        frame.render_widget(block, chunks[1]);
        // let block = Block::default().title("Block 3").borders(Borders::ALL);
        // frame.render_widget(block, chunks[2]);
        // frame.render_widget(
        //     Paragraph::new("{{project-name}}")
        //         .block(Block::default().borders(Borders::ALL))
        //         .style(Style::default().fg(Color::White).bg(Color::Black))
        //         .alignment(Alignment::Center),
        //     frame.size(),
        // )
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
