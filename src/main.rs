#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CC3")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([920.0, 620.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CC3",
        options,
        Box::new(|cc| Ok(Box::new(Cc3App::new(cc)))),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Projects,
    McpHub,
    CmdRunner,
    Cli,
    Scrcpy,
    Network,
    Security,
    Settings,
}

struct Cc3App {
    tab: Tab,
    output: String,
    project_dir: String,
    remote_url: String,
    adb_ip: String,
    devices: Vec<Device>,
    selected_device: String,
    scrcpy_args: String,
    mcp_port: String,
    mcp_config: String,
    ngrok_target: String,
    ngrok_url: String,
    kill_port: String,
    proxy_port: String,
    serena_port: String,
    api_base_url: String,
    api_server_name: String,
    mcp_configs: Vec<String>,
    ngrok_configs: Vec<String>,
}

#[derive(Clone)]
struct Device {
    id: String,
    state: String,
}

impl Cc3App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            tab: Tab::Projects,
            output: "Ready.".to_owned(),
            project_dir: r"C:\Users\paul\Documents\.projects\pc.tool".to_owned(),
            remote_url: String::new(),
            adb_ip: String::new(),
            devices: Vec::new(),
            selected_device: String::new(),
            scrcpy_args: "--max-size 1280".to_owned(),
            mcp_port: "37373".to_owned(),
            mcp_config: "mcp.json".to_owned(),
            ngrok_target: "37373".to_owned(),
            ngrok_url: String::new(),
            kill_port: "37373".to_owned(),
            proxy_port: "8000".to_owned(),
            serena_port: "8001".to_owned(),
            api_base_url: "http://localhost:37373".to_owned(),
            api_server_name: "desktop-commander".to_owned(),
            mcp_configs: Vec::new(),
            ngrok_configs: Vec::new(),
        }
    }

    fn set_output(&mut self, title: &str, text: impl Into<String>) {
        self.output = format!("[{title}]\n{}", text.into());
    }

    fn refresh_devices(&mut self) {
        self.devices = get_devices();
        if self.selected_device.is_empty() {
            if let Some(first) = self.devices.first() {
                self.selected_device = first.id.clone();
            }
        }
        self.set_output("ADB DEVICES", format_devices(&self.devices));
    }

    fn scan_configs(&mut self) {
        self.mcp_configs = list_matching(home_dir().join(".config").join("mcp-hub"), |name| name.ends_with(".json"));
        self.ngrok_configs = list_matching(home_dir().join(".config").join("ngrok"), |name| name.ends_with(".yml") || name.ends_with(".yaml"));
        self.set_output(
            "CONFIG SCAN",
            format!(
                "MCP configs:\n{}\n\nNgrok configs:\n{}",
                empty_or_join(&self.mcp_configs),
                empty_or_join(&self.ngrok_configs),
            ),
        );
    }

    fn left_tabs(&mut self, ui: &mut egui::Ui) {
        ui.heading("CC3");
        ui.label("Rust + egui");
        ui.separator();
        ui.selectable_value(&mut self.tab, Tab::Projects, "PROJECTS");
        ui.selectable_value(&mut self.tab, Tab::McpHub, "MCP-HUB");
        ui.selectable_value(&mut self.tab, Tab::CmdRunner, "CMD RUNNER");
        ui.selectable_value(&mut self.tab, Tab::Cli, "CLI");
        ui.selectable_value(&mut self.tab, Tab::Scrcpy, "SCRCPY");
        ui.selectable_value(&mut self.tab, Tab::Network, "NETWORK");
        ui.selectable_value(&mut self.tab, Tab::Security, "SECURITY");
        ui.selectable_value(&mut self.tab, Tab::Settings, "SETTINGS");
    }

    fn projects(&mut self, ui: &mut egui::Ui) {
        ui.heading("Projects // Git");
        ui.label("Project directory");
        ui.text_edit_singleline(&mut self.project_dir);
        ui.label("Remote URL");
        ui.text_edit_singleline(&mut self.remote_url);
        ui.add_space(8.0);

        ui.horizontal_wrapped(|ui| {
            if ui.button("Git status").clicked() {
                let cwd = self.project_dir.clone();
                self.set_output("git status", exec_shell("git status", &cwd));
            }
            if ui.button("Git remote").clicked() {
                let cwd = self.project_dir.clone();
                self.set_output("git remote", exec_shell("git remote -v", &cwd));
            }
            if ui.button("Connect remote").clicked() {
                let cwd = self.project_dir.clone();
                let cmd = format!("git remote add origin {}", shell_escape(&self.remote_url));
                self.set_output("connect remote", exec_shell(&cmd, &cwd));
            }
            if ui.button("Create GH remote").clicked() {
                let cwd = self.project_dir.clone();
                self.set_output("create remote", exec_shell("gh repo create --private --source . --push", &cwd));
            }
            if ui.button("Fetch").clicked() {
                let cwd = self.project_dir.clone();
                self.set_output("git fetch", exec_shell("git fetch", &cwd));
            }
            if ui.button("Stage + push").clicked() {
                let cwd = self.project_dir.clone();
                self.set_output("stage + push", exec_shell("git add . && git commit -m \"Auto-commit from CC3\" && git push", &cwd));
            }
            if ui.button("Zip project").clicked() {
                let cwd = self.project_dir.clone();
                self.set_output("zip project", exec_shell("git archive --format=zip HEAD -o project_export.zip", &cwd));
            }
        });
    }

    fn scrcpy(&mut self, ui: &mut egui::Ui) {
        ui.heading("Scrcpy // ADB");
        ui.horizontal_wrapped(|ui| {
            if ui.button("Refresh devices").clicked() {
                self.refresh_devices();
            }
            if ui.button("Launch scrcpy").clicked() {
                let mut args = Vec::new();
                if !self.selected_device.trim().is_empty() {
                    args.push("-s".to_owned());
                    args.push(self.selected_device.clone());
                }
                args.extend(split_args(&self.scrcpy_args));
                self.set_output("scrcpy", spawn_detached("scrcpy", &args));
            }
            if ui.button("Screenshot").clicked() {
                let id = self.selected_device.clone();
                self.set_output("screenshot", adb_screenshot(&id));
            }
        });

        egui::ComboBox::from_label("Selected device")
            .selected_text(if self.selected_device.is_empty() { "Default device" } else { self.selected_device.as_str() })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected_device, String::new(), "Default device");
                for device in &self.devices {
                    ui.selectable_value(&mut self.selected_device, device.id.clone(), format!("{} [{}]", device.id, device.state));
                }
            });

        ui.label("scrcpy args");
        ui.text_edit_singleline(&mut self.scrcpy_args);
        ui.separator();
        ui.label("Wireless ADB target");
        ui.text_edit_singleline(&mut self.adb_ip);
        if ui.button("ADB connect").clicked() {
            let args = vec!["connect".to_owned(), self.adb_ip.clone()];
            self.set_output("adb connect", run_command("adb", &args, None));
            self.devices = get_devices();
        }
    }

    fn mcp_hub(&mut self, ui: &mut egui::Ui) {
        ui.heading("MCP Hub // Control");
        ui.group(|ui| {
            ui.label("MCP Hub");
            ui.horizontal(|ui| {
                ui.label("Port");
                ui.text_edit_singleline(&mut self.mcp_port);
            });
            ui.horizontal(|ui| {
                ui.label("Config");
                ui.text_edit_singleline(&mut self.mcp_config);
            });
            if ui.button("Start MCP Hub").clicked() {
                let config_path = home_dir().join(".config").join("mcp-hub").join(&self.mcp_config);
                let args = vec!["--port".to_owned(), self.mcp_port.clone(), "--config".to_owned(), config_path.display().to_string()];
                self.set_output("mcp-hub", spawn_detached("mcp-hub", &args));
            }
            if ui.button("Scan configs").clicked() {
                self.scan_configs();
            }
        });

        ui.add_space(8.0);
        ui.group(|ui| {
            ui.label("Ngrok");
            ui.horizontal(|ui| {
                ui.label("Port/address");
                ui.text_edit_singleline(&mut self.ngrok_target);
            });
            ui.horizontal(|ui| {
                ui.label("URL/domain");
                ui.text_edit_singleline(&mut self.ngrok_url);
            });
            if ui.button("Start ngrok").clicked() {
                let mut args = vec!["http".to_owned(), self.ngrok_target.clone()];
                if !self.ngrok_url.trim().is_empty() {
                    args.push("--url".to_owned());
                    args.push(normalize_url(&self.ngrok_url));
                }
                self.set_output("ngrok", spawn_detached("ngrok", &args));
            }
        });

        ui.add_space(8.0);
        ui.heading("MCP Hub REST API");
        ui.label("Base URL");
        ui.text_edit_singleline(&mut self.api_base_url);
        ui.label("Server name");
        ui.text_edit_singleline(&mut self.api_server_name);
        ui.horizontal_wrapped(|ui| {
            if ui.button("Health").clicked() {
                self.set_output("api health", http_json("GET", format!("{}/api/health", trim_url(&self.api_base_url)), None));
            }
            if ui.button("Refresh all").clicked() {
                self.set_output("api refresh", http_json("POST", format!("{}/api/refresh", trim_url(&self.api_base_url)), None));
            }
            if ui.button("Start server").clicked() {
                self.set_output("server start", self.server_api("start", false));
            }
            if ui.button("Stop server").clicked() {
                self.set_output("server stop", self.server_api("stop", false));
            }
            if ui.button("Disable server").clicked() {
                self.set_output("server disable", self.server_api("stop", true));
            }
            if ui.button("Refresh server").clicked() {
                self.set_output("server refresh", self.server_api("refresh", false));
            }
            if ui.button("Restart hub").clicked() {
                self.set_output("api restart", http_json("POST", format!("{}/api/restart", trim_url(&self.api_base_url)), None));
            }
        });
    }

    fn server_api(&self, endpoint: &str, disable: bool) -> String {
        let suffix = if endpoint == "stop" {
            format!("/api/servers/stop?disable={disable}")
        } else {
            format!("/api/servers/{endpoint}")
        };
        http_json(
            "POST",
            format!("{}{}", trim_url(&self.api_base_url), suffix),
            Some(serde_json::json!({ "server_name": self.api_server_name })),
        )
    }

    fn cmd_runner(&mut self, ui: &mut egui::Ui) {
        ui.heading("Command Runner");
        ui.label("Kill port");
        ui.text_edit_singleline(&mut self.kill_port);
        if ui.button("Run npx kill-port").clicked() {
            let args = vec!["-y".to_owned(), "kill-port".to_owned(), self.kill_port.clone()];
            self.set_output("kill-port", spawn_detached("npx", &args));
        }
        ui.separator();
        ui.label("MCP proxy port");
        ui.text_edit_singleline(&mut self.proxy_port);
        if ui.button("Start MCP proxy").clicked() {
            self.start_mcp_proxy();
        }
        ui.separator();
        ui.label("Serena port");
        ui.text_edit_singleline(&mut self.serena_port);
        if ui.button("Start Serena").clicked() {
            self.start_serena();
        }
    }

    fn cli(&mut self, ui: &mut egui::Ui) {
        ui.heading("CLI");
        ui.horizontal_wrapped(|ui| {
            if ui.button("Start Serena").clicked() {
                self.start_serena();
            }
            if ui.button("Start MCP proxy").clicked() {
                self.start_mcp_proxy();
            }
        });
        ui.label("Native Rust launchers replace Electron/Tauri IPC commands.");
    }

    fn start_mcp_proxy(&mut self) {
        let args = vec![
            "--port".to_owned(),
            self.proxy_port.clone(),
            "node".to_owned(),
            r"C:\Users\paul\Documents\.projects\mcp-server\DesktopCommanderMCP\dist\index.js".to_owned(),
        ];
        self.set_output("mcp-proxy", spawn_detached("mcp-proxy", &args));
    }

    fn start_serena(&mut self) {
        let args = vec![
            "start-mcp-server".to_owned(),
            "--transport".to_owned(),
            "streamable-http".to_owned(),
            "--host".to_owned(),
            "0.0.0.0".to_owned(),
            "--port".to_owned(),
            self.serena_port.clone(),
            "--mode".to_owned(),
            "no-onboarding".to_owned(),
            "--mode".to_owned(),
            "query-projects".to_owned(),
            "--project-from-cwd".to_owned(),
            "--context".to_owned(),
            "codex".to_owned(),
        ];
        self.set_output("serena", spawn_detached("serena", &args));
    }

    fn placeholder(&mut self, ui: &mut egui::Ui, title: &str) {
        ui.heading(format!("{title} // Offline"));
        ui.label("Placeholder kept while the app is moved from WebView/Tauri to native egui.");
    }

    fn output(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Output");
        ui.add(
            egui::TextEdit::multiline(&mut self.output)
                .desired_rows(14)
                .font(egui::TextStyle::Monospace),
        );
    }
}

impl eframe::App for Cc3App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("CC3 Native").strong());
                ui.separator();
                ui.label("Rust + egui / eframe");
            });
        });

        egui::SidePanel::left("tabs").resizable(false).min_width(180.0).show(ctx, |ui| self.left_tabs(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.tab {
                    Tab::Projects => self.projects(ui),
                    Tab::McpHub => self.mcp_hub(ui),
                    Tab::CmdRunner => self.cmd_runner(ui),
                    Tab::Cli => self.cli(ui),
                    Tab::Scrcpy => self.scrcpy(ui),
                    Tab::Network => self.placeholder(ui, "Network"),
                    Tab::Security => self.placeholder(ui, "Security"),
                    Tab::Settings => self.placeholder(ui, "Settings"),
                }
                self.output(ui);
            });
        });
    }
}

fn home_dir() -> PathBuf {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn list_matching(dir: PathBuf, predicate: impl Fn(&str) -> bool) -> Vec<String> {
    fs::read_dir(dir)
        .map(|entries| entries.flatten().filter_map(|entry| entry.file_name().into_string().ok()).filter(|name| predicate(name)).collect())
        .unwrap_or_default()
}

fn empty_or_join(values: &[String]) -> String {
    if values.is_empty() { "none".to_owned() } else { values.join("\n") }
}

fn get_devices() -> Vec<Device> {
    let Ok(out) = Command::new("adb").arg("devices").output() else { return vec![] };
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .skip(1)
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() { return None; }
            let mut parts = line.split_whitespace();
            Some(Device { id: parts.next()?.to_owned(), state: parts.next().unwrap_or("unknown").to_owned() })
        })
        .collect()
}

fn format_devices(devices: &[Device]) -> String {
    if devices.is_empty() {
        "No devices found.".to_owned()
    } else {
        devices.iter().map(|d| format!("{} [{}]", d.id, d.state)).collect::<Vec<_>>().join("\n")
    }
}

fn split_args(input: &str) -> Vec<String> {
    input.split_whitespace().map(str::trim).filter(|s| !s.is_empty()).map(ToOwned::to_owned).collect()
}

fn spawn_detached(program: &str, args: &[String]) -> String {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const DETACHED_PROCESS: u32 = 0x00000008;
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        let line = std::iter::once(program.to_owned()).chain(args.iter().cloned()).map(|part| shell_escape(&part)).collect::<Vec<_>>().join(" ");
        return match Command::new("cmd")
            .arg("/C")
            .arg(format!("start \"\" {line}"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP)
            .spawn()
        {
            Ok(_) => format!("Launched: {line}"),
            Err(err) => format!("ERROR: {err}"),
        };
    }

    #[cfg(not(windows))]
    {
        match Command::new(program).args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            Ok(_) => format!("Launched: {} {}", program, args.join(" ")),
            Err(err) => format!("ERROR: {err}"),
        }
    }
}

fn run_command(program: &str, args: &[String], cwd: Option<&str>) -> String {
    let mut command = Command::new(program);
    command.args(args);
    if let Some(cwd) = cwd.filter(|value| !value.trim().is_empty()) {
        command.current_dir(cwd);
    }
    command_output(command)
}

fn exec_shell(cmd: &str, cwd: &str) -> String {
    let mut command = shell_command(cmd);
    if !cwd.trim().is_empty() {
        command.current_dir(cwd);
    }
    command_output(command)
}

fn command_output(mut command: Command) -> String {
    match command.output() {
        Ok(out) => format!(
            "success: {}\n\nstdout:\n{}\n\nstderr:\n{}",
            out.status.success(),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        ),
        Err(err) => format!("ERROR: {err}"),
    }
}

fn shell_command(cmd: &str) -> Command {
    #[cfg(windows)]
    {
        let mut command = Command::new("cmd");
        command.args(["/C", cmd]);
        command
    }
    #[cfg(not(windows))]
    {
        let mut command = Command::new("sh");
        command.args(["-c", cmd]);
        command
    }
}

fn adb_screenshot(device_id: &str) -> String {
    let pictures = home_dir().join("Pictures");
    let _ = fs::create_dir_all(&pictures);
    let picture_path = pictures.join(format!("screenshot_{}.png", timestamp_millis()));
    let command = if device_id.trim().is_empty() {
        format!("adb exec-out screencap -p > {}", shell_escape_path(&picture_path))
    } else {
        format!("adb -s {} exec-out screencap -p > {}", shell_escape(device_id), shell_escape_path(&picture_path))
    };
    format!("{}\n\nSaved path: {}", exec_shell(&command, ""), picture_path.display())
}

fn http_json(method: &str, url: String, body: Option<Value>) -> String {
    let result = match method {
        "POST" => {
            let request = ureq::post(&url).set("Content-Type", "application/json");
            if let Some(body) = body { request.send_json(body) } else { request.call() }
        }
        _ => ureq::get(&url).call(),
    };
    match result {
        Ok(response) => {
            let status = response.status();
            match response.into_string() {
                Ok(text) => format!("HTTP {status}\n{}", pretty_json(&text)),
                Err(err) => format!("HTTP {status}\nERROR reading body: {err}"),
            }
        }
        Err(err) => format!("ERROR: {err}"),
    }
}

fn pretty_json(text: &str) -> String {
    serde_json::from_str::<Value>(text).ok().and_then(|value| serde_json::to_string_pretty(&value).ok()).unwrap_or_else(|| text.to_owned())
}

fn normalize_url(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") { trimmed.to_owned() } else { format!("https://{trimmed}") }
}

fn trim_url(value: &str) -> String {
    value.trim_end_matches('/').to_owned()
}

fn shell_escape_path(path: &Path) -> String {
    shell_escape(&path.display().to_string())
}

fn shell_escape(value: &str) -> String {
    if value.chars().all(|c| c.is_ascii_alphanumeric() || "-_.:/\\".contains(c)) {
        value.to_owned()
    } else {
        format!("\"{}\"", value.replace('"', "\\\""))
    }
}

fn timestamp_millis() -> u128 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis()).unwrap_or_default()
}
