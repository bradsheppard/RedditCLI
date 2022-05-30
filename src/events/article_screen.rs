use crossterm::event::KeyCode;

use crate::state::{State, Screen};


pub fn handle_article_screen(key_code: KeyCode, state: &mut State) {
    match key_code {
        KeyCode::Char('q') => {
            state.screen = Screen::Details;
        }
        KeyCode::Down => {
            state.comments.next();
        }
        KeyCode::Up => {
            state.comments.previous();
        }
        _ => {}
    }
}
