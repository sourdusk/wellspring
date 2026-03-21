use tauri::{AppHandle, Emitter};

pub fn handle_deep_link(app: &AppHandle, urls: Vec<url::Url>) {
    for url in urls {
        log::info!("Deep link received: {}", url);
        let _ = app.emit("siyuan-open-url", url.to_string());
    }
}
