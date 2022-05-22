use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, ListItem, List},
    style::{Color, Style, Modifier},
    text::Span,
    Frame,
};

use crate::state::StatefulList;

pub fn draw_search_screen<B: Backend>(f: &mut Frame<B>, input_string: &str, subreddits: &mut StatefulList<String>) {
    let items: Vec<ListItem> = subreddits
        .items
        .iter()
        .map(|i| {
            let span = Span::styled(i, Style::default().add_modifier(Modifier::ITALIC));
            ListItem::new(span)
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Subreddits"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ].as_ref()
        )
        .split(f.size());

    let search_block = Block::default()
         .title("Search Subreddits")
         .borders(Borders::ALL);

    let input = Paragraph::new(input_string)
        .block(search_block);

    f.render_widget(input, chunks[0]);
    f.render_stateful_widget(items, chunks[1], &mut subreddits.state);
}

