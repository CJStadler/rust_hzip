extern crate bit_vec;
extern crate getopts;

mod read_file;
mod write_file;
use read_file::read_file_to_bits;
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
    let contents = read_file_to_bits(&in_filename);
    println!("Bits: {:?}", contents);
    write_bits_to_file(&out_filename, contents);
}
