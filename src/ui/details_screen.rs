use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Paragraph, Wrap, ListItem, List},
    Frame, text::{Spans, Span}, style::{Style, Modifier, Color}
};

use crate::state::{Subreddit, Article, StatefulList};


pub fn draw_detail_screen<B: Backend>(f: &mut Frame<B>, details: &Subreddit, articles: &mut StatefulList<Article>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(20),
                Constraint::Length(100)
            ].as_ref()
        )
        .split(f.size());

    let help_line = Spans::from("q: Go Back");
    let help_paragraph = Paragraph::new(help_line);

    let meta_lines = vec![
        Spans::from(&*details.name),
        Spans::from("\n"),
        Spans::from("Subscriber Count: ".to_owned() + &details.subscriber_count.to_string()),
        Spans::from("\n"),
        Spans::from(&*details.description)
    ];
    let meta_block = Block::default()
        .title("Subreddit Info")
        .borders(Borders::ALL);
    let meta_paragraph = Paragraph::new(meta_lines)
        .block(meta_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let article_items: Vec<ListItem> = articles
        .items
        .iter()
        .map(|i| {
            let span = Span::styled(&i.title, Style::default().add_modifier(Modifier::ITALIC));
            ListItem::new(span)
        })
        .collect();
    let article_list = List::new(article_items)
        .block(Block::default().borders(Borders::ALL).title("Articles"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol(">> ");

    f.render_widget(help_paragraph, chunks[0]);
    f.render_widget(meta_paragraph, chunks[1]);
    f.render_stateful_widget(article_list, chunks[2], &mut articles.state);
}
