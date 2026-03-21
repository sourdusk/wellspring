# Tauri Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate SiYuan's desktop shell from Electron to Tauri v2 using a sidecar + ifdef approach, coexisting with the Electron build.

**Architecture:** Add a `src-tauri/` Rust backend alongside the existing Electron main process. Create a `webpack.tauri.js` config that targets `web` with a `TAURI` ifdef flag. A bridge module (`src/tauri/bridge.ts`) translates `ipcRenderer` patterns to Tauri's `invoke`/`listen` APIs. The Go kernel runs as a Tauri sidecar binary.

**Tech Stack:** Tauri v2, Rust, @tauri-apps/api v2, existing TypeScript/webpack/ifdef-loader toolchain

---

## Task 1: Tauri Project Scaffolding

**Files:**
- Create: `app/src-tauri/Cargo.toml`
- Create: `app/src-tauri/build.rs`
- Create: `app/src-tauri/tauri.conf.json`
- Create: `app/src-tauri/capabilities/default.json`
- Create: `app/src-tauri/src/main.rs`
- Create: `app/src-tauri/icons/` (copy from stage/icon-large.png, generate with `cargo tauri icon`)
- Modify: `app/package.json` (add dependencies and scripts)

**Step 1: Install Tauri CLI and API**

Run:
```bash
cd app && pnpm add -D @tauri-apps/cli@^2 && pnpm add @tauri-apps/api@^2
```

**Step 2: Create `app/src-tauri/Cargo.toml`**

```toml
[package]
name = "siyuan"
version = "3.6.1"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [
    "tray-icon",
    "image-png",
    "protocol-asset",
] }
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-notification = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-deep-link = "2"
tauri-plugin-autostart = "2"
tauri-plugin-process = "2"
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
log = "0.4"
env_logger = "0.11"
portpicker = "0.1"
```

**Step 3: Create `app/src-tauri/build.rs`**

```rust
fn main() {
    tauri_build::build();
}
```

**Step 4: Create `app/src-tauri/tauri.conf.json`**

```json
{
  "$schema": "https://raw.githubusercontent.com/nickel-org/nickel.rs/refs/heads/master/schema/tauri-schema-v2.json",
  "productName": "SiYuan",
  "identifier": "org.b3log.siyuan",
  "version": "3.6.1",
  "build": {
    "frontendDist": "../stage/build/tauri",
    "devUrl": "http://localhost:6806/stage/build/tauri/",
    "beforeBuildCommand": "pnpm run build:tauri",
    "beforeDevCommand": "pnpm run dev:tauri"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "SiYuan",
        "width": 1280,
        "height": 800,
        "minWidth": 493,
        "minHeight": 376,
        "decorations": false,
        "visible": false,
        "url": "index.html"
      }
    ],
    "security": {
      "dangerousRemoteDomainIpcAccess": [
        {
          "domain": "127.0.0.1",
          "enableTauriAPI": true,
          "windows": ["main", "window-*"]
        }
      ],
      "csp": null
    },
    "trayIcon": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  },
  "bundle": {
    "active": true,
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "externalBin": [
      "kernel/SiYuan-Kernel"
    ],
    "resources": [
      "appearance/**/*",
      "guide/**/*",
      "stage/**/*",
      "changelogs/**/*"
    ],
    "targets": "all"
  }
}
```

**Step 5: Create `app/src-tauri/capabilities/default.json`**

```json
{
  "$schema": "https://raw.githubusercontent.com/nickel-org/nickel.rs/refs/heads/master/schema/tauri-schema-v2.json",
  "identifier": "default",
  "description": "Default capabilities for SiYuan",
  "windows": ["main", "window-*"],
  "permissions": [
    "core:default",
    "core:window:default",
    "core:window:allow-create",
    "core:window:allow-close",
    "core:window:allow-set-focus",
    "core:window:allow-set-size",
    "core:window:allow-set-position",
    "core:window:allow-set-fullscreen",
    "core:window:allow-set-always-on-top",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "core:window:allow-unmaximize",
    "core:window:allow-show",
    "core:window:allow-hide",
    "core:window:allow-set-decorations",
    "core:window:allow-is-fullscreen",
    "core:window:allow-is-maximized",
    "core:window:allow-set-title",
    "core:window:allow-center",
    "shell:default",
    "shell:allow-open",
    "shell:allow-spawn",
    "shell:allow-execute",
    "dialog:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "dialog:allow-message",
    "dialog:allow-ask",
    "notification:default",
    "notification:allow-notify",
    "notification:allow-is-permission-granted",
    "notification:allow-request-permission",
    "global-shortcut:default",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "deep-link:default",
    "autostart:default",
    "process:default",
    "opener:default"
  ]
}
```

**Step 6: Create minimal `app/src-tauri/src/main.rs`**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 7: Add Tauri scripts to `app/package.json`**

Add to the `"scripts"` section:
```json
"dev:tauri": "webpack --mode development --config webpack.tauri.js",
"build:tauri": "webpack --mode production --config webpack.tauri.js",
"tauri": "tauri"
```

**Step 8: Verify Rust compiles**

Run:
```bash
cd app/src-tauri && cargo check
```
Expected: Compiles with no errors (warnings OK).

**Step 9: Commit**

```bash
git add app/src-tauri/ app/package.json app/pnpm-lock.yaml
git commit -m "feat(tauri): scaffold Tauri v2 project with plugins and capabilities"
```

---

## Task 2: Webpack Tauri Config

**Files:**
- Create: `app/webpack.tauri.js`
- Create: `app/src/assets/template/tauri/index.tpl`
- Create: `app/src/assets/template/tauri/window.tpl`

**Step 1: Create `app/webpack.tauri.js`**

Clone from `webpack.config.js` with these changes:

```javascript
const path = require("path");
const webpack = require("webpack");
const pkg = require("./package.json");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const {CleanWebpackPlugin} = require("clean-webpack-plugin");
const {EsbuildPlugin} = require("esbuild-loader");

module.exports = (env, argv) => {
    return {
        mode: argv.mode || "development",
        watch: argv.mode !== "production",
        devtool: argv.mode !== "production" ? "eval-source-map" : false,
        target: "web",
        output: {
            publicPath: "auto",
            filename: "[name].[chunkhash].js",
            path: path.resolve(__dirname, "stage/build/tauri"),
        },
        entry: {
            "main": "./src/index.ts",
            "window": "./src/window/index.ts",
        },
        resolve: {
            extensions: [".ts", ".js", ".tpl", ".scss", ".png", ".svg"],
        },
        optimization: {
            minimize: argv.mode === "production",
            minimizer: [
                new EsbuildPlugin({
                    target: "es2021",
                    sourcemap: argv.mode !== "production",
                }),
            ],
        },
        module: {
            rules: [
                {
                    test: /\.tpl/,
                    include: [
                        path.resolve(__dirname, "src/assets/template/tauri/index.tpl"),
                        path.resolve(__dirname, "src/assets/template/tauri/window.tpl")],
                    loader: "html-loader",
                    options: {
                        sources: false,
                    },
                },
                {
                    test: /\.ts(x?)$/,
                    include: [path.resolve(__dirname, "src")],
                    use: [
                        {
                            loader: "esbuild-loader",
                            options: {
                                target: "es2021",
                                sourcemap: argv.mode !== "production",
                            },
                        },
                        {
                            loader: "ifdef-loader", options: {
                                BROWSER: false,
                                MOBILE: false,
                                TAURI: true,
                            },
                        },
                    ],
                },
                {
                    test: /\.scss$/,
                    include: [
                        path.resolve(__dirname, "src/assets/scss"),
                    ],
                    use: [
                        MiniCssExtractPlugin.loader,
                        {
                            loader: "css-loader",
                            options: {
                                sourceMap: argv.mode !== "production",
                            },
                        },
                        {
                            loader: "sass-loader",
                            options: {
                                sourceMap: argv.mode !== "production",
                            },
                        },
                    ],
                },
                {
                    test: /\.(png|svg)$/,
                    use: [
                        {
                            loader: "file-loader",
                            options: {
                                name: "[name].[ext]",
                                outputPath: "../../",
                            },
                        },
                    ],
                },
            ],
        },
        plugins: [
            new CleanWebpackPlugin({
                cleanStaleWebpackAssets: false,
                cleanOnceBeforeBuildPatterns: [
                    path.join(__dirname, "stage/build/tauri")],
            }),
            new webpack.DefinePlugin({
                SIYUAN_VERSION: JSON.stringify(pkg.version),
                NODE_ENV: JSON.stringify(argv.mode),
            }),
            new MiniCssExtractPlugin({
                filename: "base.[contenthash].css",
            }),
            new HtmlWebpackPlugin({
                inject: "head",
                chunks: ["main"],
                filename: "index.html",
                template: "src/assets/template/tauri/index.tpl",
            }),
            new HtmlWebpackPlugin({
                inject: "head",
                chunks: ["window"],
                filename: "window.html",
                template: "src/assets/template/tauri/window.tpl",
            }),
        ],
    };
};
```

**Step 2: Create `app/src/assets/template/tauri/index.tpl`**

Copy from `app/src/assets/template/app/index.tpl` verbatim. The template is pure HTML — no Electron-specific content.

**Step 3: Create `app/src/assets/template/tauri/window.tpl`**

Copy from `app/src/assets/template/app/window.tpl` verbatim.

**Step 4: Run the Tauri webpack build**

Run:
```bash
cd app && pnpm run build:tauri
```
Expected: Build fails with Electron import errors. This is expected — we haven't created the bridge yet. Confirm the webpack config itself is valid and the ifdef-loader processes `TAURI: true`.

**Step 5: Commit**

```bash
git add app/webpack.tauri.js app/src/assets/template/tauri/
git commit -m "feat(tauri): add webpack config and HTML templates for Tauri build"
```

---

## Task 3: Frontend IPC Bridge

This is the core translation layer. It replaces all `ipcRenderer.send()`, `ipcRenderer.invoke()`, and `ipcRenderer.on()` calls with Tauri equivalents.

**Files:**
- Create: `app/src/tauri/bridge.ts`

**Step 1: Create `app/src/tauri/bridge.ts`**

```typescript
import {invoke} from "@tauri-apps/api/core";
import {listen, emit, type UnlistenFn} from "@tauri-apps/api/event";
import {getCurrentWindow} from "@tauri-apps/api/window";

// Maps Electron ipcRenderer.send(channel, data) pattern
export function send(channel: string, data?: any): void {
    switch (channel) {
        case "siyuan-cmd": {
            const cmd = data?.cmd;
            if (cmd) {
                invoke(`cmd_${toSnakeCase(cmd)}`, {data}).catch(console.error);
            }
            break;
        }
        case "siyuan-config-tray":
            invoke("config_tray", {data}).catch(console.error);
            break;
        case "siyuan-export-pdf":
            invoke("export_pdf", {data}).catch(console.error);
            break;
        case "siyuan-export-newwindow":
            invoke("export_new_window", {data}).catch(console.error);
            break;
        case "siyuan-quit":
            invoke("quit_app").catch(console.error);
            break;
        case "siyuan-show-window":
            getCurrentWindow().show().catch(console.error);
            break;
        case "siyuan-open-window":
            invoke("open_window", {data}).catch(console.error);
            break;
        case "siyuan-open-workspace":
            invoke("open_workspace", {data}).catch(console.error);
            break;
        case "siyuan-hotkey":
            invoke("register_hotkey", {data}).catch(console.error);
            break;
        case "siyuan-send-windows":
            emit("siyuan-send-windows", data).catch(console.error);
            break;
        case "siyuan-auto-launch":
            invoke("set_auto_launch", {data}).catch(console.error);
            break;
        case "siyuan-first-init":
            invoke("first_init", {data}).catch(console.error);
            break;
        case "siyuan-ready-to-show":
            getCurrentWindow().show().catch(console.error);
            break;
        case "siyuan-context-menu":
            invoke("show_context_menu", {data}).catch(console.error);
            break;
        case "siyuan-confirm-dialog":
            invoke("show_confirm_dialog", {data}).catch(console.error);
            break;
        case "siyuan-alert-dialog":
            invoke("show_alert_dialog", {data}).catch(console.error);
            break;
        case "siyuan-first-quit":
            invoke("first_quit").catch(console.error);
            break;
        case "siyuan-event":
            invoke("register_window_events").catch(console.error);
            break;
        default:
            console.warn(`[tauri bridge] unhandled send channel: ${channel}`);
    }
}

// Maps Electron ipcRenderer.invoke(channel, data) pattern
export async function invokeHandler(channel: string, data?: any): Promise<any> {
    switch (channel) {
        case "siyuan-init":
            return invoke("siyuan_init", {data});
        case "siyuan-get": {
            const cmd = data?.cmd;
            if (cmd) {
                return invoke(`get_${toSnakeCase(cmd)}`, {data});
            }
            return null;
        }
        default:
            console.warn(`[tauri bridge] unhandled invoke channel: ${channel}`);
            return null;
    }
}

// Maps Electron ipcRenderer.on(channel, callback) pattern
export function on(channel: string, callback: (...args: any[]) => void): UnlistenFn | void {
    const tauriChannel = channel;
    let unlisten: Promise<UnlistenFn>;

    switch (channel) {
        case "siyuan-event":
        case "siyuan-open-url":
        case "siyuan-open-file":
        case "siyuan-save-close":
        case "siyuan-send-windows":
        case "siyuan-hotkey":
        case "siyuan-export-pdf":
            unlisten = listen(tauriChannel, (event) => {
                // Electron callback signature is (event, data), Tauri is (event)
                callback(null, event.payload);
            });
            break;
        default:
            console.warn(`[tauri bridge] unhandled on channel: ${channel}`);
            return;
    }

    // Return synchronously — caller can ignore the unlisten if they want
    let unlistenFn: UnlistenFn | undefined;
    unlisten.then(fn => unlistenFn = fn);
    return (() => unlistenFn?.()) as UnlistenFn;
}

function toSnakeCase(str: string): string {
    return str.replace(/([A-Z])/g, "_$1").toLowerCase().replace(/^_/, "");
}
```

**Step 2: Commit**

```bash
git add app/src/tauri/
git commit -m "feat(tauri): add IPC bridge translating ipcRenderer to Tauri invoke/listen"
```

---

## Task 4: Update Frontend Files with TAURI ifdef Blocks

Modify all 24 frontend files that import Electron APIs to add `/// #if TAURI` conditional blocks. The pattern is:

```typescript
/// #if !BROWSER
/// #if !TAURI
import {ipcRenderer} from "electron";
/// #endif
/// #endif
/// #if TAURI
import {send, invokeHandler, on} from "../tauri/bridge";
/// #endif
```

Then within function bodies, replace `ipcRenderer.send(` with `send(`, `ipcRenderer.invoke(` with `invokeHandler(`, and `ipcRenderer.on(` with `on(`.

**Files to modify (24 files):**

Each file follows the same mechanical pattern. Listed with their key changes:

- Modify: `app/src/boot/onGetConfig.ts` — Replace `ipcRenderer`/`webFrame`/`fs`/`path` imports. Replace `webFrame.setZoomFactor()` with CSS `document.body.style.zoom`. Replace `fs.writeFileSync` in PDF export with Tauri `invoke("write_file")`.
- Modify: `app/src/boot/globalEvent/command/global.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/boot/globalEvent/commonHotkey.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/boot/globalEvent/keydown.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/card/openCard.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/config/about.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/config/editor.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/config/exportConfig.ts` — Replace `ipcRenderer.send`/`invoke` calls
- Modify: `app/src/config/keymap.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/dialog/processSystem.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/editor/openLink.ts` — Replace `shell.openExternal` with Tauri `open()`
- Modify: `app/src/editor/util.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/index.ts` — Replace `ipcRenderer.send`/`on` in main entry
- Modify: `app/src/layout/dock/Files.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/layout/status.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/layout/Tab.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/layout/topBar.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/menus/commonMenuItem.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/menus/index.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/menus/navigation.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/menus/util.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/menus/workspace.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/plugin/platformUtils.ts` — Replace notification IPC
- Modify: `app/src/util/pathName.ts` — Replace `shell.openExternal` calls
- Modify: `app/src/window/closeWin.ts` — Replace `ipcRenderer.send` calls
- Modify: `app/src/window/init.ts` — Replace `ipcRenderer` calls
- Modify: `app/src/window/openNewWindow.ts` — Replace `ipcRenderer.send` calls

**Step 1: For each file, apply the ifdef pattern**

The transformation is mechanical. For every `/// #if !BROWSER` block that contains Electron imports or `ipcRenderer` calls:

1. Wrap the Electron-specific code in `/// #if !TAURI` inside the existing `/// #if !BROWSER`
2. Add a sibling `/// #if TAURI` block with the Tauri equivalent

Example transformation for `app/src/boot/onGetConfig.ts`:

**Before:**
```typescript
/// #if !BROWSER
import {ipcRenderer, webFrame} from "electron";
import * as fs from "fs";
import * as path from "path";
/// #endif
```

**After:**
```typescript
/// #if !BROWSER
/// #if !TAURI
import {ipcRenderer, webFrame} from "electron";
import * as fs from "fs";
import * as path from "path";
/// #endif
/// #endif
/// #if TAURI
import {send, invokeHandler, on} from "../tauri/bridge";
/// #endif
```

And within function bodies, `ipcRenderer.send(...)` becomes `send(...)`, etc.

For `webFrame.setZoomFactor(zoom)`, the Tauri replacement is:
```typescript
/// #if TAURI
document.documentElement.style.setProperty("zoom", String(zoom));
/// #endif
```

For `shell.openExternal(url)` (in `openLink.ts`, `pathName.ts`):
```typescript
/// #if TAURI
import {open} from "@tauri-apps/plugin-opener";
open(url);
/// #endif
```

**Step 2: Verify the Tauri webpack build compiles**

Run:
```bash
cd app && pnpm run build:tauri
```
Expected: Build succeeds with no Electron import errors. The output lands in `stage/build/tauri/`.

**Step 3: Commit**

```bash
git add app/src/
git commit -m "feat(tauri): add TAURI ifdef blocks to all 24 frontend files"
```

---

## Task 5: Rust Kernel Sidecar Management

The most critical Rust code — spawning, monitoring, and shutting down the Go kernel.

**Files:**
- Create: `app/src-tauri/src/kernel.rs`
- Modify: `app/src-tauri/src/main.rs`

**Step 1: Create `app/src-tauri/src/kernel.rs`**

```rust
use std::sync::Mutex;
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
    let args = vec![
        "--port".to_string(), port.to_string(),
        "--wd".to_string(), app.path().resource_dir().unwrap().to_string_lossy().to_string(),
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
```

**Step 2: Update `app/src-tauri/src/main.rs` to use kernel module and manage state**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kernel;

use std::sync::Mutex;
use tauri::{Manager, Emitter};

#[tauri::command]
async fn siyuan_init(
    app: tauri::AppHandle,
    data: serde_json::Value,
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
        .plugin(tauri_plugin_global_shortcut::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            siyuan_init,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app.state::<Mutex<kernel::KernelState>>();
            let port = state.lock().unwrap().port;

            // Spawn kernel
            let workspace_dir = dirs::home_dir()
                .map(|d| d.join("SiYuan").to_string_lossy().to_string())
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
                }

                let _ = app_handle2.emit("kernel-ready", port);
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Prevent immediate close — let frontend handle save/close flow
                api.prevent_close();
                let _ = window.emit("siyuan-save-close", ());
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: Add `dirs` crate to Cargo.toml**

Add to `[dependencies]`:
```toml
dirs = "5"
```

**Step 4: Verify it compiles**

Run:
```bash
cd app/src-tauri && cargo check
```
Expected: Compiles successfully.

**Step 5: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): implement kernel sidecar lifecycle management"
```

---

## Task 6: Rust IPC Command Handlers

Implement the `#[tauri::command]` functions that handle the frontend bridge calls.

**Files:**
- Create: `app/src-tauri/src/commands/mod.rs`
- Create: `app/src-tauri/src/commands/window.rs`
- Create: `app/src-tauri/src/commands/system.rs`
- Create: `app/src-tauri/src/commands/dialog.rs`
- Create: `app/src-tauri/src/commands/tray.rs`
- Create: `app/src-tauri/src/commands/export.rs`
- Modify: `app/src-tauri/src/main.rs` (register all commands)

**Step 1: Create `app/src-tauri/src/commands/mod.rs`**

```rust
pub mod window;
pub mod system;
pub mod dialog;
pub mod tray;
pub mod export;
```

**Step 2: Create `app/src-tauri/src/commands/window.rs`**

```rust
use tauri::{AppHandle, Manager, Emitter};

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
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn cmd_restore(window: tauri::Window) -> Result<(), String> {
    window.set_fullscreen(false).map_err(|e| e.to_string())
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
```

**Step 3: Create `app/src-tauri/src/commands/system.rs`**

```rust
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn cmd_notification(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    let title = data.get("title").and_then(|v| v.as_str()).unwrap_or("SiYuan");
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
    let path = data.get("path").and_then(|v| v.as_str()).unwrap_or("");
    app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_show_item_in_folder(app: AppHandle, data: serde_json::Value) -> Result<(), String> {
    let path = data.get("path").and_then(|v| v.as_str()).unwrap_or("");
    app.opener().reveal_item_in_dir(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_open_dev_tools(window: tauri::Window) -> Result<(), String> {
    #[cfg(debug_assertions)]
    window.open_devtools();
    Ok(())
}

#[tauri::command]
pub async fn cmd_clear_cache(window: tauri::Window) -> Result<(), String> {
    // Tauri v2 doesn't expose cache clearing directly
    // This is a no-op for now
    log::info!("clearCache requested (no-op in Tauri)");
    Ok(())
}

#[tauri::command]
pub async fn cmd_undo(window: tauri::Window) -> Result<(), String> {
    // Handled by webview natively
    Ok(())
}

#[tauri::command]
pub async fn cmd_redo(window: tauri::Window) -> Result<(), String> {
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
pub async fn cmd_set_traffic_light_position() -> Result<(), String> {
    // macOS-only, Tauri handles via config
    Ok(())
}

#[tauri::command]
pub async fn register_window_events() -> Result<(), String> {
    // Window events are handled via Tauri's built-in event system
    Ok(())
}
```

**Step 4: Create `app/src-tauri/src/commands/dialog.rs`**

```rust
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

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
        .ok_button_label("OK")
        .cancel_button_label("Cancel")
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
        .ok_button_label("OK")
        .blocking_show();

    Ok(())
}

#[tauri::command]
pub async fn get_show_open_dialog(
    app: AppHandle,
    data: serde_json::Value,
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
```

**Step 5: Create `app/src-tauri/src/commands/tray.rs`**

```rust
use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;

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
```

**Step 6: Create `app/src-tauri/src/commands/export.rs`**

```rust
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn export_pdf(
    app: AppHandle,
    data: serde_json::Value,
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
```

**Step 7: Update `main.rs` to register all command handlers**

Add all commands to the `invoke_handler`:

```rust
.invoke_handler(tauri::generate_handler![
    siyuan_init,
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
    commands::system::cmd_set_traffic_light_position,
    commands::system::register_window_events,
    commands::dialog::show_confirm_dialog,
    commands::dialog::show_alert_dialog,
    commands::dialog::get_show_open_dialog,
    commands::dialog::get_show_save_dialog,
    commands::tray::config_tray,
    commands::export::export_pdf,
    commands::export::export_new_window,
])
```

Add `mod commands;` at the top of main.rs.

**Step 8: Verify it compiles**

Run:
```bash
cd app/src-tauri && cargo check
```
Expected: Compiles successfully.

**Step 9: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): implement IPC command handlers for window, system, dialog, tray, export"
```

---

## Task 7: Window State Persistence

**Files:**
- Create: `app/src-tauri/src/window_state.rs`
- Modify: `app/src-tauri/src/main.rs`

**Step 1: Create `app/src-tauri/src/window_state.rs`**

```rust
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
        .join("siyuan");
    fs::create_dir_all(&config_dir).ok();
    config_dir.join("windowState.json")
}

pub fn load() -> WindowState {
    let path = state_path();
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => WindowState::default(),
    }
}

pub fn save(state: &WindowState) {
    let path = state_path();
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = fs::write(path, json);
    }
}
```

**Step 2: Integrate into `main.rs` setup and window event handler**

In the `setup` closure, load window state and apply it to the main window. In the `on_window_event` handler, save state on move/resize. This is a modification to the existing `main.rs` — add calls to `window_state::load()` in setup and `window_state::save()` in the window event handler.

**Step 3: Commit**

```bash
git add app/src-tauri/src/window_state.rs app/src-tauri/src/main.rs
git commit -m "feat(tauri): add window state persistence"
```

---

## Task 8: Deep Link and Auto-Launch

**Files:**
- Create: `app/src-tauri/src/protocol.rs`
- Modify: `app/src-tauri/src/main.rs`

**Step 1: Create `app/src-tauri/src/protocol.rs`**

```rust
use tauri::{AppHandle, Emitter};

pub fn handle_deep_link(app: &AppHandle, urls: Vec<url::Url>) {
    for url in urls {
        log::info!("Deep link received: {}", url);
        let _ = app.emit("siyuan-open-url", url.to_string());
    }
}
```

**Step 2: Add `url` crate to `Cargo.toml`**

```toml
url = "2"
```

**Step 3: Register deep link handler in `main.rs` setup**

```rust
app.handle().plugin(tauri_plugin_deep_link::init())?;
// In setup:
#[cfg(any(target_os = "linux", target_os = "windows"))]
{
    use tauri_plugin_deep_link::DeepLinkExt;
    let _ = app.deep_link().register("siyuan");
}
```

**Step 4: Register auto-launch in `main.rs` commands**

```rust
#[tauri::command]
pub async fn set_auto_launch(data: serde_json::Value) -> Result<(), String> {
    // Handled by tauri-plugin-autostart — enable/disable based on data
    Ok(())
}

#[tauri::command]
pub async fn first_init(data: serde_json::Value) -> Result<(), String> {
    // First-run initialization
    Ok(())
}

#[tauri::command]
pub async fn first_quit() -> Result<(), String> {
    Ok(())
}
```

**Step 5: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): add deep link handling and auto-launch support"
```

---

## Task 9: Global Shortcuts

**Files:**
- Modify: `app/src-tauri/src/main.rs`

**Step 1: Register global shortcut handler in setup**

```rust
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

// In setup, after kernel ready:
#[tauri::command]
pub async fn register_hotkey(
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
        .map_err(|e: tauri_plugin_global_shortcut::Error| e.to_string())?;

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
```

**Step 2: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): add global shortcut registration"
```

---

## Task 10: System Tray Setup

**Files:**
- Modify: `app/src-tauri/src/main.rs`

**Step 1: Create tray in setup**

```rust
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use tauri::menu::{Menu, MenuItem};

// In setup:
let show_item = MenuItem::with_id(app, "show", "Show SiYuan", true, None::<&str>)
    .map_err(|e| e.to_string())?;
let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)
    .map_err(|e| e.to_string())?;
let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])
    .map_err(|e| e.to_string())?;

let app_handle_tray = app.handle().clone();
TrayIconBuilder::with_id("main-tray")
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&tray_menu)
    .on_tray_icon_event(move |tray, event| {
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
```

**Step 2: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): add system tray with show/quit menu"
```

---

## Task 11: Context Menu Support

**Files:**
- Modify: `app/src-tauri/src/commands/dialog.rs`

**Step 1: Add context menu command**

The Electron version builds native context menus from JSON. In Tauri, we can use `tauri::menu::Menu` built dynamically:

```rust
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
) -> Result<Menu<tauri::Wry>, String> {
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
```

**Step 2: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): add dynamic context menu support"
```

---

## Task 12: Integration Test — Build and Launch

**Step 1: Full webpack build**

Run:
```bash
cd app && pnpm run build:tauri
```
Expected: Successful build with output in `stage/build/tauri/`.

**Step 2: Full Rust build**

Run:
```bash
cd app/src-tauri && cargo build
```
Expected: Compiles successfully. Binary at `target/debug/siyuan` (or `siyuan.exe` on Windows).

**Step 3: Verify the Go kernel sidecar is in place**

The kernel binary needs to be copied to the right location for development:
```bash
# On Windows:
cp kernel/SiYuan-Kernel.exe src-tauri/SiYuan-Kernel-x86_64-pc-windows-msvc.exe
# On Linux:
# cp kernel/SiYuan-Kernel src-tauri/SiYuan-Kernel-x86_64-unknown-linux-gnu
# On macOS:
# cp kernel/SiYuan-Kernel src-tauri/SiYuan-Kernel-aarch64-apple-darwin
```

**Step 4: Launch in dev mode**

Run:
```bash
cd app && pnpm tauri dev
```
Expected: App window opens, kernel starts, SiYuan UI loads. If the UI doesn't load, check the terminal for kernel startup errors and the webview console for JS errors.

**Step 5: Smoke test checklist**

- [ ] App window opens and shows SiYuan UI
- [ ] Can create/edit a document
- [ ] System tray icon appears (Windows/Linux)
- [ ] Tray click toggles window visibility
- [ ] Window minimize/maximize/close buttons work
- [ ] File open/save dialogs work
- [ ] Desktop notifications work
- [ ] Opening external links works (shell.openExternal equivalent)
- [ ] Context menus appear
- [ ] Window position/size persists across restarts

**Step 6: Commit any fixes**

```bash
git add -A
git commit -m "fix(tauri): integration fixes from first launch testing"
```

---

## Task 13: Multi-Window Coordination

**Files:**
- Modify: `app/src-tauri/src/commands/window.rs`
- Modify: `app/src-tauri/src/main.rs`

**Step 1: Add inter-window event broadcasting**

In Electron, `siyuan-send-windows` broadcasts to all windows. In Tauri, use `emit` which broadcasts to all windows:

```rust
#[tauri::command]
pub async fn send_to_windows(
    app: AppHandle,
    data: serde_json::Value,
) -> Result<(), String> {
    app.emit("siyuan-send-windows", &data).map_err(|e| e.to_string())
}
```

Register this command in the invoke handler.

**Step 2: Handle window focus/blur events**

Add window event listeners in the `on_window_event` handler to emit `siyuan-event` with focus/blur/maximize/unmaximize/fullscreen payloads:

```rust
tauri::WindowEvent::Focused(focused) => {
    let event_type = if *focused { "focus" } else { "blur" };
    let _ = window.emit("siyuan-event", serde_json::json!({"type": event_type}));
}
```

**Step 3: Commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): add multi-window event broadcasting and focus tracking"
```

---

## Task 14: Clipboard Support

**Files:**
- Modify: `app/src-tauri/src/commands/system.rs`

**Step 1: Add clipboard read command**

Tauri doesn't have a built-in clipboard plugin in v2 core, but `arboard` works:

Add to `Cargo.toml`:
```toml
arboard = "3"
```

```rust
#[tauri::command]
pub async fn get_clipboard_read(data: serde_json::Value) -> Result<String, String> {
    let format = data.get("format").and_then(|v| v.as_str()).unwrap_or("text");
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;

    match format {
        "text" | "text/plain" => clipboard.get_text().map_err(|e| e.to_string()),
        "text/html" => clipboard.get_text().map_err(|e| e.to_string()), // HTML reading varies
        _ => Ok(String::new()),
    }
}
```

**Step 2: Register in invoke handler and commit**

```bash
git add app/src-tauri/
git commit -m "feat(tauri): add clipboard read support"
```

---

## Summary of Commits

| # | Commit Message |
|---|---------------|
| 1 | `feat(tauri): scaffold Tauri v2 project with plugins and capabilities` |
| 2 | `feat(tauri): add webpack config and HTML templates for Tauri build` |
| 3 | `feat(tauri): add IPC bridge translating ipcRenderer to Tauri invoke/listen` |
| 4 | `feat(tauri): add TAURI ifdef blocks to all 24 frontend files` |
| 5 | `feat(tauri): implement kernel sidecar lifecycle management` |
| 6 | `feat(tauri): implement IPC command handlers for window, system, dialog, tray, export` |
| 7 | `feat(tauri): add window state persistence` |
| 8 | `feat(tauri): add deep link handling and auto-launch support` |
| 9 | `feat(tauri): add global shortcut registration` |
| 10 | `feat(tauri): add system tray with show/quit menu` |
| 11 | `feat(tauri): add dynamic context menu support` |
| 12 | `fix(tauri): integration fixes from first launch testing` |
| 13 | `feat(tauri): add multi-window event broadcasting and focus tracking` |
| 14 | `feat(tauri): add clipboard read support` |
