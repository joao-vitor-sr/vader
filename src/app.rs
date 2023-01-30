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
    pub selected_entry: Option<usize>,
    pub route: Route,
    pub depth: u8,
    pub parents_indeces: Vec<u32>,
    pub parent_path: PathBuf,
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
            parents_indeces: vec![],
            selected_entry: None,
            entries: return_entries(&root_path),
            status: StatusInfo::default(),
            current_song: SongInfo::default(),
            tick_rate_milliseconds: 250,
            should_quit: false,
            songs_root_path: root_path.clone(),
            route: Route::ListSongs,
            depth: 0,
            parent_path: root_path,
        }
    }
}
