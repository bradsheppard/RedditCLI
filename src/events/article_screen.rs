use crossterm::event::KeyCode;

use crate::state::{State, Screen};

const SCROLL_OFFSET: u16 = 10;

pub fn handle_article_screen(key_code: KeyCode, state: &mut State) {
    match key_code {
        KeyCode::Char('q') => {
            state.screen = Screen::Details;
        }
        KeyCode::Down => {
            state.scroll = state.scroll + SCROLL_OFFSET;
        }
        KeyCode::Up => {
            if state.scroll >= SCROLL_OFFSET {
                state.scroll = state.scroll - SCROLL_OFFSET;
            }
        }
        _ => {}
    }
}
