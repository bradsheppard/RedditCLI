use tui::Frame;
use tui::layout::{Direction, Constraint, Alignment, Layout};
use tui::style::{Style, Modifier, Color};
use tui::text::{Spans, Span};
use tui::widgets::{Paragraph, Block, Borders, Wrap, ListItem, List};
use tui::backend::Backend;

use crate::state::{Article, Comment, StatefulList};

pub fn draw_article_screen<B: Backend>(f: &mut Frame<B>, article: &Article, comments: &mut StatefulList<Comment>) {
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
        Spans::from(&*article.title)
    ];
    let meta_block = Block::default()
        .title("Article Info")
        .borders(Borders::ALL);
    let meta_paragraph = Paragraph::new(meta_lines)
        .block(meta_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let comment_items: Vec<ListItem> = comments
        .items
        .iter()
        .map(|i| {
            let span = Span::styled(&i.body, Style::default().add_modifier(Modifier::ITALIC));
            ListItem::new(span)
        })
        .collect();
    let comment_list = List::new(comment_items)
        .block(Block::default().borders(Borders::ALL).title("Comments"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol(">> ");

    f.render_widget(help_paragraph, chunks[0]);
    f.render_widget(meta_paragraph, chunks[1]);
    f.render_stateful_widget(comment_list, chunks[2], &mut comments.state);
}
