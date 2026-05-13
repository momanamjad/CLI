use crate::utils::printer;

pub fn run() {
    printer::print_header("GitHub Clone Project");
    printer::print_info("Frontend  : React 19 + Vite + Tailwind CSS");
    printer::print_info("UI Lib    : Radix UI + Lucide + Octicons");
    printer::print_info("Auth      : Google OAuth");
    printer::print_info("State     : React Context (GitHubContext)");
    printer::print_info("CLI Tool  : Rust (you are here)");
    printer::print_info("Developer : Moman");
    println!();
}