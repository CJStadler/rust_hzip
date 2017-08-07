extern crate bit_vec;
extern crate byteorder;
extern crate getopts;

mod byte_node;
mod byte_frequency_map;
mod code_map;
mod code_tree;
mod decoder;
mod encoder;
mod read_file;
mod write_file;

use decoder::decode;
use encoder::encode;
use read_file::read_file_to_bytes;
use write_file::write_bytes_to_file;
use getopts::Options;
use std::env;

fn read_args() -> (bool, String, String) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("d", "decode", "decode input to output (oterwise encodes)");
    opts.reqopt("i", "infile", "set input file name", "FILENAME");
    opts.reqopt("o", "outfile", "set output file name", "FILENAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let decode = matches.opt_present("d");
    let in_filename = matches.opt_str("i").unwrap();
    let out_filename = matches.opt_str("o").unwrap();
    (decode, in_filename, out_filename)
}

fn main() {
    let (decode_flag, in_filename, out_filename) = read_args();
    let bytes = read_file_to_bytes(&in_filename);

    let out_bytes;
    if decode_flag {
        out_bytes = decode(&bytes);
    } else {
        out_bytes = encode(&bytes);
    };

    write_bytes_to_file(&out_filename, &out_bytes);
}
