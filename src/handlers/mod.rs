mod common_keys;
mod list_queue;
mod list_songs;
pub mod search;

use crate::app::{App, Route};
use crossterm::event::KeyCode;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('/') => {
            app.search_input = "".to_string();
            app.search_mode = true;
        }
        KeyCode::Char('1') => {
            app.route = Route::ListSongs;
        }
        KeyCode::Char('2') => {
            app.route = Route::ListQueue;
        }
        _ => handle_block(key, app),
    }
}

fn handle_block(key: KeyCode, app: &mut App) {
    match app.route {
        Route::ListSongs => list_songs::handler(key, app),
        Route::ListQueue => list_queue::handler(key, app)
    }
}
