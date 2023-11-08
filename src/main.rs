// #[allow(unused, dead_code)]
use anyhow::Result; //to avoid writing the error type <Box dyn Error> everywhere

pub mod api;
pub mod app;
pub mod components;
pub mod config;
pub mod constants;
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

use crate::{config::Config, update::initiate_auto_refresh};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>; // alias for the frame type

#[tokio::main]
async fn main() -> Result<()> {
    println!("fetching stations...");

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);

    let sender = events.sender.clone(); //we can clone it as we can have multiple senders for this channel

    let mut app = App::new().await;

    initiate_auto_refresh(sender);

    let config = Config::parse();
    app.scroll_state.select(config.fav_station_idx);
    app.select_station().await;
    // current behavior: no highlighting until user starts scrolling, selecting new destination sets highlight to none
    // app.dep_tbl_state.select(Some(0));
    let display_seconds = config.display_seconds.unwrap_or_default();
    let refresh_rate = config.refresh_rate.unwrap_or(5) as i64;

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        if app.should_redraw {
            //this makes sure that we don't redraw the screen if there is no change
            tui.draw(&mut app)?;
            app.should_redraw = false;
        }

        match tui.events.next()? {
            Event::Tick => {
                if display_seconds {
                    app.update_seconds_since_last_refresh(refresh_rate);
                } 
            } //every 250ms we get a tick event
            Event::Key(key_event) => update(&mut app, key_event).await,
        };
    }

    tui.exit()?;
    return Ok(());
}
