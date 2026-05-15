pub fn calculate_entropy(data: &[u8]) -> f64 {
    let mut freq = [0usize; 256];

    for &byte in data {
        freq[byte as usize] += 1;
    }

    let len = data.len() as f64;

    let mut entropy = 0.0;

    for count in freq {
        if count == 0 {
            continue;
        }

        let p = count as f64 / len;

        entropy -= p * p.log2();
    }

    entropy
}