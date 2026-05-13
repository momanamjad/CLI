use colored::*;

pub fn run() -> String {
    let mut output = String::new();
    output.push_str(&format!("\n{}\n", "=== Available Commands ===".green().bold()));
    output.push_str(&format!("{:<20} {}\n", "help".cyan(), "Show this help message"));
    output.push_str(&format!("{:<20} {}\n", "about".cyan(), "Learn more about this project"));
    output.push_str(&format!("{:<20} {}\n", "pwd".cyan(), "Show current working directory"));
    output.push_str(&format!("{:<20} {}\n", "ls <path>".cyan(), "List files (defaults to .)"));
    output.push_str(&format!("{:<20} {}\n", "cat <file>".cyan(), "Read file contents"));
    output.push_str(&format!("{:<20} {}\n", "find <p> <n>".cyan(), "Search for file by name recursively"));
    output.push_str(&format!("{:<20} {}\n", "open <path>".cyan(), "Open file/folder in VS Code"));
    output.push_str(&format!("{:<20} {}\n", "serve".cyan(), "Start backend server on port 3001"));
    output.push_str(&format!("{:<20} {}\n", "stats".cyan(), "Show project stats (LOC, breakdown)"));
    output.push_str(&format!("{:<20} {}\n", "deps".cyan(), "Show package dependencies in a table"));
    output.push_str(&format!("{:<20} {}\n", "search <key>".cyan(), "Search keyword inside file contents"));
    output.push_str(&format!("{:<20} {}\n", "recent".cyan(), "Show 10 most recently modified files"));
    output.push_str(&format!("{:<20} {}\n", "git-status".cyan(), "Show git status"));
    output.push_str(&format!("{:<20} {}\n", "git-log <n>".cyan(), "Show last N git commits (default 5)"));
    output.push_str(&format!("{:<20} {}\n", "git-branch".cyan(), "List all git branches"));
    output.push_str(&format!("{:<20} {}\n", "exit".cyan(), "Quit the CLI"));
    output
}