use tui::Frame;
use tui::layout::{Direction, Constraint, Alignment, Layout};

use tui::text::{Spans, Span};
use tui::widgets::{Paragraph, Block, Borders, Wrap};
use tui::backend::Backend;

use crate::state::{Article, StatefulList, Comment};

pub fn draw_article_screen<B: Backend>(f: &mut Frame<B>, article: &Article, comments: &StatefulList<Comment>,
                                       scroll: &mut u16) {
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

    recurse_comments(&mut comment_items, &comments.items, 0);

    let comment_block = Block::default()
        .title("Comments")
        .borders(Borders::ALL);
    let comment_paragraph = Paragraph::new(comment_items)
        .block(comment_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .scroll((*scroll, 0));

    f.render_widget(help_paragraph, chunks[0]);
    f.render_widget(meta_paragraph, chunks[1]);
    f.render_widget(comment_paragraph, chunks[2]);
}

fn recurse_comments<'a>(spans: &mut Vec<Spans<'a>>, comments: &'a Vec<Comment>, depth: u16) {
    for comment in comments {
        let body = comment.body.to_owned();
        let mut spacing = "".to_owned();

        for _ in 0..depth {
            spacing.push_str(" - ");
        }

        let spacing_span = Span::raw(spacing);
        let body_span = Span::raw(body);

        let text = Spans::from(vec![spacing_span, body_span]);
        let space = Spans::from("");

        spans.push(text);
        spans.push(space);

        recurse_comments(spans, &comment.replies, depth + 1);
    }
}
