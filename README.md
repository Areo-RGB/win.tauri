# CC3 egui

Native Rust desktop port of CC3 using [`egui`](https://github.com/emilk/egui) through `eframe`.

This replaces the former Tauri/Vite/WebView stack with a pure Rust GUI.

## Run locally

```powershell
cargo run
```

## Build release exe

```powershell
cargo build --release
```

The executable will be created at:

```text
target/release/cc3-egui.exe
```

## Included tabs

- Projects / Git helpers
- SCRCPY / ADB helpers
- MCP Hub launcher and REST controls
- Command runner
- CLI launcher shortcuts
- Placeholder Network, Security, and Settings tabs

## Optional external tools used by buttons

- `adb`
- `scrcpy`
- `mcp-hub`
- `ngrok`
- `mcp-proxy`
- `serena`
- `gh`
- `npx kill-port`
