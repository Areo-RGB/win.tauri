use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tauri::Manager;

#[derive(Serialize)]
struct Device {
    id: String,
    state: String,
}

#[derive(Serialize)]
struct ActionResult {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct ExecResult {
    success: bool,
    stdout: String,
    stderr: String,
}

#[derive(Serialize)]
struct ConfigScanResult {
    #[serde(rename = "mcpConfigs")]
    mcp_configs: Vec<String>,
    #[serde(rename = "ngrokConfigs")]
    ngrok_configs: Vec<String>,
}

#[derive(Serialize)]
struct ApiResult {
    success: bool,
    data: Option<Value>,
    message: Option<String>,
}

fn home_dir() -> PathBuf {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn spawn_detached(program: &str, args: &[String]) -> Result<(), String> {
    let mut command = Command::new(program);
    command.args(args);
    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x00000008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        command.creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP);
    }

    command.spawn().map(|_| ()).map_err(|e| e.to_string())
}

fn exec_shell(cmd: &str, cwd: &str) -> ExecResult {
    #[cfg(windows)]
    let output = Command::new("cmd")
        .args(["/C", cmd])
        .current_dir(cwd)
        .output();

    #[cfg(not(windows))]
    let output = Command::new("sh")
        .args(["-c", cmd])
        .current_dir(cwd)
        .output();

    match output {
        Ok(out) => ExecResult {
            success: out.status.success(),
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
        },
        Err(e) => ExecResult {
            success: false,
            stdout: String::new(),
            stderr: e.to_string(),
        },
    }
}

fn http_json(method: &str, url: String, body: Option<Value>) -> ApiResult {
    let result = match method {
        "POST" => {
            let request = ureq::post(&url).set("Content-Type", "application/json");
            if let Some(body) = body {
                request.send_json(body)
            } else {
                request.call()
            }
        }
        _ => ureq::get(&url).call(),
    };

    match result {
        Ok(response) => match response.into_json::<Value>() {
            Ok(data) => ApiResult { success: true, data: Some(data), message: None },
            Err(e) => ApiResult { success: false, data: None, message: Some(e.to_string()) },
        },
        Err(e) => ApiResult { success: false, data: None, message: Some(e.to_string()) },
    }
}

#[tauri::command]
fn window_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
fn window_toggle_maximize(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().map_err(|e| e.to_string())? {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn window_close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_devices() -> Vec<Device> {
    let output = Command::new("adb").arg("devices").output();
    let Ok(out) = output else { return vec![] };
    let stdout = String::from_utf8_lossy(&out.stdout);

    stdout
        .lines()
        .skip(1)
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let mut parts = line.split_whitespace();
            Some(Device {
                id: parts.next()?.to_string(),
                state: parts.next().unwrap_or("unknown").to_string(),
            })
        })
        .collect()
}

#[tauri::command]
fn launch_scrcpy(device_id: Option<String>, custom_args: Vec<String>) -> Result<(), String> {
    let mut args = Vec::new();
    if let Some(id) = device_id.filter(|s| !s.trim().is_empty()) {
        args.push("-s".to_string());
        args.push(id);
    }
    args.extend(custom_args);
    spawn_detached("scrcpy", &args)
}

#[tauri::command]
fn adb_connect(ip: String) -> ActionResult {
    match Command::new("adb").args(["connect", ip.as_str()]).output() {
        Ok(out) => ActionResult {
            success: out.status.success(),
            message: if out.status.success() {
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            } else {
                String::from_utf8_lossy(&out.stderr).trim().to_string()
            },
        },
        Err(e) => ActionResult { success: false, message: e.to_string() },
    }
}

#[tauri::command]
fn adb_screenshot(device_id: String) -> ActionResult {
    let picture_path = home_dir()
        .join("Pictures")
        .join(format!("screenshot_{}.png", chrono_like_timestamp()));

    let cmd = if device_id.trim().is_empty() {
        format!("adb exec-out screencap -p > \"{}\"", picture_path.display())
    } else {
        format!("adb -s {} exec-out screencap -p > \"{}\"", shell_escape(&device_id), picture_path.display())
    };

    #[cfg(windows)]
    let output = Command::new("cmd").args(["/C", &cmd]).output();

    #[cfg(not(windows))]
    let output = Command::new("sh").args(["-c", &cmd]).output();

    match output {
        Ok(out) if out.status.success() => ActionResult { success: true, message: format!("Saved to {}", picture_path.display()) },
        Ok(out) => ActionResult { success: false, message: String::from_utf8_lossy(&out.stderr).to_string() },
        Err(e) => ActionResult { success: false, message: e.to_string() },
    }
}

#[tauri::command]
fn scan_configs() -> ConfigScanResult {
    let mcp_path = home_dir().join(".config").join("mcp-hub");
    let ngrok_path = home_dir().join(".config").join("ngrok");

    let mcp_configs = fs::read_dir(mcp_path)
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|entry| entry.file_name().into_string().ok())
                .filter(|name| name.ends_with(".json"))
                .collect()
        })
        .unwrap_or_default();

    let ngrok_configs = fs::read_dir(ngrok_path)
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|entry| entry.file_name().into_string().ok())
                .filter(|name| name.ends_with(".yml") || name.ends_with(".yaml"))
                .collect()
        })
        .unwrap_or_default();

    ConfigScanResult { mcp_configs, ngrok_configs }
}

#[tauri::command]
fn launch_mcp(port: String, config_file_name: String) -> Result<(), String> {
    let config_path = home_dir().join(".config").join("mcp-hub").join(config_file_name);
    spawn_detached("mcp-hub", &["--port".into(), port, "--config".into(), config_path.display().to_string()])
}

#[tauri::command]
fn launch_ngrok(port_or_address: String, url: String) -> Result<(), String> {
    let normalized_url = if url.trim().is_empty() {
        String::new()
    } else if url.starts_with("http://") || url.starts_with("https://") {
        url
    } else {
        format!("https://{}", url)
    };

    let mut args = vec!["http".to_string(), port_or_address];
    if !normalized_url.is_empty() {
        args.push("--url".to_string());
        args.push(normalized_url);
    }
    spawn_detached("ngrok", &args)
}

#[tauri::command]
fn launch_kill_port(port: String) -> Result<(), String> {
    spawn_detached("npx", &["-y".into(), "kill-port".into(), port])
}

#[tauri::command]
fn launch_mcp_proxy(port: String) -> Result<(), String> {
    spawn_detached(
        "mcp-proxy",
        &[
            "--port".into(),
            port,
            "node".into(),
            r"C:\Users\paul\Documents\.projects\mcp-server\DesktopCommanderMCP\dist\index.js".into(),
        ],
    )
}

#[tauri::command]
fn launch_serena(port: String) -> Result<(), String> {
    spawn_detached(
        "serena",
        &[
            "start-mcp-server".into(),
            "--transport".into(),
            "streamable-http".into(),
            "--host".into(),
            "0.0.0.0".into(),
            "--port".into(),
            port,
            "--mode".into(),
            "no-onboarding".into(),
            "--mode".into(),
            "query-projects".into(),
            "--project-from-cwd".into(),
            "--context".into(),
            "codex".into(),
        ],
    )
}

#[tauri::command]
fn mcp_api_health(base_url: String) -> ApiResult {
    http_json("GET", format!("{}/api/health", base_url.trim_end_matches('/')), None)
}

#[tauri::command]
fn mcp_api_refresh_all(base_url: String) -> ApiResult {
    http_json("POST", format!("{}/api/refresh", base_url.trim_end_matches('/')), None)
}

#[tauri::command]
fn mcp_api_server_start(base_url: String, server_name: String) -> ApiResult {
    http_json(
        "POST",
        format!("{}/api/servers/start", base_url.trim_end_matches('/')),
        Some(serde_json::json!({ "server_name": server_name })),
    )
}

#[tauri::command]
fn mcp_api_server_stop(base_url: String, server_name: String) -> ApiResult {
    http_json(
        "POST",
        format!("{}/api/servers/stop?disable=false", base_url.trim_end_matches('/')),
        Some(serde_json::json!({ "server_name": server_name })),
    )
}

#[tauri::command]
fn mcp_api_server_disable(base_url: String, server_name: String) -> ApiResult {
    http_json(
        "POST",
        format!("{}/api/servers/stop?disable=true", base_url.trim_end_matches('/')),
        Some(serde_json::json!({ "server_name": server_name })),
    )
}

#[tauri::command]
fn mcp_api_server_refresh(base_url: String, server_name: String) -> ApiResult {
    http_json(
        "POST",
        format!("{}/api/servers/refresh", base_url.trim_end_matches('/')),
        Some(serde_json::json!({ "server_name": server_name })),
    )
}

#[tauri::command]
fn mcp_api_restart(base_url: String) -> ApiResult {
    http_json("POST", format!("{}/api/restart", base_url.trim_end_matches('/')), None)
}

#[tauri::command]
fn git_status(cwd: String) -> ExecResult {
    exec_shell("git status", &cwd)
}

#[tauri::command]
fn git_remote(cwd: String) -> ExecResult {
    exec_shell("git remote -v", &cwd)
}

#[tauri::command]
fn git_connect_remote(cwd: String, url: String) -> ExecResult {
    exec_shell(&format!("git remote add origin {}", shell_escape(&url)), &cwd)
}

#[tauri::command]
fn git_create_remote(cwd: String) -> ExecResult {
    exec_shell("gh repo create --private --source . --push", &cwd)
}

#[tauri::command]
fn git_fetch(cwd: String) -> ExecResult {
    exec_shell("git fetch", &cwd)
}

#[tauri::command]
fn git_stage_push(cwd: String) -> ExecResult {
    exec_shell("git add . && git commit -m \"Auto-commit from CC3\" && git push", &cwd)
}

#[tauri::command]
fn zip_project(cwd: String) -> ExecResult {
    exec_shell("git archive --format=zip HEAD -o project_export.zip", &cwd)
}

fn shell_escape(value: &str) -> String {
    if value.chars().all(|c| c.is_ascii_alphanumeric() || "-_.:/\\".contains(c)) {
        value.to_string()
    } else {
        format!("\"{}\"", value.replace('"', "\\\""))
    }
}

fn chrono_like_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            window_minimize,
            window_toggle_maximize,
            window_close,
            get_devices,
            launch_scrcpy,
            adb_connect,
            adb_screenshot,
            scan_configs,
            launch_mcp,
            launch_ngrok,
            launch_kill_port,
            launch_mcp_proxy,
            launch_serena,
            mcp_api_health,
            mcp_api_refresh_all,
            mcp_api_server_start,
            mcp_api_server_stop,
            mcp_api_server_disable,
            mcp_api_server_refresh,
            mcp_api_restart,
            git_status,
            git_remote,
            git_connect_remote,
            git_create_remote,
            git_fetch,
            git_stage_push,
            zip_project,
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(false);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
