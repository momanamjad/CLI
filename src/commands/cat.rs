use crate::utils::printer;
use std::fs;

pub fn run(path: &str) {
    printer::print_header(&format!("File: {}", path));

    match fs::read_to_string(path) {
        Ok(content) => {
            // print with line numbers
            for (index, line) in content.lines().enumerate() {
                println!("{:>4} │ {}", index + 1, line);
            }
            println!();
        }
        Err(e) => {
            printer::print_error(&format!("Cannot read '{}': {}", path, e));
        }
    }
}