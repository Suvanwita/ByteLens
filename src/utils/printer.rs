use colored::*;

pub fn header(title: &str) {
    println!(
        "\n{}",
        format!("=== {} ===", title)
            .bright_green()
            .bold()
    );
}