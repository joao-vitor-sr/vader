use crossterm::event::KeyCode;

pub fn down_event(key: KeyCode) -> bool {
    matches!(key, KeyCode::Down | KeyCode::Char('j'))
}

pub fn up_event(key: KeyCode) -> bool {
    matches!(key, KeyCode::Up | KeyCode::Char('k'))
}

pub fn right_event(key: KeyCode) -> bool {
    matches!(key, KeyCode::Right | KeyCode::Char('l'))
}

pub fn left_event(key: KeyCode) -> bool {
    matches!(key, KeyCode::Left | KeyCode::Char('h'))
}
