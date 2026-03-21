# SiYuan Electron to Tauri Migration Design

**Date:** 2026-03-20
**Approach:** Sidecar + ifdef (coexist initially, replace later)
**Goal:** Smaller binary + lower RAM footprint
**Platforms:** Windows, macOS, Linux

## Context

SiYuan is a local-first personal knowledge management app. The frontend is vanilla TypeScript/HTML/SCSS with 347 source files, of which only 24 (7%) import Electron APIs. The codebase already uses `ifdef-loader` conditional compilation with 172 `/// #if !BROWSER` blocks, and has separate webpack configs for Electron, desktop-web, and mobile builds. The Go kernel communicates with the frontend exclusively over HTTP on localhost.

This makes it unusually well-suited for a Tauri migration.

## Architecture

### Project Structure

```
app/
├── src-tauri/                    # Tauri Rust backend (new)
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Window defs, sidecar config, permissions
│   ├── build.rs
│   ├── icons/
│   ├── src/
│   │   ├── main.rs               # Entry point, plugin registration
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── window.rs         # show/hide/minimize/maximize/pin/destroy
│   │   │   ├── system.rs         # notifications, shell operations, clipboard
│   │   │   ├── dialog.rs         # file open/save, confirm/alert dialogs
│   │   │   ├── tray.rs           # tray menu creation and updates
│   │   │   ├── export.rs         # PDF export coordination
│   │   │   └── kernel.rs         # Kernel sidecar lifecycle management
│   │   ├── kernel.rs             # Process spawning, boot polling, exit handling
│   │   ├── window_state.rs       # Window state persistence (windowState.json)
│   │   └── protocol.rs           # siyuan:// deep link handling
│   └── capabilities/
│       └── default.json          # Tauri v2 permission capabilities
├── src/
│   ├── tauri/                    # Tauri frontend bridge (new)
│   │   └── bridge.ts             # Maps ipcRenderer patterns to @tauri-apps/api
│   └── ... (existing files get /// #if TAURI blocks)
├── webpack.tauri.js              # target: "web", TAURI=true
├── package.json                  # Add @tauri-apps/api, @tauri-apps/cli
└── electron-builder.yml          # Unchanged
```

### IPC Bridge

The bridge (`app/src/tauri/bridge.ts`) exports the same API surface as Electron's `ipcRenderer` but routes to Tauri's `invoke()` and `listen()` APIs. The 24 affected frontend files use `/// #if TAURI` blocks to import from the bridge instead of Electron.

Electron IPC channels map to Tauri commands:
- `ipcRenderer.send("siyuan-cmd", {cmd: "show"})` -> `invoke("cmd_show")`
- `ipcRenderer.invoke("siyuan-get", {cmd: "isFullScreen"})` -> `invoke("get_is_full_screen")`
- `ipcRenderer.on("siyuan-event", cb)` -> `listen("siyuan-event", cb)`

Each of the 18 `siyuan-cmd` sub-commands and 12 `siyuan-get` sub-commands becomes an individual `#[tauri::command]` function in Rust.

### Kernel Lifecycle

The Go kernel binary is declared as a Tauri sidecar in `tauri.conf.json` under `bundle.externalBin`. Tauri handles bundling it per-platform.

**Startup:**
1. Find available port (bind to port 0, read assigned port, close)
2. Spawn kernel sidecar: `--port`, `--wd`, `--workspace`, `--lang`
3. Poll `GET /api/system/version` (15 attempts, 500ms apart)
4. Poll `GET /api/system/bootProgress` until `progress >= 100`
5. Send `POST /api/system/uiproc?pid=<pid>`
6. Emit `kernel-ready` event to frontend

**Shutdown:**
- Window close -> frontend `exportLayout()` -> close/minimize decision
- App quit -> Rust sends `POST /api/system/exit` to each kernel
- Kernel exit codes (20, 21, 24, 25, 26) -> error window

**Multi-workspace:** Rust state holds a `Vec<Workspace>` with port, process handle, window, and tray per workspace.

### Webpack Config

`webpack.tauri.js` cloned from `webpack.config.js` with:
- `target: "web"` (not `"electron-renderer"`)
- ifdef flags: `BROWSER: false, MOBILE: false, TAURI: true`
- Output: `stage/build/tauri/`
- Node.js builtins excluded from bundle

### Platform Features

| Feature | Implementation |
|---------|---------------|
| Window state | Rust reads/writes `windowState.json` in `~/.config/siyuan/`, same format as Electron |
| System tray | `tauri-plugin-tray`, same menu structure, click toggles show/hide (Win/Linux only) |
| Global shortcuts | `tauri-plugin-global-shortcut`, first = show/hide toggle, rest broadcast |
| Deep linking | `tauri-plugin-deep-link` for `siyuan://` protocol |
| Notifications | `tauri-plugin-notification` |
| File dialogs | `tauri-plugin-dialog` for open/save/confirm/alert |
| Auto-launch | `tauri-plugin-autostart` |
| Spell check | Platform WebView defaults (no custom config initially) |
| PDF export | Phase 1: delegate to Go kernel (pandoc). Phase 2: investigate alternatives |
| Custom window controls | Existing HTML/SVG buttons work with Tauri frameless windows |
| Header manipulation | Tauri HTTP plugin or Rust proxy middleware for stripping X-Frame-Options/CSP |

## Deferred (Not In Scope)

1. Removing Electron — both builds coexist
2. Security hardening — Tauri capabilities set to allow-all initially
3. WebView tag replacement — export preview falls back to new window or iframe
4. Platform abstraction refactor — do after Tauri is stable
5. Power monitor — suspend/resume sync triggers are nice-to-have
6. Referer header manipulation — Bilibili/YouTube embed workaround
