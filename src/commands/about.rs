use colored::*;

pub fn run() -> String {
    let mut output = String::new();
    output.push_str(&format!("\n{}\n", "╔══════════════════════════════════╗".yellow()));
    output.push_str(&format!("║       {}      ║\n", "GitHub Clone CLI v0.2".bold()));
    output.push_str(&format!("{}\n", "╚══════════════════════════════════╝".yellow()));
    output.push_str(&format!("Built with {} by a Rust learner.\n", "Rust".red().bold()));
    output.push_str("This tool interacts with your React/Vite/Tailwind project.\n");
    output.push_str("It features a config system, git integration, and stats analysis.\n");
    output
}