use crate::utils::printer;

pub fn run() {
    printer::print_header("Available Commands");
    printer::print_info("help         — show this message");
    printer::print_info("about        — about this project");
    printer::print_info("ls           — list project files");
    printer::print_info("ls <folder>  — list files in a folder");
    printer::print_info("exit         — quit the CLI");
    printer::print_info("cat <file>   — read a file's contents");
    printer::print_info("find <path> <name> — search for a file recursively");
    println!();
}