use crate::data_structures::Content;
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

pub fn expert(path: &Path) -> Result<ExpertResult, std::io::Error> {
    let metadata = path.metadata()?;
    if metadata.is_dir() {
        return Ok(ExpertResult::Kind("Directory".to_string()));
    }

    if let Some(lang) = guess_by_filename(path) {
        return Ok(ExpertResult::Kind(lang.to_string()));
    }
    let content = Content::new(path)?;
    match content {
        Content::Binary(_) => return Ok(ExpertResult::Kind("Binary".to_string())),
        Content::Empty => return Ok(ExpertResult::Kind("Unknown file".to_string())),
        Content::Text { modelines, body } => {
            if let Some(interpreter) = guess_by_interpreter(&body) {
                return Ok(ExpertResult::Kind(interpreter.to_string()));
            }
            if let Some(lang) = guess_by_modeline(&modelines) {
                return Ok(ExpertResult::Kind(lang.to_string()));
            }

            if let Some(ext_vec) = extensions(&path) {
                for ext in ext_vec {
                    if let Some(lang) = guess_by_linguist_heuristic(&ext, &body) {
                        return Ok(ExpertResult::Kind(lang.to_string()));
                    }

                    if let Some(lang) = guess_by_extensions(&ext) {
                        return Ok(ExpertResult::Kind(lang.to_string()));
                    }
                }
            }
        }
    }
    Ok(ExpertResult::Unknown)
}

fn extensions(path: &Path) -> Option<Vec<String>> {
    if let Some(os_str) = path.file_name() {
        if let Some(filename) = os_str.to_str() {
            let mut result = Vec::with_capacity(2);
            let mut ext1 = String::new();
            for c in filename.chars().rev() {
                ext1.insert(0, c);
                if c == '.' {
                    break;
                }
            }
            if ext1.len() == filename.len() {
                return None;
            }

            let mut ext2 = ext1.to_owned();
            let new_end = filename.len() - ext1.len();
            for c in filename[0..new_end].chars().rev() {
                ext2.insert(0, c);
                if c == '.' {
                    break;
                }
            }

            if !ext2.is_empty() && ext2.len() != filename.len() {
                ext2 = ext2.to_lowercase();
                result.push(ext2);
            }
            ext1 = ext1.to_lowercase();
            result.push(ext1);
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
#[test]
fn test_extensions() {
    let path = Path::new("foo/bar.js");
    let expected = ".js";
    let actual = &extensions(&path).unwrap()[0];
    assert_eq!(expected, actual);

    let path = Path::new("foo/bar.Js");
    let expected = ".js";
    let actual = &extensions(&path).unwrap()[0];
    assert_eq!(expected, actual);

    let path = Path::new(".gitignore");
    let expected: Option<Vec<String>> = None;
    let actual = extensions(&path);
    assert_eq!(expected, actual);

    let path = Path::new(".gitignore.js");
    let expected = ".js";
    let actual = &extensions(&path).unwrap()[0];
    assert_eq!(expected, actual);

    let path = Path::new("libfoo.dll.config");
    let expected = vec![".dll.config".to_string(), ".config".to_string()];
    let actual = &extensions(&path).unwrap();
    assert_eq!(expected[0], actual[0]);
    assert_eq!(expected[1], actual[1]);
}
