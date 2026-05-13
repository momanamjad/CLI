mod cli;
mod commands;
mod utils;
mod config;
mod server;

#[tokio::main]
async fn main() {
    // If SERVER_ONLY env var is set, just run the server (for Railway/cloud)
    if std::env::var("SERVER_ONLY").is_ok() {
        println!("Starting github-cli server in server-only mode...");
        server::start_server().await;
        return;
    }

    // Otherwise run both (local dev)
    tokio::spawn(async {
        server::start_server().await;
    });

    // Small delay so server prints its address first
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Start CLI loop
    cli::run().await;
}