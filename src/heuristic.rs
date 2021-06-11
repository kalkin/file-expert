use crate::linguist_aliases::ALIASES;
use crate::linguist_extensions::EXTENSIONS;
use crate::linguist_filenames::FILENAMES;
use crate::linguist_heuristics::linguist_heuristic;
use crate::linguist_interpreters::INTERPRETERS;
use crate::modeline;
use crate::shebang;
use std::path::Path;

pub fn guess_by_filename(path: &Path) -> Option<&'static String> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    return FILENAMES.get(filename);
}

pub fn guess_by_interpreter(first_line: &str) -> Option<&'static String> {
    if let Some(interpreter) = shebang::interpreter(&first_line) {
        return INTERPRETERS.get(&interpreter);
    }
    None
}

pub fn guess_by_modeline(modelines: &Vec<String>) -> Option<&'static String> {
    for line in modelines {
        if let Some(alias) = modeline::parse_modeline(&line) {
            if ALIASES.contains_key(alias) {
                return ALIASES.get(alias);
            }
        }
    }
    None
}

pub fn guess_by_linguist_heuristic(ext: &str, body: &Vec<String>) -> Option<&'static str> {
    linguist_heuristic(&ext, body)
}

pub fn guess_by_extensions(ext: &str) -> Option<&'static String> {
    if EXTENSIONS.contains_key(ext) {
        return Some(&EXTENSIONS[ext]);
    }
    None
}
