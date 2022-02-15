//
// Copyright (c) 2018-2020 Bahtiar `kalkin-` Gadimov.
//
// This file is part of file-expert
// (see https://github.com/kalkin/file-expert).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
use crate::data_structures::Content;
use crate::heuristic::{
    guess_by_extensions, guess_by_filename, guess_by_heuristic, guess_by_interpreter,
    guess_by_modeline,
};
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub enum Guess {
    Kind(String),
    Unknown,
}

impl Display for Guess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Guess::Kind(lang) => f.write_str(lang),
            Guess::Unknown => f.write_str("Unknown file"),
        }
    }
}

///
/// # Errors
/// Will return [`std::io::Error`] if there're issues with reading the file
pub fn guess(path: &Path) -> Result<Guess, std::io::Error> {
    let metadata = path.metadata()?;
    if metadata.is_dir() {
        return Ok(Guess::Kind("Directory".to_string()));
    }

    if let Some(lang) = guess_by_filename(path) {
        return Ok(Guess::Kind(lang.to_string()));
    }
    let optional_extensions = extensions(path);
    let content = Content::new(path)?;
    match content {
        Content::Binary(_) => return Ok(Guess::Kind("Binary".to_string())),
        Content::Empty => {
            if let Some(ext_vec) = optional_extensions {
                for ext in ext_vec {
                    if let Some(lang) = guess_by_extensions(&ext) {
                        return Ok(Guess::Kind(lang.to_string()));
                    }
                }
            }
            return Ok(Guess::Kind("Unknown file".to_string()));
        }
        Content::Text { modelines, body } => {
            if let Some(interpreter) = guess_by_interpreter(&body) {
                return Ok(Guess::Kind(interpreter.to_string()));
            }
            if let Some(lang) = guess_by_modeline(&modelines) {
                return Ok(Guess::Kind(lang.to_string()));
            }

            if let Some(ext_vec) = optional_extensions {
                for ext in ext_vec {
                    if let Some(lang) = guess_by_heuristic(&ext, &body) {
                        return Ok(Guess::Kind(lang.to_string()));
                    }

                    if let Some(lang) = guess_by_extensions(&ext) {
                        return Ok(Guess::Kind(lang.to_string()));
                    }
                }
            }
        }
    }
    Ok(Guess::Unknown)
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

            let mut ext2 = ext1.clone();
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
#[cfg(not(tarpaulin_include))]
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
