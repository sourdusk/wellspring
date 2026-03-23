use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn cmd_notification(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    let title = data.get("title").and_then(|v| v.as_str()).unwrap_or("Wellspring");
    let body = data.get("body").and_then(|v| v.as_str()).unwrap_or("");

    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_open_path(app: AppHandle, data: serde_json::Value) -> Result<(), String> {
    let path = data.get("filePath").and_then(|v| v.as_str()).unwrap_or("");

    // Reject known executable/script extensions as a defense-in-depth measure
    let lower = path.to_lowercase();
    let blocked_extensions = [
        ".exe", ".bat", ".cmd", ".ps1", ".vbs", ".vbe", ".jse",
        ".wsf", ".wsh", ".msi", ".msp", ".scr", ".com", ".pif", ".hta",
        ".cpl", ".inf", ".reg",
    ];
    for ext in &blocked_extensions {
        if lower.ends_with(ext) {
            return Err(format!("Opening files with '{}' extension is not allowed", ext));
        }
    }

    app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_show_item_in_folder(app: AppHandle, data: serde_json::Value) -> Result<(), String> {
    let path = data.get("filePath").and_then(|v| v.as_str()).unwrap_or("");
    app.opener().reveal_item_in_dir(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_open_dev_tools(window: tauri::Window) -> Result<(), String> {
    if let Some(webview) = window.get_webview_window(window.label()) {
        webview.open_devtools();
    }
    Ok(())
}

#[tauri::command]
pub async fn cmd_clear_cache(_window: tauri::Window) -> Result<(), String> {
    // Tauri v2 doesn't expose cache clearing directly
    log::info!("clearCache requested (no-op in Tauri)");
    Ok(())
}

#[tauri::command]
pub async fn cmd_undo(_window: tauri::Window) -> Result<(), String> {
    // Handled by webview natively
    Ok(())
}

#[tauri::command]
pub async fn cmd_redo(_window: tauri::Window) -> Result<(), String> {
    // Handled by webview natively
    Ok(())
}

#[tauri::command]
pub async fn cmd_write_log(data: serde_json::Value) -> Result<(), String> {
    let msg = data.get("msg").and_then(|v| v.as_str()).unwrap_or("");
    log::info!("[frontend] {}", msg);
    Ok(())
}

#[tauri::command]
pub async fn cmd_set_spell_checker_languages() -> Result<(), String> {
    // Spell checker is managed by the webview natively in Tauri
    log::info!("setSpellCheckerLanguages requested (webview-managed in Tauri)");
    Ok(())
}

#[tauri::command]
pub async fn get_available_spell_checker_languages() -> Result<Vec<String>, String> {
    // WebView2 manages spell checking natively; return empty list
    // TODO: query WebView2 for available languages if needed
    Ok(vec![])
}

#[tauri::command]
pub async fn cmd_set_traffic_light_position() -> Result<(), String> {
    // macOS-only, Tauri handles via config
    Ok(())
}

#[tauri::command]
pub async fn register_window_events() -> Result<(), String> {
    // Window events are handled via Tauri's built-in event system
    Ok(())
}
