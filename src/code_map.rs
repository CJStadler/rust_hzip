use byte_node::ByteNode;
use std::collections::HashMap;
use bit_vec::BitVec;
use code_tree::CodeTree;

#[derive(Debug)]
pub struct CodeMap {
    map: HashMap<u8, BitVec>
}

impl CodeMap {
    pub fn new() -> CodeMap {
        CodeMap { map: HashMap::new() }
    }

    pub fn get(&self, k: &u8) -> Option<&BitVec> {
        self.map.get(k)
    }

    pub fn insert(&mut self, k: u8, v: BitVec) -> Option<BitVec> {
        self.map.insert(k, v)
    }

    pub fn from_tree(tree: CodeTree) -> CodeMap {
        let mut map = CodeMap::new();

        if tree.root.is_leaf() {
            map.insert(tree.root.byte.unwrap(), BitVec::from_elem(1, true));
        } else {
            map.add_nodes_recursively(*tree.root.left.unwrap(), BitVec::from_elem(1, false));
            map.add_nodes_recursively(*tree.root.right.unwrap(), BitVec::from_elem(1, true));
        }

        map
    }

    fn add_nodes_recursively(&mut self, root: ByteNode, path: BitVec) {
        if root.is_leaf() {
            self.insert(root.byte.unwrap(), path);
        } else {
            let mut left_path = path;
            let mut right_path = left_path.clone();
            left_path.push(false);
            right_path.push(true);
            self.add_nodes_recursively(*root.left.unwrap(), left_path);
            self.add_nodes_recursively(*root.right.unwrap(), right_path);
        }
    }

    // fn nodes_from_frequency_map(frequencies: &ByteFrequencyMap) -> Vec<ByteNode> {
    //     let nodes = Vec::new();
    //
    //     for (byte, frequency) in frequencies.iter() {
    //         let node = ByteNode {
    //             byte: byte,
    //             frequency: frequency
    //         };
    //
    //         nodes.push(node)
    //     }
    //
    //     nodes
    // }
}
