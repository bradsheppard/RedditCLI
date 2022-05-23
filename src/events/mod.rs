mod handler;

use crossterm::event::KeyCode;
use handler::handle_search_screen;

use crate::{api::ApiClient, state::{State, Screen}};


pub fn handle_area(key: KeyCode, state: &mut State, client: &ApiClient) {
    match state.screen {
        Screen::Search => {
            handle_search_screen(key, state, client);
        }
        _ => {}
    }
}
