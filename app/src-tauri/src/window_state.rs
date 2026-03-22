use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowState {
    #[serde(default = "default_width")]
    pub width: f64,
    #[serde(default = "default_height")]
    pub height: f64,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub maximized: bool,
}

fn default_width() -> f64 { 1280.0 }
fn default_height() -> f64 { 800.0 }

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            x: None,
            y: None,
            fullscreen: false,
            maximized: false,
        }
    }
}

fn state_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wellspring");
    fs::create_dir_all(&config_dir).ok();
    config_dir.join("windowState.json")
}

pub fn load() -> WindowState {
    let path = state_path();
    let state = match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => WindowState::default(),
    };
    // Basic sanity check: reject absurd sizes (position is validated
    // against actual monitor geometry in main.rs)
    if state.width < 200.0 || state.height < 200.0
        || state.width > 20000.0 || state.height > 20000.0
    {
        log::warn!("Saved window size ({} x {}) is out of range, using defaults", state.width, state.height);
        return WindowState::default();
    }
    state
}

pub fn save(state: &WindowState) {
    let path = state_path();
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = fs::write(path, json);
    }
}
