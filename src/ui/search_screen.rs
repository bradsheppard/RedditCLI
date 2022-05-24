use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, ListItem, List},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    Frame,
};

use crate::state::StatefulList;

pub fn draw_search_screen<B: Backend>(f: &mut Frame<B>, input_string: &str, subreddits: &mut StatefulList<String>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(5),
                Constraint::Length(40)
            ].as_ref()
        )
        .split(f.size());

   let help_line = Spans::from("ESC: To Exit");
   let help_paragraph = Paragraph::new(help_line);

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

    let search_block = Block::default()
         .title("Search Subreddits")
         .borders(Borders::ALL);

    let input = Paragraph::new(input_string)
        .block(search_block);

    f.render_widget(help_paragraph, chunks[0]);
    f.render_widget(input, chunks[1]);
    f.render_stateful_widget(items, chunks[2], &mut subreddits.state);
}

