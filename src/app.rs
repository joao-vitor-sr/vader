use std::path::PathBuf;

use crossterm::event::KeyCode;

use crate::{
    mpc::{SongInfo, StatusInfo},
    utils,
};

pub struct App {
    pub tick_rate_milliseconds: u64,
    pub should_quit: bool,
    pub songs_root_path: PathBuf,
    pub current_song: SongInfo,
    pub status: StatusInfo,
}

impl App {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl Default for App {
    fn default() -> App {
        App {
            status: StatusInfo::default(),
            current_song: SongInfo::default(),
            tick_rate_milliseconds: 250,
            should_quit: false,
            songs_root_path: PathBuf::from(utils::return_songs_root_path().unwrap()),
        }
    }
}
