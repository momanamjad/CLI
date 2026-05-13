use std::io;
use std::io::Write;
use crate::commands::{help, about, ls, cat, find};

pub fn run() {
    println!("╔══════════════════════════════════╗");
    println!("║       GitHub Clone CLI v0.1      ║");
    println!("║  Your project, from the terminal ║");
    println!("╚══════════════════════════════════╝");
    println!("Type 'help' to see available commands.\n");

    loop {
        print!("github-cli> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // split input into parts: "ls src" → ["ls", "src"]
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue; // skip empty input, restart loop
        }

        match parts[0] {
            "help"  => help::run(),
            "about" => about::run(),
            "cat" => {
    if parts.len() < 2 {
        println!("Usage: cat <filepath>\n");
    } else {
        cat::run(parts[1]);
    }
}
            "ls" => {
                let path = if parts.len() > 1 { parts[1] } else { "." };
                ls::run(path);
            }
            "exit" => {
                println!("Goodbye!");
                break;
            }
            "find" => {
    if parts.len() < 3 {
        println!("Usage: find <path> <filename>\n");
    } else {
        find::run(parts[1], parts[2]);
    }
}
            _ => {
                println!("'{}' is not a recognized command. Type 'help'.\n", input);
            }
        }
    }
}