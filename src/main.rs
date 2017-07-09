extern crate bit_vec;
extern crate getopts;

mod byte_node;
mod code_map;
mod code_tree;
mod byte_frequency_map;
mod read_file;
mod write_file;

use code_map::CodeMap;
use code_tree::CodeTree;
use read_file::read_file_to_bytes;
use write_file::write_bits_to_file;
use getopts::Options;
use std::env;

fn read_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("i", "", "set input file name", "NAME");
    opts.optopt("o", "", "set output file name", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let in_filename = matches.opt_str("i").unwrap();
    let out_filename = matches.opt_str("o").unwrap();
    (in_filename, out_filename)
}

fn main() {
    let (in_filename, out_filename) = read_args();
    let bytes = read_file_to_bytes(&in_filename);
    let code_tree = CodeTree::from_bytes(&bytes);
    let code_map = CodeMap::from_tree(code_tree);

    let encoded_bits = bytes.iter().flat_map(|byte|
        code_map.get(byte).unwrap()
    ).collect();

    write_bits_to_file(&out_filename, &encoded_bits);
    // write tree
    println!("Code map: {:?}", code_map);
}
