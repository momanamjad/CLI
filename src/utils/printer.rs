pub fn print_success(msg: &str) {
    println!("✓ {}", msg);
}

pub fn print_error(msg: &str) {
    println!("✗ Error: {}", msg);
}

pub fn print_info(msg: &str) {
    println!("  {}", msg);
}

pub fn print_header(msg: &str) {
    println!("\n{}", msg);
    println!("{}", "─".repeat(msg.len()));
}