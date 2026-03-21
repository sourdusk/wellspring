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
