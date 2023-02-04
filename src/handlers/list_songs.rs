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
                app.parent_path = dir.to_owned();
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

            app.parent_path = app.parent_path.parent().unwrap().to_path_buf();
            app.entries = utils::return_entries(&app.parent_path);
            app.depth = app.depth - 1;
            app.selected_entry = app.parents_indeces.last().unwrap().to_owned() as usize;
            app.parents_indeces.pop();
        }

        k if common_keys::down_event(k) => {
            app.selected_entry =
                common_keys::on_down_press_handler(&app.entries, Some(app.selected_entry));
        }
        k if common_keys::up_event(k) => {
            app.selected_entry =
                common_keys::on_up_press_handler(&app.entries, Some(app.selected_entry));
        }
        _ => {}
    }
}
