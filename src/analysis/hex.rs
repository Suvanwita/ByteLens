use colored::*;

pub fn hex_view(bytes: &[u8], width: usize) {
    for (i, chunk) in bytes.chunks(width).enumerate() {

        // Offset
        print!(
            "{}  ",
            format!("{:08x}", i * width)
                .bright_blue()
                .bold()
        );

        // Hex bytes
        for byte in chunk {

            let formatted = format!("{:02x}", byte);

            // Apply colors
            if *byte == 0x00 {
                // Null bytes
                print!("{} ", formatted.dimmed());

            } else if byte.is_ascii_graphic() || *byte == b' ' {
                // Printable ASCII
                print!("{} ", formatted.bright_green());

            } else if i == 0 && (*byte == 0x7f || *byte == 0x45 || *byte == 0x4c || *byte == 0x46) {
                // ELF magic bytes
                print!("{} ", formatted.bright_cyan().bold());

            } else {
                // Other bytes
                print!("{} ", formatted.yellow());
            }
        }

        // Padding
        for _ in 0..(width - chunk.len()) {
            print!("   ");
        }

        print!(" ");

        // ASCII view
        for byte in chunk {
            let c = *byte as char;

            if c.is_ascii_graphic() || c == ' ' {
                print!("{}", c.to_string().bright_green());
            } else {
                print!("{}", ".".dimmed());
            }
        }

        println!();
    }
}