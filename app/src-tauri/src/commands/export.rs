use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn export_pdf(
    _app: AppHandle,
    _data: serde_json::Value,
) -> Result<(), String> {
    // PDF export in Tauri delegates to the kernel's pandoc-based export
    // The frontend will call the kernel's /api/export/* endpoints directly
    log::info!("PDF export requested — delegating to kernel");
    Ok(())
}

#[tauri::command]
pub async fn export_new_window(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    let url = data.get("url").and_then(|v| v.as_str()).unwrap_or("");
    let state = app.state::<std::sync::Mutex<crate::kernel::KernelState>>();
    let port = state.lock().unwrap().port;
    let full_url = format!("http://127.0.0.1:{}{}", port, url);

    let label = format!("export-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    tauri::WebviewWindowBuilder::new(
        &app,
        &label,
        tauri::WebviewUrl::External(full_url.parse().unwrap()),
    )
    .title("SiYuan Export")
    .inner_size(1200.0, 750.0)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
