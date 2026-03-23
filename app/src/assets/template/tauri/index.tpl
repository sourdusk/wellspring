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
        @keyframes loading-spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        .loading-wrapper {
            position: absolute;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 24px;
            z-index: 1;
        }
        .loading-throbber {
            width: 28px;
            height: 28px;
            border: 3px solid rgba(106, 155, 235, 0.15);
            border-top-color: #6a9beb;
            border-radius: 50%;
            animation: loading-spin 0.9s linear infinite;
        }
        .loading-logo {
            width: 36vh;
            animation: loading-pulse 2.4s ease-in-out infinite;
        }
    </style>
</head>
<body class="fn__flex-column">
<div id="loading" class="b3-dialog b3-dialog--open">
    <div class="b3-dialog__scrim" style="background-color: #1e1e1e"></div>
    <div class="loading-wrapper">
        <img class="loading-logo" src="data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgdmlld0JveD0iMTA1IDEyMCAzMDIgMjkwIgogICB3aWR0aD0iNTEyIgogICBoZWlnaHQ9IjUxMiIKICAgdmVyc2lvbj0iMS4xIgogICBpZD0ic3ZnMTQiCiAgIHNvZGlwb2RpOmRvY25hbWU9IndlbGxzcHJpbmctbG9nby5zdmciCiAgIGlua3NjYXBlOnZlcnNpb249IjEuMi4yICg3MzJhMDFkYTYzLCAyMDIyLTEyLTA5KSIKICAgeG1sbnM6aW5rc2NhcGU9Imh0dHA6Ly93d3cuaW5rc2NhcGUub3JnL25hbWVzcGFjZXMvaW5rc2NhcGUiCiAgIHhtbG5zOnNvZGlwb2RpPSJodHRwOi8vc29kaXBvZGkuc291cmNlZm9yZ2UubmV0L0RURC9zb2RpcG9kaS0wLmR0ZCIKICAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogICB4bWxuczpzdmc9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZGVmcwogICAgIGlkPSJkZWZzMTgiIC8+CiAgPHNvZGlwb2RpOm5hbWVkdmlldwogICAgIGlkPSJuYW1lZHZpZXcxNiIKICAgICBwYWdlY29sb3I9IiNmZmZmZmYiCiAgICAgYm9yZGVyY29sb3I9IiMwMDAwMDAiCiAgICAgYm9yZGVyb3BhY2l0eT0iMC4yNSIKICAgICBpbmtzY2FwZTpzaG93cGFnZXNoYWRvdz0iMiIKICAgICBpbmtzY2FwZTpwYWdlb3BhY2l0eT0iMC4wIgogICAgIGlua3NjYXBlOnBhZ2VjaGVja2VyYm9hcmQ9IjAiCiAgICAgaW5rc2NhcGU6ZGVza2NvbG9yPSIjZDFkMWQxIgogICAgIHNob3dncmlkPSJmYWxzZSIKICAgICBpbmtzY2FwZTp6b29tPSIyLjM4MjgxMjUiCiAgICAgaW5rc2NhcGU6Y3g9IjIzNy41MzQ0MyIKICAgICBpbmtzY2FwZTpjeT0iMzAzLjIxMzExIgogICAgIGlua3NjYXBlOndpbmRvdy13aWR0aD0iMjQ2MiIKICAgICBpbmtzY2FwZTp3aW5kb3ctaGVpZ2h0PSIxNDExIgogICAgIGlua3NjYXBlOndpbmRvdy14PSIyNDg5IgogICAgIGlua3NjYXBlOndpbmRvdy15PSItOSIKICAgICBpbmtzY2FwZTp3aW5kb3ctbWF4aW1pemVkPSIxIgogICAgIGlua3NjYXBlOmN1cnJlbnQtbGF5ZXI9InN2ZzE0IiAvPgogIDwhLS0gQmFzZSBwb29sIC0tPgogIDxsaW5lCiAgICAgeDE9IjEzMCIKICAgICB5MT0iMzgwIgogICAgIHgyPSIzODIiCiAgICAgeTI9IjM4MCIKICAgICBzdHJva2U9IiM2YTliZWIiCiAgICAgc3Ryb2tlLXdpZHRoPSIxNiIKICAgICBzdHJva2UtbGluZWNhcD0icm91bmQiCiAgICAgaWQ9ImxpbmUyIiAvPgogIDwhLS0gRml2ZSBhbmd1bGFyIGpldHMgYXMgZmlsbGVkIHRyaWFuZ2xlcyAocG9pbnRlZC90YXBlcmVkKSAtLT4KICA8IS0tIEZhciBsZWZ0IC0tPgogIDwhLS0gSW5uZXIgbGVmdCAtLT4KICA8IS0tIENlbnRlciAtLT4KICA8cGF0aAogICAgIGlkPSJwb2x5Z29uOCIKICAgICBzdHlsZT0iZmlsbDojNmE5YmViO3N0cm9rZS13aWR0aDoxLjEzODkyIgogICAgIGQ9Im0gMjU3LjE3NCwyMTAuNzc3NTUgLTYuODAwNiw0LjcwOTY3IC02LjgwMDYxLDE1OC4yNTIzMyBoIDI3LjIwMjQzIGwgLTYuODAwNjEsLTE1OC4yNTIzMyB6IG0gMi4yOTE1Nyw0MS4zNTAzMiIKICAgICBzb2RpcG9kaTpub2RldHlwZXM9ImNjY2NjYyIgLz4KICA8cGF0aAogICAgIGlkPSJwb2x5Z29uOC04IgogICAgIHN0eWxlPSJmaWxsOiM2YTliZWI7c3Ryb2tlLXdpZHRoOjEuMDk0ODIiCiAgICAgZD0ibSAzMjYuOTc1NjYsMjMxLjEzNjk5IC03LjY5NTI1LDIuNDQzNTUgLTQ0LjQxNjc3LDEzOS40OTAwOSAyNi4yNzU1Myw3LjA0MDUgMzEuMjc5LC0xNDMuMDEwMzMgeiBtIC03LjY3NTkzLDM3LjUwMDg3IgogICAgIHNvZGlwb2RpOm5vZGV0eXBlcz0iY2NjY2NjIiAvPgogIDxwYXRoCiAgICAgaWQ9InBvbHlnb244LTgtMSIKICAgICBzdHlsZT0iZmlsbDojNmE5YmViO3N0cm9rZS13aWR0aDoxLjA0NjA0IgogICAgIGQ9Im0gMzg1LjY2OTU4LDI2Mi4wMzg5IC03Ljg3NTksMC4wNDAzIC03Mi42MzU3MywxMTIuMjA3NTYgMjMuNTU3OTksMTMuNjAxMjIgNjAuODU2NzQsLTExOS4wMDgxOCB6IG0gLTE1LjQ1NTgxLDMxLjM1MzM4IgogICAgIHNvZGlwb2RpOm5vZGV0eXBlcz0iY2NjY2NjIiAvPgogIDxwYXRoCiAgICAgaWQ9InBvbHlnb244LTgtMyIKICAgICBzdHlsZT0iZmlsbDojNmE5YmViO3N0cm9rZS13aWR0aDoxLjA5NDgyIgogICAgIGQ9Im0gMTg0LjQ0NTU1LDIzMS4wODM5MiA3LjY5NTI1LDIuNDQzNTUgNDQuNDE2NzcsMTM5LjQ5MDA5IC0yNi4yNzU1Myw3LjA0MDUgLTMxLjI3OSwtMTQzLjAxMDMzIHogbSA3LjY3NTkzLDM3LjUwMDg3IgogICAgIHNvZGlwb2RpOm5vZGV0eXBlcz0iY2NjY2NjIiAvPgogIDxwYXRoCiAgICAgaWQ9InBvbHlnb244LTgtMS01IgogICAgIHN0eWxlPSJmaWxsOiM2YTliZWI7c3Ryb2tlLXdpZHRoOjEuMDQ2MDQiCiAgICAgZD0ibSAxMjUuNzUxNjMsMjYxLjk4NTgzIDcuODc1OSwwLjA0MDMgNzIuNjM1NzMsMTEyLjIwNzU2IC0yMy41NTc5OSwxMy42MDEyMiAtNjAuODU2NzQsLTExOS4wMDgxOCB6IG0gMTUuNDU1ODEsMzEuMzUzMzgiCiAgICAgc29kaXBvZGk6bm9kZXR5cGVzPSJjY2NjY2MiIC8+CiAgPCEtLSBJbm5lciByaWdodCAtLT4KICA8IS0tIEZhciByaWdodCAtLT4KPC9zdmc+Cg==">
        <div class="loading-throbber"></div>
    </div>
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
</body>
</html>
