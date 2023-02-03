use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
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

    // draw_list_songs(f, app, horizontal_chunks[0]);
    draw_songs_table(f, app, horizontal_chunks[0]);
    draw_mpd_info(f, app, horizontal_chunks[1]);

    if app.search_mode {
        draw_search_input(f, app, chunks[1]);
    } else {
        draw_info_song(f, app, chunks[1]);
    }
}

fn draw_songs_table<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let mut lines = vec![];
    for entry in &app.entries {
        let mut first_field = "";
        let mut second_field = "";

        match &entry.song {
            Some(song) => {
                match &song.title {
                    Some(title) => {
                        first_field = title;
                    }
                    None => {
                        first_field = &song.file;
                    }
                };

                match &song.artist {
                    Some(artist) => {
                        second_field = artist;
                    }
                    None => {}
                };
            }
            None => {}
        };

        match &entry.dir {
            Some(dir) => {
                first_field = dir.file_name().unwrap().to_str().unwrap();
            }
            None => {}
        };

        lines.push([first_field, second_field]);
    }

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let rows = lines.iter().map(|line| {
        let height = line
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = line.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16)
    });

    let t = Table::new(rows)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

    let mut state = TableState::default();
    state.select(Some(app.selected_entry));
    f.render_stateful_widget(t, chunk, &mut state);
}

fn draw_search_input<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
    let input =
        Paragraph::new(app.search_input.as_ref()).block(Block::default().borders(Borders::ALL));
    f.set_cursor(chunk.x + app.search_input.len() as u16 + 1, chunk.y + 1);
    f.render_widget(input, chunk);
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
