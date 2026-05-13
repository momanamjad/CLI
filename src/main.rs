mod cli;
mod commands;
mod utils;
mod config;
mod server;

#[tokio::main]
async fn main() {
    // Start HTTP server in background
    tokio::spawn(async {
        server::start_server().await;
    });

    // Small delay so server prints its address first
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Start CLI loop
    cli::run().await;
}