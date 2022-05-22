use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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

use api::ApiClient;
use state::{State, StatefulList};

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut state: State, client: ApiClient) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw_search_screen(f, &state.input, &mut state.subbreddits))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            else {
                match key.code {
                    KeyCode::Char(c) => {
                        state.input.push(c);
                    }
                    KeyCode::Backspace => {
                        state.input.pop();
                    }
                    KeyCode::Enter => {
                        let subbreddits = client.get_subreddits(&state.input).await;
                        state.subbreddits = StatefulList::with_items(subbreddits.unwrap());
                    }
                    KeyCode::Down => {
                        state.subbreddits.next()
                    }
                    KeyCode::Up => {
                        state.subbreddits.previous()
                    }
                    _ => {}
                }
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
    let app = State::new();
    let client = ApiClient::new().await;

    let res = match client {
        Ok(c) => run_app(&mut terminal, app, c).await,
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
