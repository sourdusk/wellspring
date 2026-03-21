use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
pub async fn cmd_show(window: tauri::Window) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_hide(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_maximize(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())?;
        let _ = window.emit("siyuan-event", "unmaximize");
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        let _ = window.emit("siyuan-event", "maximize");
    }
    Ok(())
}

#[tauri::command]
pub async fn cmd_restore(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())?;
    }
    if window.is_fullscreen().unwrap_or(false) {
        window.set_fullscreen(false).map_err(|e| e.to_string())?;
    }
    let _ = window.emit("siyuan-event", "unmaximize");
    Ok(())
}

#[tauri::command]
pub async fn cmd_focus(window: tauri::Window) -> Result<(), String> {
    window.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_destroy(window: tauri::Window) -> Result<(), String> {
    window.destroy().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_set_always_on_top_true(window: tauri::Window) -> Result<(), String> {
    window.set_always_on_top(true).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_set_always_on_top_false(window: tauri::Window) -> Result<(), String> {
    window.set_always_on_top(false).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_is_full_screen(window: tauri::Window) -> Result<bool, String> {
    window.is_fullscreen().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_is_maximized(window: tauri::Window) -> Result<bool, String> {
    window.is_maximized().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_close_button_behavior(
    app: AppHandle,
    window: tauri::Window,
    data: serde_json::Value,
) -> Result<(), String> {
    let behavior = data.get("behavior")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    if behavior == 0 {
        // Minimize to tray
        window.hide().map_err(|e| e.to_string())
    } else {
        // Quit
        let state = app.state::<std::sync::Mutex<crate::kernel::KernelState>>();
        let port = state.lock().unwrap().port;
        let _ = crate::kernel::exit_kernel(port).await;
        app.exit(0);
        Ok(())
    }
}

#[tauri::command]
pub async fn open_window(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    let url = data.get("url")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let label = format!("window-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    let state = app.state::<std::sync::Mutex<crate::kernel::KernelState>>();
    let port = state.lock().unwrap().port;
    let full_url = format!("http://127.0.0.1:{}{}", port, url);

    tauri::WebviewWindowBuilder::new(
        &app,
        &label,
        tauri::WebviewUrl::External(full_url.parse().unwrap()),
    )
    .title("SiYuan")
    .inner_size(1200.0, 750.0)
    .min_inner_size(493.0, 376.0)
    .decorations(false)
    .disable_drag_drop_handler()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: AppHandle) -> Result<(), String> {
    let state = app.state::<std::sync::Mutex<crate::kernel::KernelState>>();
    let port = state.lock().unwrap().port;
    let _ = crate::kernel::exit_kernel(port).await;
    app.exit(0);
    Ok(())
}
