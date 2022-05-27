use crossterm::event::KeyCode;

use crate::{state::{State, StatefulList, Screen}, api::ApiClient};

pub async fn handle_search_screen(key_code: KeyCode, state: &mut State, client: &ApiClient) {
    match key_code {
        KeyCode::Char(c) => {
            state.input.push(c);
        }
        KeyCode::Backspace => {
            state.input.pop();
        }
        KeyCode::Enter => {
            let selected = state.subbreddits.state.selected();

            match selected {
                None => {
                    let subbreddits = client.get_subreddits(&state.input).await;
                    state.subbreddits = StatefulList::with_items(subbreddits.unwrap());
                }
                Some(index) => {
                    let selected_subreddit_name = &state.subbreddits.items[index];
                    let selected_subreddit_details = client.get_subreddit_details(selected_subreddit_name).await;
                    let articles = client.get_subreddit_articles(selected_subreddit_name).await;

                    match selected_subreddit_details {
                        Ok(details) => {
                            state.selected_subreddit = Some(details);
                            state.screen = Screen::Details;
                        }
                        Err(error) => {
                            state.input = error.to_string();
                        }
                    }

                    match articles {
                        Ok(a) => {
                            state.articles = StatefulList::with_items(a);
                        }
                        Err(error) => {
                            state.input = error.to_string();
                        }
                    }
                }
            }
        }
        KeyCode::Down => {
            state.subbreddits.next();
        }
        KeyCode::Up => {
            state.subbreddits.previous();
        }
        _ => {}
    }
}
