use crossterm::event::KeyCode;

pub fn down_event(key: KeyCode) -> bool {
    matches!(key, KeyCode::Down | KeyCode::Char('j'))
}

pub fn up_event(key: KeyCode) -> bool {
    matches!(key, KeyCode::Up | KeyCode::Char('k'))
}
