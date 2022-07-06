use super::app::{App, AppResult};
use super::event::{Event, EventHandler};
use super::handler::handle_key_events;
use super::tui::Tui;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn run() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Rc::new(RefCell::new(Terminal::new(backend)?));
    let events = EventHandler::new(250);
    let mut tui = Tui::new(Rc::clone(&terminal), events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app, Rc::clone(&terminal))?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
