use colored::*;

pub fn run() -> String {
    let mut output = String::new();
    output.push_str(&format!("\n{}\n", "=== Available Commands ===".green().bold()));
    output.push_str(&format!("{:<25} {}\n", "help".cyan(), "Show this help message"));
    output.push_str(&format!("{:<25} {}\n", "about".cyan(), "Learn more about this project"));
    output.push_str(&format!("{:<25} {}\n", "pwd".cyan(), "Show current working directory"));
    output.push_str(&format!("{:<25} {}\n", "ls <path>".cyan(), "List files (defaults to .)"));
    output.push_str(&format!("{:<25} {}\n", "cat <file>".cyan(), "Read file contents"));
    output.push_str(&format!("{:<25} {}\n", "find <p> <n>".cyan(), "Search for file by name recursively"));
    output.push_str(&format!("{:<25} {}\n", "open <path>".cyan(), "Open file/folder in VS Code"));
    output.push_str(&format!("{:<25} {}\n", "serve".cyan(), "Start backend server on port 3001"));
    output.push_str(&format!("{:<25} {}\n", "stats".cyan(), "Show project stats (LOC, breakdown)"));
    output.push_str(&format!("{:<25} {}\n", "deps".cyan(), "Show package dependencies in a table"));
    output.push_str(&format!("{:<25} {}\n", "search <key>".cyan(), "Search keyword inside file contents"));
    output.push_str(&format!("{:<25} {}\n", "recent".cyan(), "Show 10 most recently modified files"));
    output.push_str(&format!("{:<25} {}\n", "login <email> <pwd>".cyan(), "Authenticate local session"));
    output.push_str(&format!("{:<25} {}\n", "remote-create <n> [d]".cyan(), "Create remote repository on web clone"));
    output.push_str(&format!("{:<25} {}\n", "remote-link <repo_id>".cyan(), "Link local folder to existing web repo"));
    output.push_str(&format!("{:<25} {}\n", "remote-push".cyan(), "Push all folder files to linked repo"));
    output.push_str(&format!("{:<25} {}\n", "remote-pull".cyan(), "Pull all remote repo files to local folder"));
    output.push_str(&format!("{:<25} {}\n", "git-status".cyan(), "Show git status"));
    output.push_str(&format!("{:<25} {}\n", "git-log <n>".cyan(), "Show last N git commits (default 5)"));
    output.push_str(&format!("{:<25} {}\n", "git-branch".cyan(), "List all git branches"));
    output.push_str(&format!("{:<25} {}\n", "secret-set <k> <v>".cyan(), "Set or update repository secret"));
    output.push_str(&format!("{:<25} {}\n", "secret-list".cyan(), "List all secret names in repository"));
    output.push_str(&format!("{:<25} {}\n", "secret-delete <k>".cyan(), "Delete a repository secret"));
    output.push_str(&format!("{:<25} {}\n", "exit".cyan(), "Quit the CLI"));
    output
}