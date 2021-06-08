use crate::data_structures::Text;
use crate::linguist_heuristics::linguist_heuristic;
use crate::modeline;
use crate::shebang;
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/linguist_interpreters.rs"));
include!(concat!(env!("OUT_DIR"), "/linguist_aliases.rs"));
include!(concat!(env!("OUT_DIR"), "/linguist_filenames.rs"));
include!(concat!(env!("OUT_DIR"), "/linguist_extensions.rs"));

pub fn guess_by_filename(path: &Path) -> Option<&'static String> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    return FILENAMES.get(filename);
}

pub fn guess_by_interpreter(content: &Text) -> Option<&'static String> {
    if let Some(interpreter) = shebang::interpreter(&content.first_line) {
        return INTERPRETERS.get(&interpreter);
    }
    None
}

pub fn guess_by_modeline(content: &Text) -> Option<&'static String> {
    if let Some(alias) = modeline::parse_modeline(&content.first_line) {
        return ALIASES.get(alias);
    }
    None
}

pub fn guess_by_linguist_heuristic(ext: &str, content: &Text) -> Option<&'static str> {
    linguist_heuristic(&ext, &content.body)
}

pub fn guess_by_extensions(ext: &str) -> Option<&'static String> {
    if EXTENSIONS.contains_key(ext) {
        return Some(&EXTENSIONS[ext]);
    }
    None
}
