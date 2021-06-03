use crate::heuristic;
use crate::heuristic::guess_by_interpreter;
use std::fmt::{Display, Formatter, Write};
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
    if let Some(interpreter) = guess_by_interpreter(path) {
        return ExpertResult::Kind(interpreter.to_string());
    }
    ExpertResult::Unknown
}
