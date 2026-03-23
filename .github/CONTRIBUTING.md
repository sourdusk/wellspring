[中文](https://github.com/siyuan-note/siyuan/blob/master/.github/CONTRIBUTING_zh_CN.md)

## Get the Source Code

* `git clone git@github.com:siyuan-note/siyuan.git`
* Switch to the dev branch: `git checkout dev`

## NPM Dependencies

Install pnpm: `npm install -g pnpm@10.30.3`

<details>
<summary>For China mainland</summary>

Set the Electron mirror environment variable and install Electron:

* macOS/Linux: `ELECTRON_MIRROR=https://npmmirror.com/mirrors/electron/ pnpm install electron@40.8.1 -D`
* Windows:
  * `SET ELECTRON_MIRROR=https://npmmirror.com/mirrors/electron/`
  * `pnpm install electron@40.8.1 -D`

NPM mirror:

* Use npmmirror China mirror repository `pnpm --registry https://registry.npmmirror.com/ i`
* Revert to using official repository `pnpm --registry https://registry.npmjs.org i`
</details>

In the `app/` folder, run:

* `pnpm install electron@40.8.1 -D`
* `pnpm run dev`
* `pnpm run start`

**Note:** The dev environment does not start the kernel automatically — you need to start it manually first (see below).

## Kernel

1. Install the latest version of [Go](https://go.dev/)
2. Enable CGO by setting the environment variable `CGO_ENABLED=1`

### Desktop

* `cd kernel`
* Windows: `go build --tags "fts5" -o "../app/kernel/Wellspring-Kernel.exe"`
* Linux/macOS: `go build --tags "fts5" -o "../app/kernel/Wellspring-Kernel"`
* `cd ../app/kernel`
* Windows: `./Wellspring-Kernel.exe --wd=.. --mode=dev`
* Linux/macOS: `./Wellspring-Kernel --wd=.. --mode=dev`

### iOS

* `cd kernel`
* `gomobile bind --tags fts5 -ldflags '-s -w' -v -o ./ios/iosk.xcframework -target=ios ./mobile/`
* https://github.com/siyuan-note/siyuan-ios

### Android

* `cd kernel`
* `set JAVA_TOOL_OPTIONS=-Dfile.encoding=UTF-8`
* `gomobile bind --tags fts5 -ldflags "-s -w"  -v -o kernel.aar -target=android/arm64 -androidapi 26 ./mobile/`
* https://github.com/siyuan-note/siyuan-android

### HarmonyOS

HarmonyOS builds are only supported on Linux. You must install the Harmony SDK and patch the Go source code.

* `cd kernel/harmony`
* `./build.sh` (or `./build-win.sh` for the Windows emulator)
* https://github.com/siyuan-note/siyuan-harmony

Required Go source code patches:

1. go/src/runtime/vim tls_arm64.s

   Change the ending `DATA runtime·tls_g+0(SB)/8, $16` to `DATA runtime·tls_g+0(SB)/8, $-144`

2. go/src/runtime/cgo/gcc_android.c

   Empty the `inittls` function body

   ```c
   inittls(void **tlsg, void **tlsbase)
   {
     return;
   }
   ```
3. go/src/net/cgo_resold.go
   `C.size_t(len(b))` to `C.socklen_t(len(b))`

For more details, see https://github.com/siyuan-note/siyuan/issues/13184
