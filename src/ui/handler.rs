use super::app::{App, AppResult, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io::Stderr;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use std::cell::RefCell;
use std::rc::Rc;
use unicode_width::UnicodeWidthStr;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    terminal: Rc<RefCell<Terminal<CrosstermBackend<Stderr>>>>,
) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Editing;
                // let mut t = terminal.borrow_mut();
                // t.set_cursor(
                //     app.query_section.rect.x + app.query.width() as u16 + 1,
                //     app.query_section.rect.y,
                // )?;
            }
            KeyCode::Char('q') => {
                return Ok(());
            }
            // exit application on ESC
            KeyCode::Esc => {
                app.running = false;
            }
            _ => {}
        },
        InputMode::Editing => match key_event.code {
            KeyCode::Enter => {
                // app.messages.push(app.query.drain(..).collect());
            }
            KeyCode::Char(c) => {
                app.query.push(c);
            }
            KeyCode::Backspace => {
                app.query.pop();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
    }
    match key_event.code {
        // exit application on Ctrl-D
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        _ => {}
    }
    Ok(())
}
