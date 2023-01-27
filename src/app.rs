use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::utils;

pub struct App {
    pub tick_rate_milliseconds: u64,
    pub should_quit: bool,
    pub songs_root_path: PathBuf,
}

impl App {
    pub fn handle_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl Default for App {
    fn default() -> App {
        App {
            tick_rate_milliseconds: 250,
            should_quit: false,
            songs_root_path: PathBuf::from(utils::return_songs_root_path().unwrap()),
        }
    }
}
