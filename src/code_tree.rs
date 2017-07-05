use byte_frequency_map::ByteFrequencyMap;
use byte_node::ByteNode;

#[derive(Debug)]
pub struct CodeTree {
    pub root: ByteNode
}

impl CodeTree {
    pub fn from_bytes(bytes: &Vec<u8>) -> CodeTree {
        let frequency_map = ByteFrequencyMap::from_vec(bytes);

        let nodes: Vec<ByteNode> = frequency_map.iter()
            .map(|(byte, frequency)|
                ByteNode {
                    byte: Some(byte.clone()),
                    frequency: frequency.clone(),
                    left: None,
                    right: None
                }
            )
            .collect();

        let root = huffman(nodes);

        CodeTree { root: root }
    }

}


fn huffman(mut nodes: Vec<ByteNode>) -> ByteNode {
    nodes.sort();

    if nodes.len() == 1 {
        nodes.pop().unwrap()
    } else {
        let first = nodes.pop().unwrap();
        let second = nodes.pop().unwrap();
        let new_node = ByteNode {
            byte: None,
            frequency: first.frequency + second.frequency,
            left: Some(Box::new(first)),
            right: Some(Box::new(second))
        };

        nodes.push(new_node);

        huffman(nodes)
    }
}
