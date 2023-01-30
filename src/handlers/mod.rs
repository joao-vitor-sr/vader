mod list_songs;
mod common_keys;

use crate::app::{App, Route};
use crossterm::event::KeyCode;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        _ => handle_block(key, app),
    }
}

fn handle_block(key: KeyCode, app: &mut App) {
    match app.route {
        Route::ListSongs => list_songs::handler(key, app),
    }
}
