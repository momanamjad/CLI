use crate::utils::printer;
use std::fs;

pub fn run(path: &str) {
    printer::print_header(&format!("Contents of: {}", path));

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut dirs: Vec<String> = Vec::new();
            let mut files: Vec<String> = Vec::new();

            for entry in entries {
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                let is_dir = entry.file_type().unwrap().is_dir();

                if is_dir {
                    dirs.push(format!("📁 {}/", name));
                } else {
                    files.push(format!("📄 {}", name));
                }
            }

            dirs.sort();
            files.sort();

            for d in &dirs  { printer::print_info(d); }
            for f in &files { printer::print_info(f); }

            println!();
            printer::print_success(&format!(
                "{} folders, {} files",
                dirs.len(),
                files.len()
            ));
            println!();
        }
        Err(_) => {
            printer::print_error(&format!("Cannot read path: '{}'", path));
        }
    }
}