use crossterm::event::KeyCode;

use crate::{state::{State, Screen, StatefulList}, api::ApiClient};

pub async fn handle_detail_screen(key_code: KeyCode, state: &mut State, client: &ApiClient) {
    match key_code {
        KeyCode::Char('q') => {
            state.screen = Screen::Search;
        }
        KeyCode::Enter => {
            let selected_article_index = state.articles.state.selected();
            let selected_subreddit = &state.selected_subreddit;

            match (selected_article_index, selected_subreddit) {
                (Some(index), Some(subreddit)) => {
                    let selected_article = &state.articles.items[index];
                    let article_comments = client.get_article_comments(&subreddit.name, &selected_article.id).await;

                    match article_comments {
                        Ok(comments) => {
                            state.comments = StatefulList::with_items(comments);
                            state.screen = Screen::Article;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
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
