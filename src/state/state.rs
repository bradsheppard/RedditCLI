use tui::widgets::ListState;

use super::Comment;
use super::Subreddit;
use super::Article;


pub struct State {
    pub input: String,
    pub subbreddits: StatefulList<String>,
    pub selected_subreddit: Option<Subreddit>,
    pub articles: StatefulList<Article>,
    pub screen: Screen,
    pub comments: StatefulList<Comment>
}

pub enum Screen {
    Search,
    Details,
    Article
}

impl State {
    pub fn new() -> Self {
        State { 
            input: String::new(), 
            subbreddits: StatefulList::with_items(vec![]),
            selected_subreddit: None,
            articles: StatefulList::with_items(vec![]),
            screen: Screen::Search,
            comments: StatefulList::with_items(vec![])
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
