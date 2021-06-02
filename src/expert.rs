use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader};
use crate::shebang;

#[derive(Debug, Eq, PartialEq)]
pub enum ExpertResult {
    Kind(String),
    Unknown,
}

pub fn expert(path: &Path) -> ExpertResult {
    // let first_line = first_line(path);
    // if let Some(interpreter) = shebang::interpreter(&first_line) {
    //
    // }
    ExpertResult::Unknown
}

fn first_line(path: &Path) -> String {
    let file = match fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {:?}", &path),
    };
    let mut buffer = BufReader::new(file);
    let mut first_line = String::new();
    buffer.read_line(&mut first_line).unwrap();
    first_line
}