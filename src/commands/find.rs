use crate::utils::printer;
use std::fs;
use std::path::Path;

pub fn run(root: &str, query: &str) {
    printer::print_header(&format!("Searching '{}' for: {}", root, query));
    
    let mut results: Vec<String> = Vec::new();
    search_dir(root, query, &mut results);

    if results.is_empty() {
        printer::print_error("No matches found.");
    } else {
        for path in &results {
            printer::print_info(path);
        }
        println!();
        printer::print_success(&format!("{} match(es) found", results.len()));
    }
    println!();
}

// recursive function — calls itself for each subfolder
fn search_dir(dir: &str, query: &str, results: &mut Vec<String>) {
    let path = Path::new(dir);

    // skip folders we can't read (permissions, etc.)
    let entries = match fs::read_dir(path) {
        Ok(e)  => e,
        Err(_) => return, // just skip this folder silently
    };

    for entry in entries {
        let entry = match entry {
            Ok(e)  => e,
            Err(_) => continue, // skip bad entries, keep going
        };

        let entry_path = entry.path();
        let name = entry.file_name().into_string().unwrap_or_default();

        // skip node_modules and .git — too many files, not useful
        if name == "node_modules" || name == ".git" || name == "target" {
            continue;
        }

        if entry_path.is_dir() {
            // recursion: search inside this subfolder too
            let sub = entry_path.to_string_lossy().to_string();
            search_dir(&sub, query, results);
        } else {
            // check if filename contains the search query
            if name.to_lowercase().contains(&query.to_lowercase()) {
                results.push(entry_path.to_string_lossy().to_string());
            }
        }
    }
}