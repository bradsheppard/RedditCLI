use crossterm::event::{self, Event, KeyCode};

use crate::{state::{State, StatefulList}, api::ApiClient};

pub async fn handle_key_press(state: &mut State, client: &ApiClient) -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        if let KeyCode::Char('q') = key.code {
            return false;
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
                    state.subbreddits.next();
                }
                KeyCode::Up => {
                    state.subbreddits.previous();
                }
                _ => {}
            }

            return true;
        }
    }
    else {
        return false;
    }
}
