use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

pub struct KernelState {
    pub port: u16,
    pub child: Option<CommandChild>,
    pub workspace_dir: String,
}

impl Default for KernelState {
    fn default() -> Self {
        Self {
            port: 0,
            child: None,
            workspace_dir: String::new(),
        }
    }
}

pub fn find_available_port() -> u16 {
    portpicker::pick_unused_port().expect("No available ports")
}

pub fn spawn_kernel(app: &AppHandle, port: u16, workspace_dir: &str, lang: &str) -> Result<CommandChild, String> {
    // In dev mode, resources are in the app/ directory (parent of src-tauri/)
    // In production, they're in the resource_dir() provided by Tauri
    let wd = if cfg!(dev) {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        manifest_dir.parent().unwrap().to_string_lossy().to_string()
    } else {
        app.path().resource_dir().unwrap().to_string_lossy().to_string()
    };

    let args = vec![
        "--port".to_string(), port.to_string(),
        "--wd".to_string(), wd,
        "--workspace".to_string(), workspace_dir.to_string(),
        "--lang".to_string(), lang.to_string(),
    ];

    let (mut rx, child) = app.shell()
        .sidecar("SiYuan-Kernel")
        .expect("failed to find sidecar binary")
        .args(&args)
        .spawn()
        .map_err(|e| format!("Failed to spawn kernel: {}", e))?;

    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    let line = String::from_utf8_lossy(&line);
                    log::info!("[kernel stdout] {}", line);
                }
                CommandEvent::Stderr(line) => {
                    let line = String::from_utf8_lossy(&line);
                    log::error!("[kernel stderr] {}", line);
                }
                CommandEvent::Terminated(payload) => {
                    let code = payload.code.unwrap_or(-1);
                    log::info!("[kernel] exited with code {}", code);
                    let _ = app_handle.emit("kernel-exit", code);
                }
                _ => {}
            }
        }
    });

    Ok(child)
}

pub async fn wait_for_kernel(port: u16) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/api/system/version", port);

    for attempt in 0..15 {
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                log::info!("Kernel responded on attempt {}", attempt + 1);
                return Ok(());
            }
            _ => {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
    Err("Kernel failed to start within 7.5 seconds".to_string())
}

pub async fn wait_for_boot(port: u16) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/api/system/bootProgress", port);

    loop {
        match client.get(&url).send().await {
            Ok(resp) => {
                if let Ok(body) = resp.json::<serde_json::Value>().await {
                    if let Some(progress) = body["data"]["progress"].as_f64() {
                        if progress >= 100.0 {
                            return Ok(());
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Boot progress check failed: {}", e);
            }
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}

pub async fn register_ui_proc(port: u16) -> Result<(), String> {
    let client = reqwest::Client::new();
    let pid = std::process::id();
    let url = format!("http://127.0.0.1:{}/api/system/uiproc?pid={}", port, pid);
    client.post(&url).send().await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn exit_kernel(port: u16) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/api/system/exit", port);
    let body = serde_json::json!({"force": false});
    let _ = client.post(&url).json(&body).send().await;
    Ok(())
}
