use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, text::Spans
};

use crate::state::SubredditDetail;


pub fn draw_detail_screen<B: Backend>(f: &mut Frame<B>, details: &SubredditDetail) {
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
        Spans::from(&*details.description)
    ];
    let meta_block = Block::default()
        .title("Subreddit Info")
        .borders(Borders::ALL);
    let meta_paragraph = Paragraph::new(meta_lines)
        .block(meta_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, chunks[0]);
    f.render_widget(meta_paragraph, chunks[1]);
}
