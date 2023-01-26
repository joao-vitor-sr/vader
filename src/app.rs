use crossterm::event::KeyCode;

pub struct App {
    pub tick_rate_milliseconds: u64,
    pub should_quit: bool,
    pub temp_alternate_state: bool,
}

impl App {
    pub fn handle_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn new() -> Self {
        Self { ..Self::default() }
    }
}

impl Default for App {
    fn default() -> App {
        App {
            tick_rate_milliseconds: 250,
            should_quit: false,
            temp_alternate_state: true,
        }
    }
}
