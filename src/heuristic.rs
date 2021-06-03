use crate::shebang;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/linguist_interpreters.rs"));

pub fn guess_by_interpreter(path: &Path) -> Option<&String> {
    let first_line = read_first_line(path);
    if let Some(interpreter) = shebang::interpreter(&first_line) {
        return INTERPRETERS.get(&interpreter);
    }
    None
}

fn read_first_line(path: &Path) -> String {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {:?}", &path),
    };
    let mut buffer = BufReader::new(file);
    let mut first_line = String::new();
    buffer.read_line(&mut first_line).unwrap();
    first_line
}
