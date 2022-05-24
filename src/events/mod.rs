mod search_screen;
mod details_screen;

use crossterm::event::KeyCode;

use search_screen::handle_search_screen;
use details_screen::handle_detail_screen;

use crate::{api::ApiClient, state::{State, Screen}};


pub async fn handle_area(key: KeyCode, state: &mut State, client: &ApiClient) {
    match state.screen {
        Screen::Search => {
            handle_search_screen(key, state, client).await;
        }
        Screen::Details => {
            handle_detail_screen(key, state, client).await;
        }
    }
}

pub fn handle_global(key: KeyCode) -> bool {
    match key {
        KeyCode::Esc => {
            false
        }
        _ => {
            true
        }
    }
}
