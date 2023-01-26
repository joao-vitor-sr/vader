use tui::{backend::Backend, Frame, widgets::{Block, Borders}};

use crate::app::App;

pub fn draw_main<B: Backend>(f: &mut Frame<B>, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(match app.temp_alternate_state {
            true => "yes",
            false => "no",
        });
    f.render_widget(block, f.size());
}
