use std::fs::File;
use std::io::prelude::*;

pub fn write_file(filename: &str, contents: String) {
    let mut file = File::create(filename).unwrap();
    file.write_all(contents.as_bytes()).unwrap()
}
