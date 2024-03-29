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

pub fn on_down_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                let next_index = selection_index + 1;
                if next_index > selection_data.len() - 1 {
                    return 0;
                } else {
                    return next_index;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_up_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                if selection_index > 0 {
                    return selection_index - 1;
                } else {
                    return selection_data.len() - 1;
                }
            }
            0
        }
        None => 0,
    }
}
