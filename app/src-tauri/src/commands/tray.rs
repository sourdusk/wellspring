use tauri::AppHandle;
use tauri::menu::{Menu, MenuItem};

#[tauri::command]
pub async fn config_tray(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    // Tray configuration from frontend
    // Build menu items from the data payload
    let languages = data.get("languages")
        .and_then(|v| v.as_object());

    let show_text = languages
        .and_then(|l| l.get("show"))
        .and_then(|v| v.as_str())
        .unwrap_or("Show");
    let quit_text = languages
        .and_then(|l| l.get("quit"))
        .and_then(|v| v.as_str())
        .unwrap_or("Quit");

    let show_item = MenuItem::with_id(&app, "show", show_text, true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let quit_item = MenuItem::with_id(&app, "quit", quit_text, true, None::<&str>)
        .map_err(|e| e.to_string())?;

    let menu = Menu::with_items(&app, &[&show_item, &quit_item])
        .map_err(|e| e.to_string())?;

    // Update or create tray
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    }

    Ok(())
}
