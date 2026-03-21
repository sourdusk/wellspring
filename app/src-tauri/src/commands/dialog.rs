use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

#[tauri::command]
pub async fn show_confirm_dialog(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<bool, String> {
    let title = data.get("title").and_then(|v| v.as_str()).unwrap_or("SiYuan");
    let message = data.get("message").and_then(|v| v.as_str()).unwrap_or("");

    let confirmed = app.dialog()
        .message(message)
        .title(title)
        .buttons(MessageDialogButtons::OkCancelCustom("OK".into(), "Cancel".into()))
        .blocking_show();

    Ok(confirmed)
}

#[tauri::command]
pub async fn show_alert_dialog(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    let title = data.get("title").and_then(|v| v.as_str()).unwrap_or("SiYuan");
    let message = data.get("message").and_then(|v| v.as_str()).unwrap_or("");

    app.dialog()
        .message(message)
        .title(title)
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();

    Ok(())
}

#[tauri::command]
pub async fn get_show_open_dialog(
    app: AppHandle,
    _data: serde_json::Value,
) -> Result<Option<String>, String> {
    let result = app.dialog()
        .file()
        .blocking_pick_file();

    Ok(result.map(|f| f.to_string()))
}

#[tauri::command]
pub async fn get_show_save_dialog(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<Option<String>, String> {
    let default_name = data.get("defaultPath")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let result = app.dialog()
        .file()
        .set_file_name(default_name)
        .blocking_save_file();

    Ok(result.map(|f| f.to_string()))
}

#[tauri::command]
pub async fn show_context_menu(
    window: tauri::Window,
    data: serde_json::Value,
) -> Result<(), String> {
    let items = data.get("items")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let app = window.app_handle();
    let menu = build_menu_from_json(app, &items)?;
    window.popup_menu(&menu).map_err(|e| e.to_string())?;
    Ok(())
}

fn build_menu_from_json(
    app: &tauri::AppHandle,
    items: &[serde_json::Value],
) -> Result<tauri::menu::Menu<tauri::Wry>, String> {
    use tauri::menu::*;

    let mut menu_items: Vec<Box<dyn IsMenuItem<tauri::Wry>>> = Vec::new();

    for item in items {
        if item.get("type").and_then(|v| v.as_str()) == Some("separator") {
            menu_items.push(Box::new(
                PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?
            ));
            continue;
        }

        let label = item.get("label").and_then(|v| v.as_str()).unwrap_or("");
        let id = item.get("id").and_then(|v| v.as_str()).unwrap_or(label);
        let enabled = item.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

        let menu_item = MenuItem::with_id(app, id, label, enabled, None::<&str>)
            .map_err(|e| e.to_string())?;
        menu_items.push(Box::new(menu_item));
    }

    let refs: Vec<&dyn IsMenuItem<tauri::Wry>> = menu_items.iter()
        .map(|i| i.as_ref())
        .collect();

    Menu::with_items(app, &refs).map_err(|e| e.to_string())
}
