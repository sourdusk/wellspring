import {invoke} from "@tauri-apps/api/core";
import {listen, emit, type UnlistenFn} from "@tauri-apps/api/event";
import {getCurrentWindow} from "@tauri-apps/api/window";

// Maps Electron ipcRenderer.send(channel, data) pattern
export function send(channel: string, data?: any): void {
    switch (channel) {
        case "wellspring-cmd": {
            const cmd = typeof data === "string" ? data : data?.cmd;
            if (cmd) {
                invoke(`cmd_${toSnakeCase(cmd)}`, {data: typeof data === "string" ? {} : data}).catch(console.error);
            }
            break;
        }
        case "wellspring-config-tray":
            invoke("config_tray", {data}).catch(console.error);
            break;
        case "wellspring-export-pdf":
            invoke("export_pdf", {data}).catch(console.error);
            break;
        case "wellspring-export-newwindow":
            invoke("export_new_window", {data}).catch(console.error);
            break;
        case "wellspring-quit":
            invoke("quit_app").catch(console.error);
            break;
        case "wellspring-show-window":
            getCurrentWindow().show().catch(console.error);
            break;
        case "wellspring-open-window":
            invoke("open_window", {data}).catch(console.error);
            break;
        case "wellspring-open-workspace":
            invoke("open_workspace", {data}).catch(console.error);
            break;
        case "wellspring-hotkey":
            invoke("register_hotkey", {data}).catch(console.error);
            break;
        case "wellspring-send-windows":
            emit("wellspring-send-windows", data).catch(console.error);
            break;
        case "wellspring-auto-launch":
            invoke("set_auto_launch", {data}).catch(console.error);
            break;
        case "wellspring-first-init":
            invoke("first_init", {data}).catch(console.error);
            break;
        case "wellspring-ready-to-show":
            getCurrentWindow().show().catch(console.error);
            break;
        case "wellspring-context-menu":
            invoke("show_context_menu", {data}).catch(console.error);
            break;
        case "wellspring-confirm-dialog":
            invoke("show_confirm_dialog", {data}).catch(console.error);
            break;
        case "wellspring-alert-dialog":
            invoke("show_alert_dialog", {data}).catch(console.error);
            break;
        case "wellspring-first-quit":
            invoke("first_quit").catch(console.error);
            break;
        case "wellspring-event":
            invoke("register_window_events").catch(console.error);
            break;
        default:
            console.warn(`[tauri bridge] unhandled send channel: ${channel}`);
    }
}

// Maps Electron ipcRenderer.invoke(channel, data) pattern
export async function invokeHandler(channel: string, data?: any): Promise<any> {
    switch (channel) {
        case "wellspring-init":
            return invoke("wellspring_init", {data});
        case "wellspring-get": {
            const cmd = data?.cmd;
            if (cmd) {
                try {
                    return await invoke(`get_${toSnakeCase(cmd)}`, {data});
                } catch (e) {
                    console.warn(`[tauri bridge] wellspring-get command failed: get_${toSnakeCase(cmd)}`, e);
                    return null;
                }
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
        case "wellspring-event":
        case "wellspring-open-url":
        case "wellspring-open-file":
        case "wellspring-save-close":
        case "wellspring-send-windows":
        case "wellspring-hotkey":
        case "wellspring-export-pdf":
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
