use bit_vec::BitVec;
use byte_frequency_map::ByteFrequencyMap;
use byte_node::ByteNode;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub struct CodeTree {
    pub root: ByteNode
}

impl CodeTree {
    pub fn from_bytes(bytes: &[u8]) -> CodeTree {
        let frequency_map = ByteFrequencyMap::from_vec(bytes);

        let nodes: Vec<ByteNode> = frequency_map.iter()
            .map(|(byte, frequency)|
                ByteNode {
                    byte: Some(*byte),
                    frequency: *frequency,
                    left: None,
                    right: None
                }
            )
            .collect();

        let root = huffman(nodes);

        CodeTree { root: root }
    }

    pub fn as_bits(&self) -> BitVec {
        let tree_bits = bits_from_traversal(&self.root);
        let tree_length = tree_bits.len() as u16;
        let mut tree_with_length = u16_to_bits(tree_length);
        append_to_bitvec(&mut tree_with_length, tree_bits);
        tree_with_length
    }
}

fn append_to_bitvec(bitvec: &mut BitVec, other: BitVec) {
    for bit in other {
        bitvec.push(bit);
    }
}

fn bits_from_traversal(root: &ByteNode) -> BitVec {
    let mut bits = BitVec::new();

    if root.is_leaf() {
        bits.push(true);
        let byte_as_bits = BitVec::from_bytes(&[root.byte.unwrap()]);
        append_to_bitvec(&mut bits, byte_as_bits);
    } else {
        bits.push(false);
        let left_traversal = bits_from_traversal(&root.left());
        let right_traversal = bits_from_traversal(&root.right());
        append_to_bitvec(&mut bits, left_traversal);
        append_to_bitvec(&mut bits, right_traversal);
    }

    bits
}

fn huffman(mut nodes: Vec<ByteNode>) -> ByteNode {
    nodes.sort_by(|a, b| a.cmp(b).reverse());

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

fn u16_to_bits(n: u16) -> BitVec {
    let mut length_bytes = [0; 2];
    LittleEndian::write_u16(&mut length_bytes, n);
    BitVec::from_bytes(&length_bytes)
}
