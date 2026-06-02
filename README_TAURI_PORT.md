# CC3 Tauri Port

This project ports the original Electron Forge + Vite app to **Tauri v2** with a Rust backend.

## Run in development

```powershell
npm install
npm run start
```

## Build desktop app

```powershell
npm run make
```

## Notes

- Frontend is the reused Vite React app.
- Electron IPC/preload was replaced with `src/tauri-api.ts` and Tauri `invoke` calls.
- Rust backend commands live in `src-tauri/src/lib.rs`.
- Tauri/Vite integration is configured in `src-tauri/tauri.conf.json`:
  - `beforeDevCommand`: `npm run dev`
  - `beforeBuildCommand`: `npm run build`
  - `devUrl`: `http://localhost:5173`
  - `frontendDist`: `../dist`

## Windows prerequisites

Install:

- Node.js
- Rust toolchain
- Microsoft C++ Build Tools / Visual Studio Desktop C++ workload
- WebView2 runtime

Optional tools used by the app:

- `adb`
- `scrcpy`
- `mcp-hub`
- `ngrok`
- `mcp-proxy`
- `serena`
- `gh`
