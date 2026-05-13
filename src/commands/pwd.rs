use std::env;
use colored::*;

pub fn run() -> String {
    match env::current_dir() {
        Ok(path) => format!("{} {}", "Current directory:".cyan().bold(), path.display().to_string().yellow()),
        Err(e) => format!("{} {}", "Error getting current directory:".red().bold(), e),
    }
}
