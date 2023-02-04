use super::common_keys;
use crate::app::App;
use crossterm::event::KeyCode;

pub fn handler(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('c') => {
            app.clear_queue();
        }
        k if common_keys::down_event(k) => {
            app.selected_queue =
                common_keys::on_down_press_handler(&app.queue, Some(app.selected_queue));
        }
        k if common_keys::up_event(k) => {
            app.selected_queue =
                common_keys::on_up_press_handler(&app.queue, Some(app.selected_queue));
        }
        _ => {}
    }
}
