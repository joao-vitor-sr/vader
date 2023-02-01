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
        KeyCode::Esc => {
            app.search_input = "".to_string();
            app.search_mode = !app.search_mode;
        }
        _ => {}
    }
}
