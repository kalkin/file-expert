use crate::data_structures::{FileContent, Text};
use crate::heuristic::{
    guess_by_extensions, guess_by_filename, guess_by_interpreter, guess_by_linguist_heuristic,
    guess_by_modeline,
};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum ExpertResult {
    Kind(String),
    Unknown,
}

impl Display for ExpertResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpertResult::Kind(lang) => f.write_str(lang),
            ExpertResult::Unknown => f.write_str("Unknown file"),
        }
    }
}

pub fn expert(path: &Path) -> ExpertResult {
    if !path.exists() {
        return ExpertResult::Kind("Missing file".to_string());
    }

    if path.metadata().unwrap().is_dir() {
        return ExpertResult::Kind("Directory".to_string());
    }

    if let Some(lang) = guess_by_filename(path) {
        return ExpertResult::Kind(lang.to_string());
    }
    let data = FileContent::new(path).unwrap();
    if data.is_empty() {
        return ExpertResult::Kind("Unknown file".to_string());
    }
    if data.is_binary() {
        return ExpertResult::Kind("Binary".to_string());
    }

    let content = Text::from(data);

    if let Some(interpreter) = guess_by_interpreter(&content) {
        return ExpertResult::Kind(interpreter.to_string());
    }

    if let Some(lang) = guess_by_modeline(&content) {
        return ExpertResult::Kind(lang.to_string());
    }

    if let Some(lang) = guess_by_linguist_heuristic(&content) {
        return ExpertResult::Kind(lang.to_string());
    }

    if let Some(lang) = guess_by_extensions(&content) {
        return ExpertResult::Kind(lang.to_string());
    }

    ExpertResult::Unknown
}
