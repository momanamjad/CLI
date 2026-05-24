use axum::{
    extract::{Query, WebSocketUpgrade, ws::{Message, WebSocket}},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use futures_util::stream::StreamExt;
use tower_http::cors::CorsLayer;
use crate::config::Config;
use crate::commands::{stats, deps, ls, search, git};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use futures_util::SinkExt;

// --- Response Structs ---

#[derive(Serialize)]
pub struct GitResponse {
    pub output: String,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Deserialize)]
pub struct LsQuery {
    pub path: Option<String>,
}

#[derive(Deserialize)]
pub struct LogQuery {
    pub n: Option<String>,
}

#[derive(Deserialize)]
pub struct PathQuery {
    pub path: Option<String>,
}

// --- System Metrics ---

#[derive(Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub disk_used_gb: f32,
    pub disk_total_gb: f32,
    pub uptime_seconds: u64,
}

fn read_ram() -> (u64, u64) {
    let content = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0u64;
    let mut available = 0u64;
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        }
    }
    let used_kb = total.saturating_sub(available);
    (used_kb / 1024, total / 1024)
}

fn read_cpu_stat() -> (u64, u64) {
    let content = std::fs::read_to_string("/proc/stat").unwrap_or_default();
    let first_line = content.lines().next().unwrap_or("");
    let nums: Vec<u64> = first_line.split_whitespace()
        .skip(1)
        .filter_map(|v| v.parse().ok())
        .collect();
    if nums.len() < 4 {
        return (0, 1);
    }
    let idle = nums[3];
    let total: u64 = nums.iter().sum();
    (idle, total)
}

fn read_uptime() -> u64 {
    let content = std::fs::read_to_string("/proc/uptime").unwrap_or_default();
    content.split_whitespace()
        .next()
        .and_then(|v| v.parse::<f64>().ok())
        .map(|v| v as u64)
        .unwrap_or(0)
}

fn read_disk() -> (f32, f32) {
    let output = std::process::Command::new("df")
        .args(&["/workspace", "--output=used,size", "-BM"])
        .output()
        .ok();
    if let Some(out) = output {
        let text = String::from_utf8_lossy(&out.stdout).to_string();
        let data_line = text.lines().nth(1).unwrap_or("");
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let used = parts[0].trim_end_matches('M').parse::<f32>().unwrap_or(0.0) / 1024.0;
            let total = parts[1].trim_end_matches('M').parse::<f32>().unwrap_or(0.0) / 1024.0;
            return (used, total);
        }
    }
    (0.0, 0.0)
}

async fn handle_metrics() -> Json<SystemMetrics> {
    let (idle1, total1) = read_cpu_stat();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let (idle2, total2) = read_cpu_stat();

    let idle_delta = idle2.saturating_sub(idle1) as f32;
    let total_delta = total2.saturating_sub(total1) as f32;
    let cpu_usage = if total_delta > 0.0 {
        (1.0 - idle_delta / total_delta) * 100.0
    } else {
        0.0
    };

    let (ram_used_mb, ram_total_mb) = read_ram();
    let uptime_seconds = read_uptime();
    let (disk_used_gb, disk_total_gb) = read_disk();

    Json(SystemMetrics {
        cpu_usage,
        ram_used_mb,
        ram_total_mb,
        disk_used_gb,
        disk_total_gb,
        uptime_seconds,
    })
}

// --- File Read Endpoint ---

async fn handle_read_file(Query(query): Query<PathQuery>) -> Json<serde_json::Value> {
    let path = query.path.unwrap_or_default();
    let path_obj = std::path::Path::new(&path);

    // Check if path exists
    if !path_obj.exists() {
        return Json(serde_json::json!({"error": format!("Path does not exist: {}", path)}));
    }

    // Check if path is a directory (not a file)
    if path_obj.is_dir() {
        return Json(serde_json::json!({"error": format!("Path is a directory, not a file: {}", path)}));
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => Json(serde_json::json!({"content": content})),
        Err(e) => Json(serde_json::json!({"error": e.to_string()})),
    }
}

// --- File Write Request ---

#[derive(Deserialize)]
struct WriteFileRequest {
    path: String,
    content: String,
}

async fn handle_write_file(Json(req): Json<WriteFileRequest>) -> Json<GitResponse> {
    match std::fs::write(&req.path, &req.content) {
        Ok(_) => Json(GitResponse { output: "File saved successfully".to_string() }),
        Err(e) => Json(GitResponse { output: format!("Error: {}", e) }),
    }
}

// --- Ports Endpoint ---

fn extract_port(line: &str) -> Option<serde_json::Value> {
    // Parse lines like: LISTEN 0  128  0.0.0.0:3001  ...  users:(("github-cli",pid=1,fd=7))
    let parts: Vec<&str> = line.split_whitespace().collect();
    for part in &parts {
        if let Some(colon_pos) = part.rfind(':') {
            let port_str = &part[colon_pos + 1..];
            if let Ok(port_num) = port_str.parse::<u16>() {
                // Try to extract process name from users:((...)) section
                let process = if let Some(users_start) = line.find("users:(") {
                    let users_section = &line[users_start..];
                    if let Some(name_start) = users_section.find('"') {
                        let after_quote = &users_section[name_start + 1..];
                        if let Some(name_end) = after_quote.find('"') {
                            after_quote[..name_end].to_string()
                        } else {
                            "unknown".to_string()
                        }
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    parts.last().unwrap_or(&"unknown").to_string()
                };
                return Some(serde_json::json!({
                    "port": port_num,
                    "process": process,
                    "status": "LISTEN"
                }));
            }
        }
    }
    None
}

async fn handle_ports() -> Json<serde_json::Value> {
    // Try ss first (needs iproute2), fall back to /proc/net/tcp
    let ss_output = std::process::Command::new("sh")
        .args(["-c", "ss -tlnp 2>/dev/null | grep LISTEN"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let mut ports = Vec::new();

    if !ss_output.trim().is_empty() {
        // Parse ss output
        for line in ss_output.lines() {
            if line.contains("LISTEN") {
                if let Some(port) = extract_port(line) {
                    ports.push(port);
                }
            }
        }
    } else {
        // Fallback: parse /proc/net/tcp (hex port values)
        if let Ok(content) = std::fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                let cols: Vec<&str> = line.split_whitespace().collect();
                if cols.len() >= 4 && cols[3] == "0A" {
                    // state 0A = LISTEN
                    // local_address is "hex_ip:hex_port"
                    if let Some(port_hex) = cols[1].split(':').nth(1) {
                        if let Ok(port_num) = u16::from_str_radix(port_hex, 16) {
                            if port_num > 0 {
                                ports.push(serde_json::json!({
                                    "port": port_num,
                                    "process": "unknown",
                                    "status": "LISTEN"
                                }));
                            }
                        }
                    }
                }
            }
        }
        // Also try /proc/net/tcp6
        if let Ok(content) = std::fs::read_to_string("/proc/net/tcp6") {
            for line in content.lines().skip(1) {
                let cols: Vec<&str> = line.split_whitespace().collect();
                if cols.len() >= 4 && cols[3] == "0A" {
                    if let Some(port_hex) = cols[1].split(':').nth(1) {
                        if let Ok(port_num) = u16::from_str_radix(port_hex, 16) {
                            if port_num > 0 {
                                // avoid duplicates already in tcp list
                                if !ports.iter().any(|p| p["port"] == port_num) {
                                    ports.push(serde_json::json!({
                                        "port": port_num,
                                        "process": "unknown",
                                        "status": "LISTEN"
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Json(serde_json::json!({ "ports": ports }))
}

// --- Server Entry Point ---

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(|| async { "GitHub CLI Backend is running!" }))
        .route("/stats", get(handle_stats))
        .route("/deps", get(handle_deps))
        .route("/ls", get(handle_ls))
        .route("/search", post(handle_search))
        .route("/git/status", get(handle_git_status))
        .route("/git/log", get(handle_git_log))
        .route("/git/branch", get(handle_git_branch))
        .route("/ws", get(handle_ws))
        .route("/metrics", get(handle_metrics))
        .route("/file", get(handle_read_file).post(handle_write_file))
        .route("/ports", get(handle_ports))
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr_str = format!("0.0.0.0:{}", port);
    println!("Backend server listening on http://{}", addr_str);
    
    let listener = tokio::net::TcpListener::bind(&addr_str).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- Handlers ---

async fn handle_stats(Query(query): Query<PathQuery>) -> Json<stats::ProjectStats> {
    let path = query.path.unwrap_or_else(|| {
        let config = Config::load();
        config.project_path
    });
    Json(stats::get_data(&path))
}

async fn handle_deps() -> Json<Option<deps::DepsData>> {
    let config = Config::load();
    Json(deps::get_data(&config.project_path))
}

async fn handle_ls(Query(query): Query<LsQuery>) -> String {
    let config = Config::load();
    let sub_path = query.path.unwrap_or_else(|| ".".to_string());
    let full_path = Path::new(&config.project_path).join(sub_path);
    ls::run(&full_path.to_string_lossy())
}

async fn handle_search(Json(req): Json<SearchRequest>) -> Json<Vec<search::SearchMatch>> {
    let config = Config::load();
    Json(search::get_data(&config.project_path, &req.query))
}

async fn handle_git_status() -> Json<GitResponse> {
    let config = Config::load();
    Json(GitResponse { 
        output: git::status(&config.project_path) 
    })
}

async fn handle_git_log(Query(query): Query<LogQuery>) -> Json<GitResponse> {
    let config = Config::load();
    let n = query.n.unwrap_or_else(|| "5".to_string());
    Json(GitResponse { 
        output: git::log(&config.project_path, &n) 
    })
}

async fn handle_git_branch() -> Json<GitResponse> {
    let config = Config::load();
    Json(GitResponse { 
        output: git::branch(&config.project_path) 
    })
}

async fn handle_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

#[derive(Deserialize)]
struct ResizeMessage {
    cols: Option<u16>,
    rows: Option<u16>,
}

async fn handle_socket(socket: WebSocket) {
    let pty_system = NativePtySystem::default();
    let pair = pty_system.openpty(PtySize {
        rows: 50,
        cols: 220,
        pixel_width: 0,
        pixel_height: 0,
    }).unwrap();

    let shell = if cfg!(target_os = "windows") {
        "powershell.exe"
    } else {
        "bash"
    };
    let mut cmd = CommandBuilder::new(shell);
    if shell == "powershell.exe" {
        cmd.args(&["-NoLogo"]);
    }
    
    let project_path = std::env::var("PROJECT_PATH").unwrap_or_else(|_| ".".to_string());
    cmd.cwd(project_path);

    let mut child = pair.slave.spawn_command(cmd).unwrap();
    
    let mut reader = pair.master.try_clone_reader().unwrap();
    let mut writer = pair.master.take_writer().unwrap();
    let master = pair.master;

    let (mut ws_sender, mut ws_receiver) = socket.split();
    // Channel now carries axum ws Message so we can send both Binary and Text
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);
    let tx_ping = tx.clone();

    // Task 1: PTY stdout -> Channel -> WebSocket
    tokio::task::spawn_blocking(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            if tx.blocking_send(Message::Binary(buf[..n].to_vec())).is_err() {
                break;
            }
        }
    });

    let ws_send_task = tokio::task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Task 2: WebSocket -> PTY stdin / Resize / Ping-Pong
    let ws_recv_task = tokio::task::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Binary(data) => {
                    let _ = writer.write_all(&data);
                    let _ = writer.flush();
                }
                Message::Text(text) => {
                    // Try to parse as a control message first
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                        let msg_type = parsed.get("type").and_then(|v| v.as_str()).unwrap_or("");

                        if msg_type == "ping" {
                            // Immediately send pong back over WebSocket
                            let pong = serde_json::json!({"type": "pong"}).to_string();
                            let _ = tx_ping.send(Message::Text(pong)).await;
                            continue;
                        }

                        if msg_type == "resize" {
                            if let Ok(resize) = serde_json::from_value::<ResizeMessage>(parsed) {
                                if let (Some(cols), Some(rows)) = (resize.cols, resize.rows) {
                                    let _ = master.resize(PtySize {
                                        rows,
                                        cols,
                                        pixel_width: 0,
                                        pixel_height: 0,
                                    });
                                }
                            }
                            continue;
                        }
                    }

                    // Not a control message — write raw text to PTY
                    let _ = writer.write_all(text.as_bytes());
                    let _ = writer.flush();
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = ws_send_task => {},
        _ = ws_recv_task => {},
    }

    // Kill the process on disconnect
    let _ = child.kill();
}
