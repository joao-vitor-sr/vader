use crossterm::event::KeyCode;

use crate::app::App;

use super::common_keys;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        k if common_keys::down_event(k) => {
            match app.selected_song {
                None => {
                    app.selected_song = Some(0);
                }
                Some(i) => {
                    let next_i = i + 1;
                    if next_i == app.entries.len() {
                        app.selected_song = Some(0);
                    } else {
                        app.selected_song = Some(next_i);
                    }
                }
            };
        }
        k if common_keys::up_event(k) => {
            match app.selected_song {
                None => {
                    app.selected_song = Some(app.entries.len() - 1);
                }
                Some(i) => {
                    if i > 0 {
                        app.selected_song = Some(i - 1);
                    } else {
                        app.selected_song = Some(app.entries.len() - 1);
                    }
                }
            };
        }
        _ => {}
    }
}
