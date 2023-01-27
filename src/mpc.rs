use crate::app::App;
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
pub async fn update_mpd(app: &Arc<Mutex<App>>) {
    loop {
        let mut app = app.lock().await;

        thread::sleep(Duration::from_millis(800));
    }
}
