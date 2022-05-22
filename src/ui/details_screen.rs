use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame
};

pub fn draw_detail_screen<B: Backend>(f: &mut Frame<B>) {
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

    let details_block = Block::default()
        .title("Subreddit Details")
        .borders(Borders::ALL);
    let details_paragraph = Paragraph::new("TEST STRING")
        .block(details_block);

    f.render_widget(details_paragraph, chunks[0]);
}
