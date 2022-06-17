use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend}, Terminal,
};

mod ui;
mod api;
mod state;
mod events;

use api::ApiClient;
use events::{handle_area, handle_global};
use state::{State, Screen};

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State, client: ApiClient) -> io::Result<()> {
    loop {
        match state.screen {
            Screen::Search => {
                terminal.draw(|f| ui::draw_search_screen(f, &state.input, &mut state.subbreddits))?;
            }
            Screen::Details => {
                match &state.selected_subreddit {
                    Some(details) => {
                        terminal.draw(|f| ui::draw_detail_screen(f, details, &mut state.articles))?;
                    }
                    None => {
                        panic!("Exiting");
                    }
                }
            }
            Screen::Article => {
                match state.articles.state.selected() {
                    Some(index) => {
                        terminal.draw(|f| ui::draw_article_screen(f, &state.articles.items[index], &mut state.comments, &mut state.scroll))?;
                    }
                    None => {
                        panic!("Exiting");
                    }
                }
            }
        }

        if let Event::Key(key) = event::read().unwrap() {
            handle_area(key.code, state, &client).await;
            let next = handle_global(key.code);

            if !next{
                return Ok(());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = State::new();
    let client = ApiClient::new().await;

    let res = match client {
        Ok(c) => run_app(&mut terminal, &mut app, c).await,
        _ => panic!("Panic")
    };

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
