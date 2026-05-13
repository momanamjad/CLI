use std::io::{self, Write};
use colored::*;
use crate::commands::{help, about, ls, cat, find, pwd, open, stats, deps, search, recent, git};
use crate::config::Config;
use crate::server;

pub struct CommandHandler {
    config: Config,
}

pub async fn run() {
    let handler = CommandHandler::new();
    handler.run().await;
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
        }
    }

    pub async fn run(&self) {
        println!("{}", "╔══════════════════════════════════╗".bright_green());
        println!("║       {}      ║", "GitHub Clone CLI v0.2".bold().white());
        println!("║  {} ║", "Production-ready Rust Terminal".dimmed());
        println!("{}", "╚══════════════════════════════════╝".bright_green());
        println!("Project path: {}\n", self.config.project_path.yellow());

        loop {
            print!("{}", "github-cli> ".bright_blue().bold());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }
            let input = input.trim();
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "help" => println!("{}", help::run()),
                "serve" => server::start_server().await,
                "about" => println!("{}", about::run()),
                "pwd" => println!("{}", pwd::run()),
                "ls" => {
                    let path = if parts.len() > 1 { parts[1] } else { "." };
                    println!("{}", ls::run(path));
                }
                "cat" => {
                    if parts.len() < 2 {
                        println!("Usage: cat <filepath>");
                    } else {
                        println!("{}", cat::run(parts[1]));
                    }
                }
                "find" => {
                    if parts.len() < 3 {
                        println!("Usage: find <path> <filename>");
                    } else {
                        println!("{}", find::run(parts[1], parts[2]));
                    }
                }
                "open" => {
                    let path = if parts.len() > 1 { parts[1] } else { "." };
                    println!("{}", open::run(path));
                }
                "stats" => println!("{}", stats::run(&self.config.project_path)),
                "deps" => println!("{}", deps::run(&self.config.project_path)),
                "search" => {
                    if parts.len() < 2 {
                        println!("Usage: search <keyword>");
                    } else {
                        println!("{}", search::run(&self.config.project_path, parts[1]));
                    }
                }
                "recent" => println!("{}", recent::run(&self.config.project_path)),
                "git-status" => println!("{}", git::status(&self.config.project_path)),
                "git-log" => {
                    let n = if parts.len() > 1 { parts[1] } else { "5" };
                    println!("{}", git::log(&self.config.project_path, n));
                }
                "git-branch" => println!("{}", git::branch(&self.config.project_path)),
                "exit" => {
                    println!("{}", "Goodbye!".bright_red());
                    break;
                }
                _ => {
                    println!("'{}' is not a recognized command. Type 'help'.", parts[0].red());
                }
            }
            println!(); // Extra newline for readability
        }
    }
}