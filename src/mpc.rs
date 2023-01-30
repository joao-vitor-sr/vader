use crate::app::App;
use mpd::State;
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
pub async fn update_mpd(app: &Arc<Mutex<App>>) {
    loop {
        let mut app = app.lock().await;

        // Song
        let current_song = app.mpd_conn.currentsong().unwrap();

        if let Some(song) = current_song {
            app.current_song.file = song.file;
            app.current_song.title = song.title;
            let artist = song.tags.get("Artist");
            if let Some(artist) = artist {
                app.current_song.artist = Some(artist.to_owned());
            }
        }

        // Status
        let current_status = app.mpd_conn.status().unwrap();
        app.status.volume = current_status.volume;
        app.status.repeat = current_status.repeat;
        app.status.random = current_status.random;
        app.status.single = current_status.single;
        app.status.state = current_status.state;

        drop(app);
        thread::sleep(Duration::from_millis(800));
    }
}

pub struct StatusInfo {
    pub volume: i8,
    pub repeat: bool,
    pub random: bool,
    pub single: bool,
    pub state: State,
}

impl Default for StatusInfo {
    fn default() -> StatusInfo {
        StatusInfo {
            volume: 0,
            repeat: false,
            random: false,
            single: false,
            state: State::Stop,
        }
    }
}

#[derive(Clone)]
pub struct SongInfo {
    pub file: String,
    pub title: Option<String>,
    pub artist: Option<String>,
}

impl Default for SongInfo {
    fn default() -> SongInfo {
        SongInfo {
            file: "".to_string(),
            title: None,
            artist: None,
        }
    }
}
