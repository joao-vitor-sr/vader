pub struct App {
    pub tick_rate_milliseconds: u64,
    pub should_quit: bool,
    pub temp_alternate_state: bool,
}

impl App {
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
