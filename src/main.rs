mod parser;
mod analysis;
mod utils;

use clap::Parser;
use parser::binary::analyze_binary;
use analysis::strings::extract_strings;
use analysis::entropy::calculate_entropy;
use analysis::hex::hex_view;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to binary file
    file: String,

    /// Show sections
    #[arg(long)]
    sections: bool,

    /// Show imports
    #[arg(long)]
    imports: bool,

    /// Extract strings
    #[arg(long)]
    strings: bool,

    /// Calculate entropy
    #[arg(long)]
    entropy: bool,

    /// Show hex view
    #[arg(long)]
    hex: bool,
}

fn main() {
    let args = Args::parse();

    let bytes = std::fs::read(&args.file)
        .expect("Failed to read file");

    analyze_binary(&bytes, &args);

    if args.strings {
        println!("\n=== STRINGS ===");
        let strings = extract_strings(&bytes);

        for s in strings {
            println!("{s}");
        }
    }

    if args.entropy {
        println!("\n=== ENTROPY ===");

        let entropy = calculate_entropy(&bytes);

        println!("Entropy: {:.4}", entropy);
    }

    if args.hex {
        println!("\n=== HEX VIEW ===");

        hex_view(&bytes, 16);
    }
}