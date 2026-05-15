pub fn extract_strings(bytes: &[u8]) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for &b in bytes {
        let c = b as char;

        if c.is_ascii_graphic() || c == ' ' {
            current.push(c);
        } else {
            if current.len() >= 4 {
                result.push(current.clone());
            }

            current.clear();
        }
    }

    result
}