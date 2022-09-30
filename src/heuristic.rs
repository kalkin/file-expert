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
use crate::linguist_aliases::MODELINE_ALIASES;
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
    path.file_name()
        .expect("Not ..")
        .to_str()
        .and_then(|f| FILENAMES.get(f))
}

pub fn guess_by_interpreter(body: &[String]) -> Option<&'static String> {
    if let Some(interpreter) = shebang::interpreter(&body[0]) {
        if let Some(language) = INTERPRETERS.get(&interpreter) {
            if language == "Shell" {
                for line in &body[1..] {
                    #[allow(clippy::else_if_without_else)]
                    if let Ok(captures) = EXEC_REGEX.captures(line) {
                        if let Some(caps) = captures {
                            let exec_interpreter = caps.get(1).unwrap().as_str();
                            if INTERPRETERS.contains_key(exec_interpreter) {
                                return INTERPRETERS.get(exec_interpreter);
                            }
                        } else {
                            break;
                        }
                    } else if SKIP_REGEX.is_match(line).is_err() {
                        break;
                    }
                }
            }

            return Some(language);
        }
    }
    None
}

pub fn guess_by_modeline(modelines: &[String]) -> Option<&'static String> {
    for line in modelines {
        if let Some(alias) = modeline::parse(line) {
            if MODELINE_ALIASES.contains_key(alias) {
                if let Some(result) = MODELINE_ALIASES.get(alias) {
                    // WORKAROUND: Vimball files modeline will not reflect their filetype.
                    if result == "Vim Help File" && modelines.iter().any(|x| x == "UseVimball") {
                        return MODELINE_ALIASES.get("vim script");
                    }
                    return Some(result);
                }
            }
        }
    }
    None
}

#[allow(clippy::module_name_repetitions)]
pub fn guess_by_heuristic(ext: &str, body: &[String]) -> Option<&'static str> {
    linguist_heuristic(ext, body)
}

pub fn guess_by_extensions(ext: &str) -> Option<&'static String> {
    if EXTENSIONS.contains_key(ext) {
        return Some(&EXTENSIONS[ext]);
    }
    None
}
