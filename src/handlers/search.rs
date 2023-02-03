use crate::app::App;
use crossterm::event::KeyCode;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char(c) => {
            app.search_input.push(c);
        }
        KeyCode::Backspace => {
            app.search_input.pop();
        }
        KeyCode::Enter => {
            exec_search(app, false);
        }
        KeyCode::Esc => {
            app.search_input = "".to_string();
            app.search_mode = !app.search_mode;
        }
        _ => {}
    }
}

pub fn exec_search(app: &mut App, rev: bool) {
    let search_input = app.search_input.clone();
    app.search_entry_by_text(&search_input, rev);
    app.search_mode = false;
}
