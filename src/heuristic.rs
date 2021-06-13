use crate::linguist_aliases::ALIASES;
use crate::linguist_extensions::EXTENSIONS;
use crate::linguist_filenames::FILENAMES;
use crate::linguist_heuristics::linguist_heuristic;
use crate::linguist_interpreters::INTERPRETERS;
use crate::modeline;
use crate::shebang;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::path::Path;

lazy_static! {
    static ref SKIP_REGEX: Regex = Regex::new(r#"^(?:#.*|\s*)"#).unwrap();
    static ref EXEC_REGEX: Regex = Regex::new(r#"^\s*exec\s+(\w+)\s+.*$"#).unwrap();
}

pub fn guess_by_filename(path: &Path) -> Option<&'static String> {
    let filename = path.file_name().unwrap().to_str().unwrap();
    return FILENAMES.get(filename);
}

pub fn guess_by_interpreter(body: &Vec<String>) -> Option<&'static String> {
    if let Some(interpreter) = shebang::interpreter(&body[0]) {
        if let Some(language) = INTERPRETERS.get(&interpreter) {
            if language == &"Shell" {
                for line in &body[1..] {
                    if let Ok(captures) = EXEC_REGEX.captures(line) {
                        if let Some(caps) = captures {
                            let interpreter = caps.get(1).unwrap().as_str();
                            if INTERPRETERS.contains_key(interpreter) {
                                return INTERPRETERS.get(interpreter);
                            }
                        } else {
                            break;
                        }
                    } else if let Err(_) = SKIP_REGEX.is_match(line) {
                        break;
                    }
                }
            }

            return Some(language);
        }
    }
    None
}

pub fn guess_by_modeline(modelines: &Vec<String>) -> Option<&'static String> {
    for line in modelines {
        if let Some(alias) = modeline::parse(&line) {
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
