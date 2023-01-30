use crate::{
    mpc::{SongInfo, StatusInfo},
    utils::{self, return_entries},
};
use std::path::PathBuf;

pub struct App {
    pub tick_rate_milliseconds: u64,
    pub should_quit: bool,
    pub songs_root_path: PathBuf,
    pub current_song: SongInfo,
    pub status: StatusInfo,
    pub entries: Vec<EntrySong>,
    pub selected_song: Option<usize>,
    pub route: Route,
}

pub enum Route {
    ListSongs,
}

pub struct EntrySong {
    pub dir: Option<PathBuf>,
    pub song: Option<SongInfo>,
}

impl App {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl Default for App {
    fn default() -> App {
        let root_path = PathBuf::from(utils::return_songs_root_path().unwrap());
        App {
            selected_song: None,
            entries: return_entries(&root_path),
            status: StatusInfo::default(),
            current_song: SongInfo::default(),
            tick_rate_milliseconds: 250,
            should_quit: false,
            songs_root_path: root_path,
            route: Route::ListSongs,
        }
    }
}
