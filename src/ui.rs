use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_main<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(92), Constraint::Percentage(8)].as_ref())
        .split(f.size());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(94), Constraint::Min(0)].as_ref())
        .split(chunks[0]);

    draw_list_songs(f, app, horizontal_chunks[0]);
    draw_mpd_info(f, app, horizontal_chunks[1]);
    draw_info_song(f, app, chunks[1]);
}

fn draw_list_songs<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let no_song_message = Span::from("No song found");
    let mut items = vec![];

    if app.entries.is_empty() {
        items.push(ListItem::new(no_song_message));
    } else {
        for entry in &app.entries {
            if let Some(dir) = &entry.dir {
                items.push(ListItem::new(Span::styled(
                    dir.file_name().unwrap().to_str().unwrap(),
                    Style::default().fg(Color::Yellow),
                )));
            } else if let Some(song) = &entry.song {
                items.push(ListItem::new(Span::styled(
                    match &song.title {
                        Some(title) => title,
                        None => &app.current_song.file,
                    },
                    Style::default().fg(Color::Green),
                )));
            }
        }
    }

    let list = List::new(items).block(Block::default().borders(Borders::ALL));
    f.render_widget(list, chunk);
}

fn draw_info_song<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let text = vec![Spans::from(vec![
        Span::raw(match &app.current_song.artist {
            Some(artist) => artist,
            None => "Unknown",
        }),
        Span::from(" - "),
        Span::raw(match &app.current_song.title {
            Some(title) => title,
            None => &app.current_song.file,
        }),
    ])];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, chunk);
}

fn draw_mpd_info<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let mut text = vec![];

    text.push(Spans::from(format!("{}%", app.status.volume)));

    if app.status.single == true {
        text.push(Spans::from("Single"));
    }

    if app.status.repeat == true {
        text.push(Spans::from("Repeat"));
    }

    if app.status.random == true {
        text.push(Spans::from("Random"));
    }

    match app.status.state {
        mpd::State::Play => {
            text.push(Spans::from("Play"));
        }
        mpd::State::Stop => {
            text.push(Spans::from("Stop"));
        }
        mpd::State::Pause => {
            text.push(Spans::from("Pause"));
        }
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunk);
}
