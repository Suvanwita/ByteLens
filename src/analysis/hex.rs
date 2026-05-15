pub fn hex_view(bytes: &[u8], width: usize) {
    for (i, chunk) in bytes.chunks(width).enumerate() {
        // Offset
        print!("{:08x}  ", i * width);

        // Hex bytes
        for byte in chunk {
            print!("{:02x} ", byte);
        }

        // Padding for incomplete rows
        for _ in 0..(width - chunk.len()) {
            print!("   ");
        }

        print!(" ");

        // ASCII representation
        for byte in chunk {
            let c = *byte as char;

            if c.is_ascii_graphic() || c == ' ' {
                print!("{c}");
            } else {
                print!(".");
            }
        }

        println!();
    }
}