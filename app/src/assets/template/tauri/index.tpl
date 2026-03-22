<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <!-- https://electronjs.org/docs/tutorial/security#csp-meta-tag
    <meta http-equiv="Content-Security-Policy" content="script-src 'self'"/>-->
    <!-- Font preloads and PDF.js are omitted here; they are unavailable on the Tauri asset
         protocol and will be loaded when the page navigates to the kernel-served URL. -->
    <style>
        @keyframes loading-pulse {
            0% { opacity: 0.3; }
            50% { opacity: 1; }
            100% { opacity: 0.3; }
        }
        @keyframes loading-bar {
            0% { transform: translateX(-100%); }
            100% { transform: translateX(100%); }
        }
        .loading-throbber {
            position: absolute;
            bottom: 38%;
            width: 200px;
            height: 3px;
            background: rgba(106, 155, 235, 0.15);
            border-radius: 3px;
            overflow: hidden;
        }
        .loading-throbber::after {
            content: '';
            display: block;
            width: 50%;
            height: 100%;
            background: #6a9beb;
            border-radius: 3px;
            animation: loading-bar 1.4s ease-in-out infinite;
        }
        .loading-logo {
            position: absolute;
            width: 36vh;
            animation: loading-pulse 2.4s ease-in-out infinite;
        }
    </style>
</head>
<body class="fn__flex-column">
<div id="loading" class="b3-dialog b3-dialog--open">
    <div class="b3-dialog__scrim" style="background-color: #1e1e1e"></div>
    <img class="loading-logo" src="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIiB3aWR0aD0iNTEyIiBoZWlnaHQ9IjUxMiI+DQogIDwhLS0gQmFzZSBwb29sIC0tPg0KICA8cGF0aCBkPSJNIDE0MCAzODAgQyAxODAgMzU1LCAzMzIgMzU1LCAzNzIgMzgwIg0KICAgICAgICBmaWxsPSJub25lIiBzdHJva2U9IiM2YTliZWIiIHN0cm9rZS13aWR0aD0iMTYiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIvPg0KDQogIDwhLS0gRml2ZSBhbmd1bGFyIGpldHMgYXMgZmlsbGVkIHRyaWFuZ2xlcyAocG9pbnRlZC90YXBlcmVkKSAtLT4NCg0KICA8IS0tIEZhciBsZWZ0IC0tPg0KICA8cG9seWdvbiBwb2ludHM9IjEzMCwyMTUgMTkyLDM2NSAyMDgsMzU1IiBmaWxsPSIjNmE5YmViIi8+DQoNCiAgPCEtLSBJbm5lciBsZWZ0IC0tPg0KICA8cG9seWdvbiBwb2ludHM9IjE5OCwxNzUgMjIwLDM3MCAyMzYsMzYwIiBmaWxsPSIjNmE5YmViIi8+DQoNCiAgPCEtLSBDZW50ZXIgLS0+DQogIDxwb2x5Z29uIHBvaW50cz0iMjU2LDE0NSAyNDgsMzY4IDI2NCwzNjgiIGZpbGw9IiM2YTliZWIiLz4NCg0KICA8IS0tIElubmVyIHJpZ2h0IC0tPg0KICA8cG9seWdvbiBwb2ludHM9IjMxNCwxNzUgMjc2LDM2MCAyOTIsMzcwIiBmaWxsPSIjNmE5YmViIi8+DQoNCiAgPCEtLSBGYXIgcmlnaHQgLS0+DQogIDxwb2x5Z29uIHBvaW50cz0iMzgyLDIxNSAzMDQsMzU1IDMyMCwzNjUiIGZpbGw9IiM2YTliZWIiLz4NCjwvc3ZnPg0K">
    <div class="loading-throbber"></div>
    <button onclick="window.location.reload()" id="loadingRefresh"
            style="display: none;position: absolute;bottom: 16px;background: transparent;border: 1px solid #4285f4;color: #4285f4;border-radius: 6px;line-height: 20px;padding: 4px 8px;">
        Click to Refresh<br>点　击　刷　新
    </button>
</div>
<div id="toolbar" class="toolbar fn__flex"></div>
<div class="fn__flex-1 fn__flex">
    <div id="dockLeft" class="dock dock--vertical"></div>
    <div id="layouts" class="layout fn__flex-1"></div>
    <div id="dockRight" class="dock dock--vertical"></div>
</div>
<div id="dockBottom" class="dock fn__none"></div>
<div id="status" class="fn__flex status"></div>
<div id="commonMenu" class="b3-menu fn__none">
    <div class="b3-menu__title fn__none">
        <svg class="b3-menu__icon"><use xlink:href="#iconLeft"></use></svg>
        <span class="b3-menu__label"></span>
    </div>
    <div class="b3-menu__items"></div>
</div>
<div id="message" class="b3-snackbars"></div>
<div id="tooltip" class="tooltip fn__none"></div>
<script>
    setTimeout(() => {
        const refreshElement = document.getElementById("loadingRefresh")
        if (refreshElement) {
            refreshElement.style.display = ""
        }
    }, 7000)
</script>
</body>
</html>
