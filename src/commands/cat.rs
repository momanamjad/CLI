use std::fs;
use colored::*;

pub fn run(filepath: &str) -> String {
    match fs::read_to_string(filepath) {
        Ok(content) => {
            let mut output = String::new();
            output.push_str(&format!("\n{} --- Start of {} ---\n", ">>>".green().bold(), filepath.yellow()));
            output.push_str(&content);
            output.push_str(&format!("\n{} --- End of {} ---\n", "<<<".green().bold(), filepath.yellow()));
            output
        }
        Err(e) => format!("{} could not read {}: {}", "Error:".red().bold(), filepath.yellow(), e),
    }
}