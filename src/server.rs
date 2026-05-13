use axum::{
    extract::{Query, WebSocketUpgrade, ws::{Message, WebSocket}},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::path::Path;
use futures_util::stream::StreamExt;
use tower_http::cors::CorsLayer;
use crate::config::Config;
use crate::commands::{stats, deps, ls, search, git, help, about, pwd, cat, find, recent};

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

// --- Server Entry Point ---

pub async fn start_server() {
    let app = Router::new()
        .route("/stats", get(handle_stats))
        .route("/deps", get(handle_deps))
        .route("/ls", get(handle_ls))
        .route("/search", post(handle_search))
        .route("/git/status", get(handle_git_status))
        .route("/git/log", get(handle_git_log))
        .route("/git/branch", get(handle_git_branch))
        .route("/ws", get(handle_ws))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Backend server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
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

async fn handle_socket(mut socket: WebSocket) {
    let config = Config::load();
    
    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            let parts: Vec<&str> = text.split_whitespace().collect();
            if parts.is_empty() { continue; }

            let output = match parts[0] {
                "help" => help::run(),
                "about" => about::run(),
                "pwd" => pwd::run(),
                "ls" => {
                    let path = if parts.len() > 1 { parts[1] } else { "." };
                    let full_path = Path::new(&config.project_path).join(path);
                    ls::run(&full_path.to_string_lossy())
                }
                "cat" => {
                    if parts.len() < 2 {
                        "Usage: cat <filepath>".to_string()
                    } else {
                        let full_path = Path::new(&config.project_path).join(parts[1]);
                        cat::run(&full_path.to_string_lossy())
                    }
                }
                "find" => {
                    if parts.len() < 3 {
                        "Usage: find <path> <filename>".to_string()
                    } else {
                        let full_path = Path::new(&config.project_path).join(parts[1]);
                        find::run(&full_path.to_string_lossy(), parts[2])
                    }
                }
                "stats" => stats::run(&config.project_path),
                "deps" => deps::run(&config.project_path),
                "search" => {
                    if parts.len() < 2 {
                        "Usage: search <keyword>".to_string()
                    } else {
                        search::run(&config.project_path, parts[1])
                    }
                }
                "recent" => recent::run(&config.project_path),
                "git-status" | "git status" => git::status(&config.project_path),
                "git-log" | "git log" => {
                    let n = if parts.len() > 1 { parts[1] } else { "5" };
                    git::log(&config.project_path, n)
                }
                "git-branch" | "git branch" => git::branch(&config.project_path),
                _ => format!("'{}' is not a recognized command.", parts[0]),
            };

            let _ = socket.send(Message::Text(output)).await;
        }
    }
}
