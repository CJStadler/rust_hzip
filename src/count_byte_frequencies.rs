use std::collections::HashMap;

pub fn count_byte_frequencies(bytes: Vec<u8>) -> HashMap<u8, u32> {
    let mut frequencies = HashMap::new();

    for byte in bytes {
        *frequencies.entry(byte).or_insert(0) += 1;
    }

    frequencies
}
