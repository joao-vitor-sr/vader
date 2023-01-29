use crate::app::App;
use crossterm::event::KeyCode;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        _ => {}
    }
}
