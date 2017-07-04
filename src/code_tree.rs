pub struct CodeTree {
    pub root: Option<ByteNode>
}

impl CodeTree {
    pub fn from_bytes(bytes: &Vec<u8>) -> CodeTree {
        let frequency_map = ByteFrequencyMap::from_vec(bytes);
        let tree = CodeTree { root: None };

        for (byte, frequency) in frequency_map.iter() {
            let node = ByteNode { byte: byte, frequency: frequency};
            tree.insert_node(node);
        }
    }

    fn insert_node(node: ByteNode) {
        print_ln!("test")
    }
}
