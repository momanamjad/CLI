use std::process::Command;
use colored::*;

pub fn status(project_path: &str) -> String {
    run_git_command(project_path, &["status"])
}

pub fn log(project_path: &str, n: &str) -> String {
    run_git_command(project_path, &["log", "-n", n, "--oneline", "--graph", "--decorate"])
}

pub fn branch(project_path: &str) -> String {
    run_git_command(project_path, &["branch", "-a"])
}

fn run_git_command(path: &str, args: &[&str]) -> String {
    let mut result = String::new();
    result.push_str(&format!("{} running git {} in {}...\n\n", "→".blue().bold(), args[0].yellow(), path.cyan()));

    let output = Command::new("git")
        .current_dir(path)
        .args(args)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                result.push_str(&String::from_utf8_lossy(&out.stdout));
            } else {
                result.push_str(&format!("{} {}", "Git error:".red().bold(), String::from_utf8_lossy(&out.stderr)));
            }
        },
        Err(e) => result.push_str(&format!("{} {}", "Failed to execute git:".red().bold(), e)),
    }
    result
}
