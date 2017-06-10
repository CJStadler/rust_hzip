use byte_node::ByteNode;
use byte_frequency_map::ByteFrequencyMap;
use std::collections::HashMap;
use bit_vec::BitVec;

#[derive(Debug)]
pub struct CodeMap {
    map: HashMap<u8, BitVec>
}

impl CodeMap {
    pub fn from_frequency_map(frequencies: &ByteFrequencyMap) -> CodeMap {
        let nodes = CodeMap::nodes_from_frequency_map(frequencies);
        nodes.sort_by(|a, b| a.frequency.cmp(&b.frequency));

        let mut code_map = HashMap::new();

        CodeMap { map: code_map }
    }

    fn nodes_from_frequency_map(frequencies: &ByteFrequencyMap) -> Vec<ByteNode> {
        let nodes = Vec::new();

        for (byte, frequency) in frequencies.iter() {
            let node = ByteNode {
                byte: byte,
                frequency: frequency
            };

            nodes.push(node)
        }

        nodes
    }
}
