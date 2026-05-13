use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::*;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SearchMatch {
    pub file: String,
    pub line: usize,
    pub content: String,
}

pub fn get_data(project_path: &str, keyword: &str) -> Vec<SearchMatch> {
    let mut matches = Vec::new();

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) 
    {
        let path = entry.path();
        let path_str = path.to_string_lossy();
        if path_str.contains("node_modules") || path_str.contains(".git") || path_str.contains("target") || path_str.contains("dist") || path_str.contains("build") || path_str.contains(".png") || path_str.contains(".jpg") {
            continue;
        }

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                if let Ok(line_content) = line {
                    if line_content.contains(keyword) {
                        matches.push(SearchMatch {
                            file: path.display().to_string(),
                            line: index + 1,
                            content: line_content.trim().to_string(),
                        });
                    }
                }
            }
        }
    }
    matches
}

pub fn run(project_path: &str, keyword: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("{} searching for '{}' in {}...\n", "→".blue().bold(), keyword.yellow(), project_path.cyan()));

    let matches = get_data(project_path, keyword);

    if matches.is_empty() {
        output.push_str(&format!("{}\n", "No matches found.".red()));
    } else {
        for m in &matches {
            output.push_str(&format!("{}:{} → {}\n", 
                m.file.cyan(), 
                m.line.to_string().yellow(), 
                m.content.green()
            ));
        }
        output.push_str(&format!("\n{} matches found.\n", matches.len().to_string().bold()));
    }
    output
}
