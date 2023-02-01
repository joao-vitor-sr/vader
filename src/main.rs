mod app;
mod handlers;
mod mpc;
mod ui;
mod utils;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use handlers::handler;
use mpc::update_mpd;
use std::{io, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialise app state
    let app = Arc::new(Mutex::new(App::new()));

    let cloned_app = Arc::clone(&app);
    std::thread::spawn(move || {
        update_mpd(&app);
    });

    start_ui(&cloned_app).await?;
    Ok(())
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &Arc<Mutex<App>>) -> Result<()> {
    loop {
        let mut app = app.lock().await;

        terminal.draw(|f| ui::draw_main(f, &app))?;
        let timeout = Duration::from_millis(app.tick_rate_milliseconds);
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if app.search_mode {
                    handlers::search::handler(key.code, &mut app);
                } else {
                    handler(key.code, &mut app);
                }
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
