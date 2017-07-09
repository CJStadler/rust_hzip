// use bit_vec::BitVec;
use std::fs::File;
use std::io::prelude::*;

// pub fn read_file_to_string(filename: &str) -> String {
//     let mut file = File::open(filename).unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     contents
// }

pub fn read_file_to_bytes(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

// pub fn read_file_to_bits(filename: &str) -> BitVec {
//     BitVec::from_bytes(&read_file_to_bytes(filename))
// }
