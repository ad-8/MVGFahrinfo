#[allow(unused, dead_code)]
use anyhow::Result; //to avoid writing the error type

pub mod api;
pub mod app;
pub mod components;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

//own modules

use app::App;
use event::{Event, EventHandler};
use tui::Tui;
use update::update;

use ratatui::prelude::{CrosstermBackend, Terminal};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>; // alias for the frame type

#[tokio::main]
async fn main() -> Result<()> {
    println!("fetching stations...");
    let mut app = App::new().await;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        if app.should_redraw {
            tui.draw(&mut app)?;
            app.should_redraw = false;
        }

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event).await,
        };
    }
    tui.exit()?;
    return Ok(());
}
