use anyhow::{bail, Result};
use std::{
    env::var,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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
