use walkdir::WalkDir;
use colored::*;

pub fn run(path: &str, target: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("{} searching for '{}' in {}...\n", "→".blue().bold(), target.yellow(), path.cyan()));

    let mut found = false;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path_str = entry.path().to_string_lossy();
        if path_str.contains("node_modules") || path_str.contains(".git") || path_str.contains("target") || path_str.contains("dist") || path_str.contains("build") {
            continue;
        }

        if entry.file_name().to_string_lossy().contains(target) {
            let path_str = entry.path().display().to_string();
            if entry.file_type().is_dir() {
                output.push_str(&format!("{} {}\n", "[DIR]".blue().bold(), path_str.blue()));
            } else {
                output.push_str(&format!("{} {}\n", "[FILE]".green().bold(), path_str.white()));
            }
            found = true;
        }
    }

    if !found {
        output.push_str(&format!("{}\n", "No matches found.".red()));
    }
    output
}