use axum::{
    extract::{Query, WebSocketUpgrade, ws::{Message, WebSocket}},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
// use std::net::SocketAddr; // Removed as it is now handled via string binding
use std::path::Path;
use futures_util::stream::StreamExt;
use tower_http::cors::CorsLayer;
use crate::config::Config;
use crate::commands::{stats, deps, ls, search, git};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use futures_util::SinkExt;

// --- Response Structs (reusing from commands where possible) ---

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
        .route("/file", post(handle_write_file))
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr_str = format!("0.0.0.0:{}", port);
    println!("Backend server listening on http://{}", addr_str);
    
    let listener = tokio::net::TcpListener::bind(&addr_str).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- Handlers ---

async fn handle_stats() -> Json<stats::ProjectStats> {
    let config = Config::load();
    Json(stats::get_data(&config.project_path))
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
    #[serde(rename = "type")]
    msg_type: String,
    cols: u16,
    rows: u16,
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
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(100);

    // Task 1: PTY stdout -> Channel -> WebSocket
    tokio::task::spawn_blocking(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            if tx.blocking_send(buf[..n].to_vec()).is_err() {
                break;
            }
        }
    });

    let ws_send_task = tokio::task::spawn(async move {
        while let Some(data) = rx.recv().await {
            if ws_sender.send(Message::Binary(data)).await.is_err() {
                break;
            }
        }
    });

    // Task 2: WebSocket -> PTY stdin / Resize
    let ws_recv_task = tokio::task::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Binary(data) => {
                    let _ = writer.write_all(&data);
                    let _ = writer.flush();
                }
                Message::Text(text) => {
                    // Try to parse as ResizeMessage first
                    if let Ok(resize) = serde_json::from_str::<ResizeMessage>(&text) {
                        if resize.msg_type == "resize" {
                            let _ = master.resize(PtySize {
                                rows: resize.rows,
                                cols: resize.cols,
                                pixel_width: 0,
                                pixel_height: 0,
                            });
                            continue; // Skip writing this to the PTY
                        }
                    }
                    
                    // Not a resize message, write as raw input
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
