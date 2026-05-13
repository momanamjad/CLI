use std::process::Command;
use colored::*;

pub fn run(path: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("{} opening in VS Code...\n", "→".blue().bold()));
    
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "code", path])
            .status()
    } else {
        Command::new("code")
            .arg(path)
            .status()
    };

    match status {
        Ok(s) if s.success() => output.push_str(&format!("{}", "Success!".green().bold())),
        Ok(_) => output.push_str(&format!("{}", "VS Code command failed. Is 'code' in your PATH?".red().bold())),
        Err(e) => output.push_str(&format!("{} {}", "Error launching VS Code:".red().bold(), e)),
    }
    output
}
