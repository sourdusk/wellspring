const CONTAINER_BACKEND_SET = new Set(["docker", "ios", "android", "harmony"]);

export const isKernelInContainer = (): boolean => {
    return CONTAINER_BACKEND_SET.has(window.siyuan.config.system.container);
};

export const isMobile = () => {
    return document.getElementById("sidebar") ? true : false;
};

// "windows" | "linux" | "darwin" | "docker" | "android" | "ios" | "harmony"
export const getBackend = () => {
    if (isKernelInContainer()) {
        return window.siyuan.config.system.container;
    } else {
        return window.siyuan.config.system.os;
    }
};

// "desktop" | "desktop-window" | "mobile" | "browser-desktop" | "browser-mobile"
export const getFrontend = () => {
    /// #if MOBILE
    if (window.navigator.userAgent.startsWith("SiYuan/")) {
        return "mobile";
    } else {
        return "browser-mobile";
    }
    /// #else
    if (window.navigator.userAgent.startsWith("SiYuan/")) {
        if (isWindow()) {
            return "desktop-window";
        }
        return "desktop";
    } else {
        return "browser-desktop";
    }
    /// #endif
};

export const isWindow = () => {
    return document.getElementById("toolbar") ? false : true;
};

export const isTouchDevice = () => {
    return ("ontouchstart" in window) && navigator.maxTouchPoints > 1;
};

export const isArrayEqual = (arr1: string[], arr2: string[]) => {
    return arr1.length === arr2.length && arr1.every((item) => arr2.includes(item));
};

export const getRandom = (min: number, max: number) => {
    return Math.floor(Math.random() * (max - min + 1)) + min; //含最大值，含最小值
};

export const getSearch = (key: string, link = window.location.search) => {
    const params = link.substring(link.indexOf("?"));
    const hashIndex = params.indexOf("#");
    // REF https://developer.mozilla.org/zh-CN/docs/Web/API/URLSearchParams
    const urlSearchParams = new URLSearchParams(params.substring(0, hashIndex >= 0 ? hashIndex : undefined));
    return urlSearchParams.get(key);
};

export const isBrowser = () => {
    /// #if BROWSER
    return true;
    /// #else
    return false;
    /// #endif
};

export const isDynamicRef = (text: string) => {
    return /^\(\(\d{14}-\w{7} '.*'\)\)$/.test(text);
};

// Compute the correction factor for position:fixed coordinates in WebView2.
// When CSS zoom is applied (e.g., via document.documentElement.style.zoom in Tauri),
// getBoundingClientRect() returns viewport coordinates but position:fixed CSS values
// are interpreted in the zoomed coordinate space. This causes a mismatch where setting
// style.top = rect.top results in the element appearing at rect.top * zoom instead.
// Additionally, WebView2 at non-100% DPI may apply its own implicit scaling.
// This factor corrects both sources of mismatch by measuring actual vs expected position.
let _fixedPositionScale: number | undefined;
let _fixedPositionScaleZoom: string | undefined;
export const getFixedPositionScale = (): number => {
    // Invalidate cache if CSS zoom has changed
    const currentZoom = document.documentElement.style.zoom || "";
    if (_fixedPositionScale !== undefined && _fixedPositionScaleZoom === currentZoom) {
        return _fixedPositionScale;
    }
    const testEl = document.createElement("div");
    testEl.style.cssText = "position:fixed;top:100px;left:100px;width:1px;height:1px;visibility:hidden;pointer-events:none;z-index:-1";
    document.body.appendChild(testEl);
    const rect = testEl.getBoundingClientRect();
    document.body.removeChild(testEl);
    if (rect.top > 0) {
        const scale = 100 / rect.top;
        // Only apply correction if the mismatch is significant (> 1%)
        _fixedPositionScale = Math.abs(scale - 1) > 0.01 ? scale : 1;
    } else {
        _fixedPositionScale = 1;
    }
    _fixedPositionScaleZoom = currentZoom;
    return _fixedPositionScale;
};

export const isFileAnnotation = (text: string) => {
    return /^<<assets\/.+\/\d{14}-\w{7} ".+">>$/.test(text);
};

export const isValidCustomAttrName = (name: string) => {
    return /^[a-z][\-0-9a-z]*$/.test(name);
};

// REF https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/eval
export const looseJsonParse = (text: string) => {
    return Function(`"use strict";return (${text})`)();
};

export const objEquals = (a: any, b: any): boolean => {
    if (a === b) return true;
    if (typeof a === "number" && isNaN(a) && typeof b === "number" && isNaN(b)) return true;
    if (a instanceof Date && b instanceof Date) return a.getTime() === b.getTime();
    if (!a || !b || (typeof a !== "object" && typeof b !== "object")) return a === b;
    if (a.prototype !== b.prototype) return false;
    const keys = Object.keys(a);
    if (keys.length !== Object.keys(b).length) return false;
    return keys.every(k => objEquals(a[k], b[k]));
};

export const duplicateNameAddOne = (name:string) => {
    if (!name) {
        return "";
    }

    const nameMatch = name.match(/^(.*) \((\d+)\)$/);
    if (nameMatch) {
        name = `${nameMatch[1]} (${parseInt(nameMatch[2]) + 1})`;
    } else {
        name = `${name} (1)`;
    }
    return name;
};
