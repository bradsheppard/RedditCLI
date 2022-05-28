use crossterm::event::KeyCode;

use crate::{state::{State, Screen}, api::ApiClient};

pub async fn handle_detail_screen(key_code: KeyCode, state: &mut State, client: &ApiClient) {
    match key_code {
        KeyCode::Char('q') => {
            state.screen = Screen::Search;
        }
        KeyCode::Down => {
            state.articles.next();
        }
        KeyCode::Up => {
            state.articles.previous();
        }
        _ => {}
    }
}
