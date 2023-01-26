use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;
use crate::app::App;

#[tokio::main]
pub async fn update_mpd(app: &Arc<Mutex<App>>) {
    loop {
        let mut app = app.lock().await;

        app.temp_alternate_state = !app.temp_alternate_state;
        thread::sleep(Duration::from_millis(800));
    }
}
