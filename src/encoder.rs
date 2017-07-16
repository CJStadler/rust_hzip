use bit_vec::BitVec;
use code_map::CodeMap;
use code_tree::CodeTree;

/// Returned Vec has the following format:
/// 1 byte recording the number of bits used in the last byte.
/// 2 bytes recording the number of bits (should this be bytes?) used by the code tree.
/// Preorder traversal of code tree.
/// Encoded message.
pub fn encode(raw_bytes: &[u8]) -> Vec<u8> {
    let code_tree = CodeTree::from_bytes(raw_bytes);
    let mut encoded_tree_and_message = code_tree.as_bits();
    let encoded_message = encode_message(raw_bytes, code_tree);
    append_to_bitvec(&mut encoded_tree_and_message, encoded_message);

    let bits_in_last_byte = (encoded_tree_and_message.len() % 8) as u8;
    let mut encoded_bytes = vec![bits_in_last_byte];
    encoded_bytes.append(&mut encoded_tree_and_message.to_bytes());
    encoded_bytes
}

fn encode_message(raw_bytes: &[u8], code_tree: CodeTree) -> BitVec {
    let code_map = CodeMap::from_tree(code_tree);
    println!("Code map: {:?}", code_map);

    raw_bytes.iter().flat_map(|byte|
        code_map.get(byte).unwrap()
    ).collect()
}

fn append_to_bitvec(bitvec: &mut BitVec, other: BitVec) {
    for bit in other {
        bitvec.push(bit);
    }
}
