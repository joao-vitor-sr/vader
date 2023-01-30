use anyhow::{bail, Result};
use audiotags::Tag;
use mpd::Song;
use std::{
    collections::BTreeMap,
    env::var,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{app::EntrySong, mpc::SongInfo};

pub fn return_song_from_path(path: &Path) -> Option<SongInfo> {
    if path.is_dir() {
        return None;
    }

    let tag = Tag::default().read_from_path(&path);

    if let Err(_) = tag {
        return None;
    }

    let tag = tag.unwrap();
    let song = SongInfo {
        file: path.to_str().unwrap().to_string(),
        title: Some(tag.title().unwrap().to_string()),
        artist: Some(tag.artist().unwrap().to_string()),
    };
    Some(song)
}

pub fn return_entries(path: &Path) -> Vec<EntrySong> {
    let mut entries = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        if entry_path.file_name().unwrap().to_str().unwrap()[0..1] == "."[..] {
            continue;
        }

        if entry_path.is_dir() {
            entries.push(EntrySong {
                dir: Some(entry_path),
                song: None,
            });
        } else {
            entries.push(EntrySong {
                dir: None,
                song: return_song_from_path(&entry_path),
            });
        }
    }
    entries
}

pub fn return_songs_root_path() -> Result<String> {
    let bail_msg = "Unable to get mpd path";

    let prefix =
        var("XDG_CONFIG_HOME").unwrap_or_else(|_| [&var("HOME").unwrap(), ".config"].join("/"));
    let mpd_conf_path = Path::new(&[&prefix, "mpd", "mpd.conf"].join("/")).to_owned();
    let mpd_conf = File::open(mpd_conf_path)?;

    for x in BufReader::new(mpd_conf).lines().flatten() {
        if x.starts_with("music_directory") {
            let value_vec = x.split_whitespace().collect::<Vec<_>>();
            let mut value = value_vec[1].to_owned();
            value.remove(0);
            value.remove(value.len() - 1);
            if value.starts_with('/') {
                return Ok(value.to_owned());
            } else if value.starts_with('~') {
                value.remove(0);
                return Ok([var("HOME").unwrap(), value].join(""));
            } else {
                bail!(bail_msg)
            }
        }
    }

    bail!(bail_msg)
}

pub fn convert_song_info_into_song(song_info: &SongInfo) -> Song {
    let mut tags = BTreeMap::new();
    tags.insert(
        "Artist".to_string(),
        match &song_info.artist {
            Some(artist) => artist.clone(),
            None => "".to_string(),
        },
    );
    let song = Song {
        file: song_info.file.clone(),
        name: None,
        title: song_info.title.clone(),
        last_mod: None,
        duration: None,
        place: None,
        range: None,
        tags,
    };
    song
}
