use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::*;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ProjectStats {
    pub total_files: usize,
    pub total_lines: usize,
    pub file_types: HashMap<String, usize>,
    pub largest_files: Vec<(String, u64)>,
}

pub fn get_data(project_path: &str) -> ProjectStats {
    let mut total_files = 0;
    let mut total_lines = 0;
    let mut extension_counts: HashMap<String, usize> = HashMap::new();
    let mut largest_files: Vec<(String, u64)> = Vec::new();

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) 
    {
        let path = entry.path();
        let path_str = path.to_string_lossy();
        if path_str.contains("node_modules") || path_str.contains("target") || path_str.contains(".git") || path_str.contains("dist") || path_str.contains("build") {
            continue;
        }

        total_files += 1;
        
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_string();
            *extension_counts.entry(ext_str).or_insert(0) += 1;
        }

        if let Ok(metadata) = entry.metadata() {
            largest_files.push((path.display().to_string(), metadata.len()));
        }

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            total_lines += reader.lines().count();
        }
    }

    largest_files.sort_by(|a, b| b.1.cmp(&a.1));

    ProjectStats {
        total_files,
        total_lines,
        file_types: extension_counts,
        largest_files,
    }
}

pub fn run(project_path: &str) -> String {
    let stats = get_data(project_path);
    let mut output = String::new();

    output.push_str(&format!("{} scanning {}...\n", "→".blue().bold(), project_path.yellow()));
    output.push_str(&format!("\n{}\n", "=== Project Statistics ===".green().bold()));
    output.push_str(&format!("{:<20} {}\n", "Total Files:", stats.total_files.to_string().yellow()));
    output.push_str(&format!("{:<20} {}\n", "Total Lines (LOC):", stats.total_lines.to_string().yellow()));
    
    output.push_str(&format!("\n{}\n", "--- File Breakdown ---".cyan().bold()));
    for (ext, count) in &stats.file_types {
        output.push_str(&format!("{:<20} {}\n", format!(".{}", ext), count.to_string().yellow()));
    }

    output.push_str(&format!("\n{}\n", "--- Largest Files ---".cyan().bold()));
    for (name, size) in stats.largest_files.iter().take(5) {
        let size_kb = *size as f64 / 1024.0;
        output.push_str(&format!("{:<40} {:.2} KB\n", name, size_kb));
    }

    output
}
