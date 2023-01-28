use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_main<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(92), Constraint::Percentage(8)].as_ref())
        .split(f.size());

    draw_list_songs(f, app, chunks[0]);
    draw_status_bar(f, app, chunks[1]);
}

fn draw_list_songs<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Songs");
    f.render_widget(block, chunk);
}

fn draw_status_bar<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(chunk);

    f.render_widget(return_current_song_info(app, Alignment::Left), chunks[0]);
    f.render_widget(return_current_song_status(app, Alignment::Right), chunks[1]);
}

fn return_current_song_status<'a>(app: &App, alignment: Alignment) -> Paragraph<'a> {
    let text = vec![Spans::from("Duration")];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(alignment);

    paragraph
}

fn return_current_song_info<'a>(app: &App, alignment: Alignment) -> Paragraph<'a> {
    let text = vec![Spans::from(vec![
        Span::from("Artist"),
        Span::from("-"),
        Span::from("current song"),
    ])];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(alignment);

    paragraph
}
