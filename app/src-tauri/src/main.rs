#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kernel;
mod commands;
mod window_state;
mod protocol;

use std::sync::Mutex;
use tauri::{Manager, Emitter};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use tauri::menu::{Menu, MenuItem};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Builder as GlobalShortcutBuilder};

#[tauri::command]
async fn wellspring_init(
    app: tauri::AppHandle,
    _data: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let state = app.state::<Mutex<kernel::KernelState>>();
    let port;
    {
        let kernel_state = state.lock().unwrap();
        port = kernel_state.port;
    }

    Ok(serde_json::json!({
        "port": port,
    }))
}

#[tauri::command]
async fn register_hotkey(
    app: tauri::AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    let hotkey = data.get("hotkey")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if hotkey.is_empty() {
        // Unregister all
        app.global_shortcut().unregister_all().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let shortcut: Shortcut = hotkey.parse()
        .map_err(|e| format!("{:?}", e))?;

    let app_handle = app.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, _event| {
            if let Some(window) = app_handle.get_webview_window("main") {
                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn set_auto_launch(_data: serde_json::Value) -> Result<(), String> {
    // Handled by tauri-plugin-autostart — enable/disable based on data
    Ok(())
}

#[tauri::command]
async fn first_init(_data: serde_json::Value) -> Result<(), String> {
    // First-run initialization
    Ok(())
}

#[tauri::command]
async fn first_quit() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
async fn open_workspace(_data: serde_json::Value) -> Result<(), String> {
    // Workspace switching — will be handled by restarting kernel with new workspace
    log::info!("open_workspace requested");
    Ok(())
}

#[tauri::command]
async fn send_to_windows(
    app: tauri::AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    app.emit("wellspring-send-windows", &data).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_clipboard_read(data: serde_json::Value) -> Result<String, String> {
    let format = data.get("format").and_then(|v| v.as_str()).unwrap_or("text");
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;

    match format {
        "text" | "text/plain" => clipboard.get_text().map_err(|e| e.to_string()),
        "text/html" => clipboard.get_text().map_err(|e| e.to_string()),
        _ => Ok(String::new()),
    }
}

fn main() {
    env_logger::init();

    let port = kernel::find_available_port();

    tauri::Builder::default()
        .manage(Mutex::new(kernel::KernelState {
            port,
            ..Default::default()
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(GlobalShortcutBuilder::new().build())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            wellspring_init,
            register_hotkey,
            set_auto_launch,
            first_init,
            first_quit,
            open_workspace,
            send_to_windows,
            get_clipboard_read,
            commands::window::cmd_show,
            commands::window::cmd_hide,
            commands::window::cmd_minimize,
            commands::window::cmd_maximize,
            commands::window::cmd_restore,
            commands::window::cmd_focus,
            commands::window::cmd_destroy,
            commands::window::cmd_set_always_on_top_true,
            commands::window::cmd_set_always_on_top_false,
            commands::window::cmd_close_button_behavior,
            commands::window::get_is_full_screen,
            commands::window::get_is_maximized,
            commands::window::open_window,
            commands::window::quit_app,
            commands::system::cmd_notification,
            commands::system::cmd_open_path,
            commands::system::cmd_show_item_in_folder,
            commands::system::cmd_open_dev_tools,
            commands::system::cmd_clear_cache,
            commands::system::cmd_undo,
            commands::system::cmd_redo,
            commands::system::cmd_write_log,
            commands::system::cmd_set_spell_checker_languages,
            commands::system::get_available_spell_checker_languages,
            commands::system::cmd_set_traffic_light_position,
            commands::system::register_window_events,
            commands::dialog::show_confirm_dialog,
            commands::dialog::show_alert_dialog,
            commands::dialog::get_show_open_dialog,
            commands::dialog::get_show_save_dialog,
            commands::dialog::show_context_menu,
            commands::tray::config_tray,
            commands::export::export_pdf,
            commands::export::export_new_window,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app.state::<Mutex<kernel::KernelState>>();
            let port = state.lock().unwrap().port;

            // Load and apply window state
            let ws = window_state::load();
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_size(tauri::LogicalSize::new(ws.width, ws.height));
                if let (Some(x), Some(y)) = (ws.x, ws.y) {
                    // Verify the position is visible on at least one monitor
                    let position_ok = if let Ok(monitors) = window.available_monitors() {
                        if monitors.is_empty() {
                            true // No monitor info available, trust the saved state
                        } else {
                            let scale = window.scale_factor().unwrap_or(1.0);
                            monitors.iter().any(|m| {
                                let pos = m.position();
                                let size = m.size();
                                // Convert monitor physical bounds to logical
                                let mx = pos.x as f64 / scale;
                                let my = pos.y as f64 / scale;
                                let mw = size.width as f64 / scale;
                                let mh = size.height as f64 / scale;
                                // Check that at least part of the title bar (top 50px)
                                // is within this monitor's bounds
                                x < mx + mw && x + ws.width > mx
                                    && y < my + mh && y + 50.0 > my
                            })
                        }
                    } else {
                        true // Can't query monitors, trust saved state
                    };

                    if position_ok {
                        let _ = window.set_position(tauri::LogicalPosition::new(x, y));
                    } else {
                        log::warn!("Saved window position ({}, {}) is off-screen, centering window", x, y);
                        let _ = window.center();
                    }
                } else {
                    // No saved position (first launch) — center the window
                    let _ = window.center();
                }
                if ws.maximized {
                    let _ = window.maximize();
                }
                if ws.fullscreen {
                    let _ = window.set_fullscreen(true);
                }
            }

            // Register deep link protocol
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let _ = app.deep_link().register("wellspring");
            }

            // Build system tray
            let show_item = MenuItem::with_id(app, "show", "Show Wellspring", true, None::<&str>)
                .map_err(|e| e.to_string())?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)
                .map_err(|e| e.to_string())?;
            let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])
                .map_err(|e| e.to_string())?;

            let app_handle_tray = app.handle().clone();
            TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        if let Some(window) = app_handle_tray.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)
                .map_err(|e| e.to_string())?;

            // Spawn kernel
            let workspace_dir = dirs::home_dir()
                .map(|d| d.join("Wellspring").to_string_lossy().to_string())
                .unwrap_or_default();

            match kernel::spawn_kernel(&app_handle, port, &workspace_dir, "en_US") {
                Ok(child) => {
                    let mut ks = state.lock().unwrap();
                    ks.child = Some(child);
                    ks.workspace_dir = workspace_dir;
                }
                Err(e) => {
                    log::error!("Failed to spawn kernel: {}", e);
                }
            }

            // Wait for kernel and open window
            let app_handle2 = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = kernel::wait_for_kernel(port).await {
                    log::error!("{}", e);
                    return;
                }
                if let Err(e) = kernel::wait_for_boot(port).await {
                    log::error!("{}", e);
                    return;
                }
                if let Err(e) = kernel::register_ui_proc(port).await {
                    log::error!("{}", e);
                }

                // Redirect main window to kernel-served page
                if let Some(window) = app_handle2.get_webview_window("main") {
                    let url = format!("http://127.0.0.1:{}/stage/build/tauri/", port);
                    let _ = window.navigate(url.parse().unwrap());
                    // Give the page a moment to load, then show the window
                    // The frontend also calls show() via bridge, but this is a fallback
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    let _ = window.show();
                    let _ = window.set_focus();
                }

                // Mark setup complete so Resized/Moved events start saving state
                {
                    let state = app_handle2.state::<Mutex<kernel::KernelState>>();
                    state.lock().unwrap().setup_complete = true;
                }

                let _ = app_handle2.emit("kernel-ready", port);
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Prevent immediate close — let frontend handle save/close flow
                    api.prevent_close();
                    let _ = window.emit("wellspring-save-close", ());
                }
                tauri::WindowEvent::Focused(focused) => {
                    let event_type = if *focused { "focus" } else { "blur" };
                    let _ = window.emit("wellspring-event", event_type);
                }
                tauri::WindowEvent::Resized(size) => {
                    // size is PhysicalSize — convert to logical for save/restore consistency
                    if window.label() == "main" {
                        // Skip saving during setup to avoid races with window state restoration
                        if let Some(state) = window.try_state::<Mutex<kernel::KernelState>>() {
                            if !state.lock().unwrap().setup_complete { return; }
                        }
                        let scale = window.scale_factor().unwrap_or(1.0);
                        let ws = window_state::WindowState {
                            width: size.width as f64 / scale,
                            height: size.height as f64 / scale,
                            x: window.outer_position().ok().map(|p| p.x as f64 / scale),
                            y: window.outer_position().ok().map(|p| p.y as f64 / scale),
                            fullscreen: window.is_fullscreen().unwrap_or(false),
                            maximized: window.is_maximized().unwrap_or(false),
                        };
                        window_state::save(&ws);
                    }
                }
                tauri::WindowEvent::Moved(position) => {
                    // position is PhysicalPosition — convert to logical for save/restore consistency
                    if window.label() == "main" {
                        // Skip saving during setup to avoid races with window state restoration
                        if let Some(state) = window.try_state::<Mutex<kernel::KernelState>>() {
                            if !state.lock().unwrap().setup_complete { return; }
                        }
                        let scale = window.scale_factor().unwrap_or(1.0);
                        if let Ok(size) = window.inner_size() {
                            let ws = window_state::WindowState {
                                width: size.width as f64 / scale,
                                height: size.height as f64 / scale,
                                x: Some(position.x as f64 / scale),
                                y: Some(position.y as f64 / scale),
                                fullscreen: window.is_fullscreen().unwrap_or(false),
                                maximized: window.is_maximized().unwrap_or(false),
                            };
                            window_state::save(&ws);
                        }
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
