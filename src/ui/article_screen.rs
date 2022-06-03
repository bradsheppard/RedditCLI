use tui::Frame;
use tui::layout::{Direction, Constraint, Alignment, Layout};

use tui::text::Spans;
use tui::widgets::{Paragraph, Block, Borders, Wrap};
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

    let mut comment_items = Vec::new();

    for comment in &comments.items {
        let text = Spans::from(&*comment.body);
        let space = Spans::from("");

        comment_items.push(text);
        comment_items.push(space);
    }

    let comment_block = Block::default()
        .title("Comments")
        .borders(Borders::ALL);
    let comment_paragraph = Paragraph::new(comment_items)
        .block(comment_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, chunks[0]);
    f.render_widget(meta_paragraph, chunks[1]);
    f.render_widget(comment_paragraph, chunks[2]);
}
