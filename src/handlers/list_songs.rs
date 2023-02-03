use crossterm::event::KeyCode;

use crate::{app::App, utils};

use super::{common_keys, search::exec_search};

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('n') => {
            exec_search(app, false);
        }
        KeyCode::Char('N') => {
            exec_search(app, true);
        }
        k if common_keys::right_event(k) => match &app.entries[app.selected_entry].dir {
            Some(dir) => {
                app.parent_path = dir.clone();
                app.entries = utils::return_entries(&dir);
                app.depth = app.depth + 1;
                app.parents_indeces.push(app.selected_entry as u32);
                app.selected_entry = 0;
            }
            None => {
                app.push_song_from_entries(app.selected_entry);
            }
        },
        k if common_keys::left_event(k) => {
            if app.depth == 0 {
                return;
            }

            app.parent_path = app.parent_path.parent().unwrap().clone().to_path_buf();
            app.entries = utils::return_entries(&app.parent_path);
            app.depth = app.depth - 1;
            app.selected_entry = app.parents_indeces.last().unwrap().clone() as usize;
            app.parents_indeces.pop();
        }

        k if common_keys::down_event(k) => {
            let next_i = app.selected_entry + 1;
            if next_i == app.entries.len() {
                app.selected_entry = 0;
            } else {
                app.selected_entry = next_i;
            }
        }
        k if common_keys::up_event(k) => {
            let l = app.entries.len();
            if app.selected_entry > 0 {
                app.selected_entry = app.selected_entry - 1;
            } else {
                app.selected_entry = l - 1;
            }
        }
        _ => {}
    }
}
