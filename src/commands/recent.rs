use walkdir::WalkDir;
use std::time::SystemTime;
use colored::*;
use chrono::{DateTime, Local};

pub fn run(project_path: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("{} fetching recently modified files...\n", "→".blue().bold()));

    let mut files: Vec<(String, SystemTime)> = Vec::new();

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) 
    {
        let path = entry.path();
        let path_str = path.to_string_lossy();
        if path_str.contains("node_modules") || path_str.contains(".git") || path_str.contains("target") || path_str.contains("dist") || path_str.contains("build") {
            continue;
        }

        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                files.push((path.display().to_string(), modified));
            }
        }
    }

    files.sort_by(|a, b| b.1.cmp(&a.1));

    output.push_str(&format!("\n{}\n", "=== Recently Modified Files ===".green().bold()));
    for (name, time) in files.iter().take(10) {
        let datetime: DateTime<Local> = (*time).into();
        output.push_str(&format!("{:<50} {}\n", name.cyan(), datetime.format("%Y-%m-%d %H:%M:%S").to_string().yellow()));
    }
    output
}
