use std::fs;
use colored::*;

pub fn run(path: &str) -> String {
    let mut output = String::new();
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(err) => {
            return format!("{} {}: {}", "Error reading".red(), path.yellow(), err);
        }
    };

    output.push_str(&format!("\n{} listing contents of {}:\n", "→".blue().bold(), path.yellow()));
    
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let metadata = entry.metadata().unwrap();
            
            if metadata.is_dir() {
                output.push_str(&format!("{}/\n", file_name.blue().bold()));
            } else {
                output.push_str(&format!("{}\n", file_name.white()));
            }
        }
    }
    output
}