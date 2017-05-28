use bit_vec::BitVec;
use std::fs::File;
use std::io::prelude::*;

pub fn write_string_to_file(filename: &str, contents: String) {
    write_bytes_to_file(filename, contents.as_bytes().to_vec())
}

pub fn write_bytes_to_file(filename: &str, contents: Vec<u8>) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&contents).unwrap()
}

pub fn write_bits_to_file(filename: &str, contents: BitVec) {
    write_bytes_to_file(filename, contents.to_bytes())
}
