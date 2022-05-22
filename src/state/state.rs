use tui::widgets::ListState;

pub struct State {
    pub input: String,
    pub subbreddits: StatefulList<String>,
    pub subbreddit_details: Option<SubredditDetail>
}

impl State {
    pub fn new() -> Self {
        State { 
            input: String::new(), 
            subbreddits: StatefulList::with_items(vec![]),
            subbreddit_details: None
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
