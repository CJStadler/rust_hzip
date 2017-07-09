use std::collections::HashMap;
use std::collections::hash_map::Iter;

#[derive(Debug)]
pub struct ByteFrequencyMap(HashMap<u8, u32>);

impl ByteFrequencyMap {
    pub fn from_vec(bytes: &[u8]) -> ByteFrequencyMap {
        let mut frequencies = HashMap::new();

        for &byte in bytes {
            *frequencies.entry(byte).or_insert(0) += 1;
        }

        ByteFrequencyMap(frequencies)
    }

    pub fn iter(&self) -> Iter<u8, u32> {
        self.0.iter()
    }
}
