import {App} from "../index";
import {Constants} from "../constants";
/// #if !BROWSER
/// #if !TAURI
import { ipcRenderer } from "electron";
/// #endif
/// #endif
/// #if TAURI
import {send} from "../tauri/bridge";
/// #endif

export const closeWindow = async (app: App) => {
    for (let i = 0; i < app.plugins.length; i++) {
        try {
            await app.plugins[i].onunload();
        } catch (e) {
            console.error(e);
        }
    }
    /// #if !TAURI
    ipcRenderer.send(Constants.SIYUAN_CMD, "destroy");
    /// #endif
    /// #if TAURI
    send(Constants.SIYUAN_CMD, "destroy");
    /// #endif
};
