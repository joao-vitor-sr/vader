use mpd::Client;

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
    pub mpd_conn: Client,
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

    pub fn push_song_from_entries(&mut self, index: usize) {
        let mut song: Option<SongInfo> = None;
        match &self.entries[index].song {
            Some(song_info) => {
                song = Some(song_info.clone());
            }
            None => {}
        }

        if let Some(song) = song {
            self.push_song(&song);
        }
    }

    pub fn push_song(&mut self, song: &SongInfo) {
        let song = utils::convert_song_info_into_song(&song);
        self.mpd_conn.push(song).unwrap();
    }
}

impl Default for App {
    fn default() -> App {
        let root_path = PathBuf::from(utils::return_songs_root_path().unwrap());
        App {
            mpd_conn: Client::connect("127.0.0.1:6600").unwrap(),
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
