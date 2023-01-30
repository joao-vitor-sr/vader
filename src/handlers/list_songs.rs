use crossterm::event::KeyCode;

use crate::{app::App, utils};

use super::common_keys;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        k if common_keys::right_event(k) => {
            if let Some(i) = app.selected_entry {
                match &app.entries[i].dir {
                    Some(dir) => {
                        app.parent_path = dir.clone();
                        app.entries = utils::return_entries(&dir);
                        app.depth = app.depth + 1;
                        app.parents_indeces.push(i as u32);
                        app.selected_entry = None;
                    }
                    None => {
                        app.push_song_from_entries(i);
                    }
                }
            }
        }
        k if common_keys::left_event(k) => {
            if app.depth == 0 {
                return;
            }

            app.parent_path = app.parent_path.parent().unwrap().clone().to_path_buf();
            app.entries = utils::return_entries(&app.parent_path);
            app.depth = app.depth - 1;
            app.selected_entry = Some(app.parents_indeces.last().unwrap().clone() as usize);
            app.parents_indeces.pop();
        }

        k if common_keys::down_event(k) => {
            match app.selected_entry {
                None => {
                    app.selected_entry = Some(0);
                }
                Some(i) => {
                    let next_i = i + 1;
                    if next_i == app.entries.len() {
                        app.selected_entry = Some(0);
                    } else {
                        app.selected_entry = Some(next_i);
                    }
                }
            };
        }
        k if common_keys::up_event(k) => {
            let l = app.entries.len();
            match app.selected_entry {
                None => {
                    app.selected_entry = Some(l - 1);
                }
                Some(i) => {
                    if i > 0 {
                        app.selected_entry = Some(i - 1);
                    } else {
                        app.selected_entry = Some(l - 1);
                    }
                }
            };
        }
        _ => {}
    }
}
