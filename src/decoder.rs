use bit_vec::BitVec;
use byteorder::{ByteOrder, LittleEndian};
use code_map::CodeMap;
use code_tree::CodeTree;

pub fn decode(encoded_bytes: &[u8]) -> Vec<u8> {
    let bits_in_last_byte = encoded_bytes.get(0).unwrap();
    let mut code_tree_length = LittleEndian::read_u16(encoded_bytes.get(0..2).unwrap());

    Vec::new()
}
